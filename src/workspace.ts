import { execFile, spawn, type ChildProcess } from 'node:child_process';
import { access, lstat, mkdir, mkdtemp, readFile, realpath, rm } from 'node:fs/promises';
import { createServer } from 'node:net';
import path from 'node:path';
import { promisify } from 'node:util';
import type { WitnessConfig } from './types.js';

const execFileAsync = promisify(execFile);
const INSTALL_TIMEOUT_MS = 300_000;
const BUILD_TIMEOUT_MS = 180_000;
const delay = (ms: number) => new Promise<void>(resolve => setTimeout(resolve, ms));

async function git(repo: string, args: string[], label: string): Promise<string> {
  try {
    const result = await execFileAsync('git', ['-C', repo, ...args], {
      encoding: 'utf8', timeout: 30_000, maxBuffer: 4 * 1024 * 1024, windowsHide: true,
    });
    return result.stdout;
  } catch {
    // Git/process output can contain private URLs, paths, or credential helpers.
    throw new Error(label);
  }
}

function parseDirtyFiles(status: string): string[] {
  const records = status.split('\0');
  const files = new Set<string>();
  for (let index = 0; index < records.length; index++) {
    const record = records[index];
    if (!record) continue;
    files.add(record.slice(3));
    if (/[RC]/.test(record.slice(0, 2)) && records[index + 1]) files.add(records[++index]);
  }
  return [...files].sort();
}

async function resolveCommit(repo: string, ref: string, side: string): Promise<string> {
  if (!ref || ref.startsWith('-') || ref.includes('\0')) throw new Error(`Invalid ${side} revision`);
  const sha = (await git(repo, ['rev-parse', '--verify', '--end-of-options', `${ref}^{commit}`], `Invalid ${side} revision`)).trim();
  if (!/^[a-f\d]{40,64}$/i.test(sha)) throw new Error(`Invalid ${side} revision`);
  return sha;
}

/** Creates detached, disposable revision checkouts; never changes the caller's index or files. */
export async function createRevisionWorkspaces(
  repo: string, baseRef: string, headRef: string, runDir: string,
): Promise<{ baseDir: string; headDir: string; baseSha: string; headSha: string; dirtyFiles: string[]; cleanup(): Promise<void> }> {
  const repoDir = await realpath(repo);
  await git(repoDir, ['rev-parse', '--show-toplevel'], 'A working Git repository is required');
  const baseSha = await resolveCommit(repoDir, baseRef, 'base');
  const headSha = await resolveCommit(repoDir, headRef, 'head');
  const dirtyFiles = parseDirtyFiles(await git(repoDir, ['status', '--porcelain=v1', '-z', '--untracked-files=all'], 'Could not inspect source checkout'));
  await mkdir(path.resolve(runDir), { recursive: true });
  const runRoot = await realpath(runDir);
  const ownedRoot = await mkdtemp(path.join(runRoot, 'worktrees-'));
  const baseDir = path.join(ownedRoot, 'base');
  const headDir = path.join(ownedRoot, 'head');
  const registered: string[] = [];
  let cleaned = false;

  async function assertOwnedRoot(): Promise<void> {
    const relative = path.relative(runRoot, ownedRoot);
    if (!relative || relative.startsWith(`..${path.sep}`) || relative === '..' || path.isAbsolute(relative)) {
      throw new Error('Refusing cleanup outside the owned run directory');
    }
    const stat = await lstat(ownedRoot);
    if (!stat.isDirectory() || stat.isSymbolicLink() || await realpath(ownedRoot) !== ownedRoot) {
      throw new Error('Refusing cleanup of a replaced workspace directory');
    }
  }

  async function cleanup(): Promise<void> {
    if (cleaned) return;
    await assertOwnedRoot();
    for (const directory of [...registered].reverse()) {
      if (path.dirname(directory) !== ownedRoot) throw new Error('Refusing cleanup outside owned worktrees');
      await git(repoDir, ['worktree', 'remove', '--force', '--', directory], 'Could not remove owned Git worktree');
      registered.splice(registered.indexOf(directory), 1);
    }
    await assertOwnedRoot();
    await rm(ownedRoot, { recursive: true, force: true });
    cleaned = true;
  }

  try {
    for (const [directory, sha] of [[baseDir, baseSha], [headDir, headSha]]) {
      await git(repoDir, ['worktree', 'add', '--detach', '--', directory, sha], 'Could not create detached Git worktree');
      registered.push(directory);
    }
  } catch (error) {
    try { await cleanup(); } catch { throw new Error('Workspace creation failed; owned workspace cleanup also failed'); }
    throw error;
  }
  return { baseDir, headDir, baseSha, headSha, dirtyFiles, cleanup };
}

async function npmCommand(): Promise<{ executable: string; prefix: string[] }> {
  if (process.platform !== 'win32') return { executable: 'npm', prefix: [] };
  const candidates = [
    process.env.npm_execpath,
    path.join(path.dirname(process.execPath), 'node_modules', 'npm', 'bin', 'npm-cli.js'),
    ...(process.env.PATH ?? '').split(path.delimiter).flatMap(directory => [
      path.join(directory.replace(/^"|"$/g, ''), 'node_modules', 'npm', 'bin', 'npm-cli.js'),
    ]),
  ];
  for (const candidate of candidates) {
    if (!candidate || path.basename(candidate).toLowerCase() !== 'npm-cli.js') continue;
    try {
      await access(candidate);
      return { executable: process.execPath, prefix: [candidate] };
    } catch { /* Try the next supported npm installation. */ }
  }
  throw new Error('Could not locate npm-cli.js; install npm alongside Node.js');
}

type ManagedProcess = { child: ChildProcess; finished: Promise<void>; stop(): Promise<void>; exited(): boolean };

function launch(executable: string, args: string[], cwd: string, label: string): ManagedProcess {
  const child = spawn(executable, args, {
    cwd, shell: false, windowsHide: true, detached: process.platform !== 'win32',
    // Do not retain raw subprocess logs or environment values in reportable errors.
    stdio: 'ignore',
    env: { ...process.env, CI: '1', BROWSER: 'none', npm_config_update_notifier: 'false' },
  });
  let exited = false;
  let stopping: Promise<void> | undefined;
  const finished = new Promise<void>((resolve, reject) => {
    child.once('error', () => { exited = true; reject(new Error(`${label} could not start`)); });
    child.once('exit', (code, signal) => {
      exited = true;
      if (code === 0) resolve();
      else reject(new Error(`${label} exited ${signal ? `with signal ${signal}` : `with code ${code ?? 'unknown'}`}`));
    });
  });
  // A server can exit before readiness is awaited; attach a handler immediately.
  void finished.catch(() => {});

  async function stop(): Promise<void> {
    if (stopping) return stopping;
    stopping = (async () => {
      const pid = child.pid;
      if (!pid) return;
      if (process.platform === 'win32') {
        // A numeric PID is the only target; never match by process name.
        if (!exited) {
          try { await execFileAsync('taskkill', ['/PID', String(pid), '/T', '/F'], { windowsHide: true, timeout: 10_000 }); }
          catch { if (!exited) throw new Error('Could not stop the started project process tree'); }
        }
      } else {
        try { process.kill(-pid, 'SIGTERM'); }
        catch (error) { if ((error as NodeJS.ErrnoException).code !== 'ESRCH') throw error; }
        await Promise.race([finished.catch(() => {}), delay(1500)]);
        try { process.kill(-pid, 'SIGKILL'); }
        catch (error) { if ((error as NodeJS.ErrnoException).code !== 'ESRCH') throw error; }
      }
      await Promise.race([finished.catch(() => {}), delay(2000)]);
    })();
    return stopping;
  }
  return { child, finished, stop, exited: () => exited };
}

async function runBounded(executable: string, args: string[], cwd: string, label: string, timeoutMs: number): Promise<void> {
  const process = launch(executable, args, cwd, label);
  let timer: NodeJS.Timeout | undefined;
  try {
    await Promise.race([
      process.finished,
      new Promise<never>((_, reject) => { timer = setTimeout(() => reject(new Error(`${label} timed out`)), timeoutMs); }),
    ]);
  } catch (error) {
    await process.stop();
    throw error;
  } finally {
    if (timer) clearTimeout(timer);
  }
}

async function availablePort(requestedPort = 0): Promise<number> {
  const server = createServer();
  return new Promise<number>((resolve, reject) => {
    server.once('error', () => reject(new Error('The requested local port is unavailable')));
    server.listen(requestedPort, '127.0.0.1', () => {
      const address = server.address();
      if (!address || typeof address === 'string') { server.close(); reject(new Error('Could not reserve a local port')); return; }
      server.close(error => error ? reject(error) : resolve(address.port));
    });
  });
}

/** Executes trusted project scripts. Caller must establish trust before calling this function. */
export async function prepareAndStart(
  projectDir: string, config: WitnessConfig, options: { install?: boolean; port?: number } = {},
): Promise<{ url: string; stop(): Promise<void> }> {
  if (options.port !== undefined && (!Number.isInteger(options.port) || options.port < 1024 || options.port > 65535)) {
    throw new Error('The requested port must be an integer between 1024 and 65535');
  }
  const directory = await realpath(projectDir);
  for (const filename of ['package.json', 'package-lock.json']) {
    try {
      const stat = await lstat(path.join(directory, filename));
      if (!stat.isFile() || stat.isSymbolicLink()) throw new Error('not a regular file');
    } catch { throw new Error(`A regular ${filename} is required in each revision`); }
  }
  let manifest: { scripts?: Record<string, unknown> };
  try {
    manifest = JSON.parse(await readFile(path.join(directory, 'package.json'), 'utf8'));
    const lock = JSON.parse(await readFile(path.join(directory, 'package-lock.json'), 'utf8'));
    if (!manifest || typeof manifest !== 'object' || !lock || typeof lock !== 'object') throw new Error('invalid manifests');
  } catch { throw new Error('Package manifest and lockfile must contain valid JSON objects'); }
  for (const script of [config.project.startScript, config.project.buildScript].filter((value): value is string => value !== undefined)) {
    if (!/^[\w:.-]+$/.test(script) || typeof manifest.scripts?.[script] !== 'string') {
      throw new Error('The configured npm script is missing or invalid');
    }
  }
  const npm = await npmCommand();
  if (options.install !== false) await runBounded(npm.executable, [...npm.prefix, 'ci', '--no-audit', '--no-fund'], directory, 'npm ci', INSTALL_TIMEOUT_MS);
  if (config.project.buildScript) {
    await runBounded(npm.executable, [...npm.prefix, 'run', config.project.buildScript], directory, 'Project build', BUILD_TIMEOUT_MS);
  }
  const port = await availablePort(options.port);
  const url = `http://127.0.0.1:${port}`;
  const server = launch(npm.executable, [
    ...npm.prefix, 'run', config.project.startScript, '--', '--host', '127.0.0.1', '--port', String(port), '--strictPort',
  ], directory, 'Project server');
  const deadline = Date.now() + config.timeoutMs;
  try {
    while (Date.now() < deadline) {
      if (server.exited()) {
        await server.finished;
        throw new Error('Project server stopped before becoming ready');
      }
      try {
        const response = await fetch(url, { signal: AbortSignal.timeout(Math.max(1, Math.min(500, deadline - Date.now()))), redirect: 'error' });
        await response.body?.cancel();
        if (response.status < 500 && !server.exited()) return { url, stop: server.stop };
      } catch { /* Startup may be in progress; retry within the configured deadline. */ }
      await delay(Math.min(100, Math.max(0, deadline - Date.now())));
    }
    throw new Error('Project server readiness timed out');
  } catch (error) {
    await server.stop();
    throw error;
  }
}

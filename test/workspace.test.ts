import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { access, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { promisify } from 'node:util';
import { createRevisionWorkspaces, prepareAndStart } from '../src/workspace.js';
import type { WitnessConfig } from '../src/types.js';

const exec = promisify(execFile);
const config: WitnessConfig = { schemaVersion: 1, project: { startScript: 'dev' }, scenarios: [], intendedChanges: [], timeoutMs: 5000 };

async function git(repo: string, ...args: string[]): Promise<string> {
  const result = await exec('git', ['-C', repo, ...args], { encoding: 'utf8', windowsHide: true });
  return result.stdout.trim();
}

async function tempFixture(t: { after(fn: () => Promise<void>): void }): Promise<string> {
  const directory = await mkdtemp(path.join(tmpdir(), 'uw workspace test '));
  t.after(() => rm(directory, { recursive: true, force: true }));
  return directory;
}

async function packageFixture(directory: string, script = 'node server.cjs'): Promise<void> {
  await writeFile(path.join(directory, 'package.json'), JSON.stringify({ name: 'workspace-fixture', version: '1.0.0', scripts: { dev: script } }));
  await writeFile(path.join(directory, 'package-lock.json'), JSON.stringify({ name: 'workspace-fixture', version: '1.0.0', lockfileVersion: 3, packages: { '': { name: 'workspace-fixture', version: '1.0.0' } } }));
}

test('detached revisions preserve dirty files and source branch, including paths with spaces', async t => {
  const repo = await tempFixture(t);
  const runs = await tempFixture(t);
  await git(repo, 'init');
  await git(repo, 'config', 'user.name', 'Workspace Test');
  await git(repo, 'config', 'user.email', 'workspace-test@example.invalid');
  await writeFile(path.join(repo, 'app.txt'), 'base');
  await git(repo, 'add', 'app.txt');
  await git(repo, 'commit', '-m', 'test: tạo bản gốc');
  const base = await git(repo, 'rev-parse', 'HEAD');
  await writeFile(path.join(repo, 'app.txt'), 'head');
  await git(repo, 'commit', '-am', 'test: tạo bản nâng cấp');
  const head = await git(repo, 'rev-parse', 'HEAD');
  const branch = await git(repo, 'branch', '--show-current');
  await writeFile(path.join(repo, 'app.txt'), 'my uncommitted change');
  await writeFile(path.join(repo, 'new file.txt'), 'my untracked file');
  await writeFile(path.join(repo, 'staged.txt'), 'my staged file');
  await git(repo, 'add', 'staged.txt');
  const originalStatus = await git(repo, 'status', '--porcelain=v1');
  const workspace = await createRevisionWorkspaces(repo, base, head, runs);
  try {
    assert.equal(workspace.baseSha, base);
    assert.equal(workspace.headSha, head);
    assert.equal(await readFile(path.join(workspace.baseDir, 'app.txt'), 'utf8'), 'base');
    assert.equal(await readFile(path.join(workspace.headDir, 'app.txt'), 'utf8'), 'head');
    assert.deepEqual(workspace.dirtyFiles, ['app.txt', 'new file.txt', 'staged.txt']);
    assert.equal(await git(workspace.baseDir, 'branch', '--show-current'), '');
    assert.equal(await git(repo, 'branch', '--show-current'), branch);
    assert.equal(await git(repo, 'status', '--porcelain=v1'), originalStatus);
  } finally { await workspace.cleanup(); }
  await workspace.cleanup();
  await assert.rejects(access(workspace.baseDir));
  assert.equal(await readFile(path.join(repo, 'app.txt'), 'utf8'), 'my uncommitted change');
  assert.equal(await git(repo, 'status', '--porcelain=v1'), originalStatus);
  assert.equal((await git(repo, 'worktree', 'list', '--porcelain')).split('\n').filter(line => line.startsWith('worktree ')).length, 1);
});

test('invalid revisions are rejected before adding worktrees', async t => {
  const repo = await tempFixture(t);
  const runs = await tempFixture(t);
  await git(repo, 'init');
  await assert.rejects(createRevisionWorkspaces(repo, '--help', 'HEAD', runs), /Invalid base revision/);
  await assert.rejects(createRevisionWorkspaces(repo, 'nonexistent', 'HEAD', runs), /Invalid base revision/);
});

test('prepare rejects missing lockfiles before executing project scripts', async t => {
  const directory = await tempFixture(t);
  await writeFile(path.join(directory, 'package.json'), JSON.stringify({ scripts: { dev: 'node server.cjs' } }));
  await assert.rejects(prepareAndStart(directory, config, { install: false }), /package-lock.json is required/);
});

test('starts and stops an offline npm script from a directory with spaces', async t => {
  const directory = await tempFixture(t);
  await packageFixture(directory);
  await writeFile(path.join(directory, 'server.cjs'), `
const http = require('node:http');
const port = Number(process.argv[process.argv.indexOf('--port') + 1]);
http.createServer((_req, res) => { res.end('fixture-ready'); }).listen(port, '127.0.0.1');
`);
  const server = await prepareAndStart(directory, config, { install: false });
  try {
    assert.equal(await (await fetch(server.url)).text(), 'fixture-ready');
  } finally { await server.stop(); }
  await server.stop();
  await assert.rejects(fetch(server.url, { signal: AbortSignal.timeout(1000) }));
  const repeated = await prepareAndStart(directory, config, { install: false, port: Number(new URL(server.url).port) });
  try {
    assert.equal(repeated.url, server.url);
    assert.equal(await (await fetch(repeated.url)).text(), 'fixture-ready');
    await assert.rejects(prepareAndStart(directory, config, { install: false, port: Number(new URL(server.url).port) }), /port is unavailable/);
  } finally { await repeated.stop(); }
});

test('invalid requested ports are rejected before running scripts', async t => {
  const directory = await tempFixture(t);
  for (const port of [0, 80, 65536, 1234.5]) {
    await assert.rejects(prepareAndStart(directory, config, { install: false, port }), /port must be an integer/);
  }
});

test('build failure prevents starting the project and keeps output private', async t => {
  const directory = await tempFixture(t);
  await packageFixture(directory);
  await writeFile(path.join(directory, 'package.json'), JSON.stringify({
    name: 'workspace-fixture', version: '1.0.0', scripts: { dev: 'node server.cjs', build: 'node build.cjs' },
  }));
  await writeFile(path.join(directory, 'build.cjs'), `console.error('SECRET_DO_NOT_EXPORT'); process.exit(8);`);
  await assert.rejects(prepareAndStart(directory, { ...config, project: { ...config.project, buildScript: 'build' } }, { install: false }), error => {
    assert.match((error as Error).message, /Project build exited/);
    assert.doesNotMatch((error as Error).message, /SECRET_DO_NOT_EXPORT/);
    return true;
  });
});

test('readiness timeout kills the launched server and never exports raw logs', async t => {
  const directory = await tempFixture(t);
  await packageFixture(directory);
  await writeFile(path.join(directory, 'server.cjs'), `
console.error('SECRET_DO_NOT_EXPORT');
require('node:fs').writeFileSync('child-pid.txt', String(process.pid));
setInterval(() => {}, 1000);
`);
  await assert.rejects(prepareAndStart(directory, { ...config, timeoutMs: 1500 }, { install: false }), error => {
    assert.match((error as Error).message, /readiness timed out/);
    assert.doesNotMatch((error as Error).message, /SECRET_DO_NOT_EXPORT/);
    return true;
  });
  const pid = Number(await readFile(path.join(directory, 'child-pid.txt'), 'utf8'));
  assert.throws(() => process.kill(pid, 0));
});

test('early script failure is reported without raw output', async t => {
  const directory = await tempFixture(t);
  await packageFixture(directory);
  await writeFile(path.join(directory, 'server.cjs'), `console.error('SECRET_DO_NOT_EXPORT'); process.exit(7);`);
  await assert.rejects(prepareAndStart(directory, config, { install: false }), error => {
    assert.match((error as Error).message, /Project server exited/);
    assert.doesNotMatch((error as Error).message, /SECRET_DO_NOT_EXPORT/);
    return true;
  });
});

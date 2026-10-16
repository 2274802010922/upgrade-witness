import { cp, mkdir, readFile, readdir, stat } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { dirname, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { randomUUID } from 'node:crypto';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const fixture = join(root, 'examples', 'tailwind-migration');
const runId = `${new Date().toISOString().replace(/[:.]/g, '-')}-${randomUUID().slice(0, 8)}`;
const workspaceRoot = join(root, '.uw', 'demo-project');
const project = join(workspaceRoot, runId);
const output = join(root, '.uw', 'demo-reports', runId);
const demoIdentity = {
  ...process.env,
  GIT_AUTHOR_NAME: 'Upgrade Witness Synthetic Demo',
  GIT_AUTHOR_EMAIL: 'synthetic-demo@upgrade-witness.invalid',
  GIT_COMMITTER_NAME: 'Upgrade Witness Synthetic Demo',
  GIT_COMMITTER_EMAIL: 'synthetic-demo@upgrade-witness.invalid',
};

function command(executable, args, cwd = root, options = {}) {
  const result = spawnSync(executable, args, { cwd, encoding: 'utf8', ...options });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`${executable} failed (${result.status}).\n${result.stderr ?? ''}\n${result.stdout ?? ''}`);
  return result.stdout.trim();
}

async function filesBelow(directory, prefix = '') {
  const result = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    if (['node_modules', 'dist', '.git', '.uw'].includes(entry.name)) continue;
    if (entry.isSymbolicLink()) throw new Error(`Demo fixtures may not contain symlinks: ${entry.name}`);
    const name = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) result.push(...await filesBelow(join(directory, entry.name), name));
    else if (entry.isFile()) result.push(name);
  }
  return result.sort();
}

async function overlay(source, filenames) {
  // Only write fixture files into this run's newly created child directory.
  const child = relative(workspaceRoot, project);
  if (!child || child.startsWith(`..${sep}`) || child === '..' || child.includes(sep)) {
    throw new Error('Unexpected demo workspace boundary');
  }
  for (const filename of filenames) {
    const destination = join(project, filename);
    await mkdir(dirname(destination), { recursive: true });
    await cp(join(source, filename), destination, { force: true });
  }
}

function commit(message) {
  command('git', ['add', '--all'], project);
  command('git', ['-c', 'commit.gpgsign=false', '-c', 'core.hooksPath=', 'commit', '--quiet', '-m', message], project, { env: demoIdentity });
  return command('git', ['rev-parse', 'HEAD'], project);
}

try {
  await stat(join(root, 'dist', 'src', 'cli.js'));
  const baseFiles = await filesBelow(join(fixture, 'base'));
  const headFiles = await filesBelow(join(fixture, 'head'));
  if (JSON.stringify(baseFiles) !== JSON.stringify(headFiles)) {
    throw new Error('The two fixture trees must have the same files so overlay needs no deletion.');
  }
  await mkdir(workspaceRoot, { recursive: true });
  await mkdir(project); // Fail if a directory already exists; never reuse a user path.
  await overlay(join(fixture, 'base'), baseFiles);
  command('git', ['init', '--quiet', '--initial-branch=demo'], project);
  const base = commit('test: tạo trạng thái gốc cho bản minh họa tổng hợp');
  await overlay(join(fixture, 'head'), headFiles);
  const head = commit('test: mô phỏng lỗi bố cục và biến thể sau nâng cấp');
  await mkdir(dirname(output), { recursive: true });
  console.log(`Synthetic demo repository: ${project}`);
  console.log(`Comparing ${base.slice(0, 12)} -> ${head.slice(0, 12)}`);
  const result = spawnSync(process.execPath, [
    join(root, 'dist', 'src', 'cli.js'), 'verify', '--repo', project,
    '--base', base, '--head', head, '--config', join(fixture, 'witness.config.json'),
    '--out', output, '--trust-project',
  ], { cwd: root, stdio: 'inherit' });
  if (result.error) throw result.error;
  const report = JSON.parse(await readFile(join(output, 'report.json'), 'utf8'));
  const regressions = report.findings.filter(f => f.kind === 'regression');
  const expected = ['column-count', 'primary-variant'];
  const observed = expected.every(id => regressions.some(f => f.scenarioId === 'desktop-overview' && f.assertionId === id));
  if (result.status !== 1 || report.exitCode !== 1 || report.coverage.notRun !== 0 || !observed) {
    throw new Error(`Expected all assertions to run, two known assertion regressions and verifier exit 1; got process=${result.status}, report=${report.exitCode}, notRun=${report.coverage.notRun}, complete=${report.complete}. See ${output}`);
  }
  console.log('\nExpected regressions confirmed:');
  for (const finding of regressions) console.log(`- ${finding.scenarioId}/${finding.assertionId}: ${finding.observation}`);
  console.log(`Coverage: ${report.coverage.planned} planned checks, ${report.coverage.notRun} not run; complete=${report.complete}.`);
  console.log('Visual differences remain review findings; they are not extra confirmed bugs.');
  console.log(`\nReport: ${join(output, 'report.json')}`);
  console.log('Demo passed: the verifier correctly returned 1 for the synthetic regressions.');
} catch (error) {
  console.error(`Demo failed: ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
}

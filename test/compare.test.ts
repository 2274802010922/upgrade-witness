import assert from 'node:assert/strict';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { PNG } from 'pngjs';
import { compareCaptures } from '../src/compare.js';
import { parseConfig } from '../src/schema.js';
import type { ScenarioCapture } from '../src/types.js';

const config = () => parseConfig({ schemaVersion: 1, scenarios: [{ id: 'panel', route: '/', screenshot: false, assertions: [{ id: 'shown', selector: '#panel', kind: 'visible', expected: true }] }] });
const capture = (actual: boolean | null, extra: Partial<ScenarioCapture> = {}): ScenarioCapture => ({ scenarioId: 'panel', browserVersion: 'test-browser', checks: [{ id: 'shown', expected: true, actual, passed: actual === true }], ...extra });

test('clean hidden controls pass without treating false as a missing observation', async () => {
  const c = config();
  c.scenarios[0].assertions[0].expected = false;
  const result = await compareCaptures(c, [capture(false)], [capture(false)]);
  assert.deepEqual(result, { complete: true, exitCode: 0, findings: [], coverage: { planned: 2, passed: 2, failed: 0, notRun: 0 } });
});

test('a new failed assertion is a regression, with auditable coverage', async () => {
  const result = await compareCaptures(config(), [capture(true)], [capture(false)]);
  assert.equal(result.exitCode, 1);
  assert.equal(result.complete, true);
  assert.deepEqual(result.coverage, { planned: 2, passed: 1, failed: 1, notRun: 0 });
  assert.equal(result.findings[0].kind, 'regression');
  assert.match(result.findings[0].observation, /does not establish dependency causality/);
});

test('declared intended expectations are applied independently of captured passed flags', async () => {
  const c = config();
  c.intendedChanges = [{ scenarioId: 'panel', assertionId: 'shown', expected: false, reason: 'The panel is intentionally hidden.' }];
  const result = await compareCaptures(c, [capture(true)], [capture(false)]);
  assert.equal(result.exitCode, 0);
  assert.equal(result.findings[0].kind, 'intended-change');
  assert.equal(result.findings[0].reason, c.intendedChanges[0].reason);
  assert.deepEqual(result.coverage, { planned: 2, passed: 2, failed: 0, notRun: 0 });
  const unmet = await compareCaptures(c, [capture(true)], [capture(true)]);
  assert.equal(unmet.exitCode, 1, 'the old value does not satisfy an explicit new requirement');
});

test('an already failing baseline cannot establish a new regression', async () => {
  const result = await compareCaptures(config(), [capture(false)], [capture(false)]);
  assert.equal(result.exitCode, 2);
  assert.equal(result.complete, false);
  assert.deepEqual(result.findings.map(f => f.kind), ['baseline-issue']);
});

test('unstable repeated baseline assertion prevents a regression claim', async () => {
  const result = await compareCaptures(config(), [capture(true)], [capture(false)], [capture(false)]);
  assert.equal(result.exitCode, 2);
  assert.equal(result.findings[0].kind, 'baseline-issue');
  assert.equal(result.findings.some(f => f.kind === 'regression'), false);
});

test('a regression needs a matching head repeat when repeat capture is requested', async () => {
  const stable = await compareCaptures(config(), [capture(true)], [capture(false)], [capture(true)], [capture(false)]);
  assert.equal(stable.exitCode, 1);
  assert.equal(stable.findings[0].kind, 'regression');
  for (const repeat of [[], [capture(true)], [capture(false, { checks: [] })]]) {
    const result = await compareCaptures(config(), [capture(true)], [capture(false)], [capture(true)], repeat);
    assert.equal(result.exitCode, 2);
    assert.equal(result.complete, false);
    assert.equal(result.findings.some(f => f.kind === 'regression'), false);
    assert.ok(result.findings.some(f => f.kind === 'inconclusive'));
  }
});

test('an unstable head assertion does not discard a separate repeatable regression', async () => {
  const c = config();
  c.scenarios[0].assertions.push({ id: 'other', kind: 'visible', selector: '#other', expected: true });
  const a = capture(true);
  a.checks.push({ id: 'other', expected: true, actual: true, passed: true });
  const b = capture(false);
  b.checks.push({ id: 'other', expected: true, actual: false, passed: false });
  const h = capture(true);
  h.checks.push({ id: 'other', expected: true, actual: false, passed: false });
  const result = await compareCaptures(c, [a], [b], [a], [h]);
  assert.equal(result.exitCode, 1);
  assert.equal(result.complete, false);
  assert.ok(result.findings.some(f => f.kind === 'inconclusive' && f.assertionId === 'shown'));
  assert.ok(result.findings.some(f => f.kind === 'regression' && f.assertionId === 'other'));
});

test('missing, duplicate, and errored observations never pass', async () => {
  for (const head of [[], [capture(true), capture(true)], [capture(true, { checks: [{ id: 'shown', expected: true, actual: null, passed: false, error: 'Selector invalid.' }] })]]) {
    const result = await compareCaptures(config(), [capture(true)], head);
    assert.equal(result.exitCode, 2);
    assert.equal(result.complete, false);
    assert.equal(result.coverage.notRun, 1);
  }
  const repeatMissing = await compareCaptures(config(), [capture(true)], [capture(true)], []);
  assert.equal(repeatMissing.exitCode, 2);
});

test('a confirmed regression retains exit 1 even if other evidence is incomplete', async () => {
  const result = await compareCaptures(config(), [capture(true)], [capture(false, { error: 'Screenshot could not be completed.' })]);
  assert.equal(result.exitCode, 1);
  assert.equal(result.complete, false);
  assert.ok(result.findings.some(f => f.kind === 'inconclusive'));
  assert.ok(result.findings.some(f => f.kind === 'regression'));
});

test('visual-only differences, intended differences, unstable images, and missing images remain reviewable', async t => {
  const dir = await mkdtemp(join(tmpdir(), 'uw-images-'));
  t.after(() => rm(dir, { recursive: true, force: true }));
  async function png(name: string, red: number, blue: number): Promise<string> {
    const image = new PNG({ width: 16, height: 16 });
    for (let i = 0; i < image.data.length; i += 4) {
      image.data[i] = red; image.data[i + 1] = 0; image.data[i + 2] = blue; image.data[i + 3] = 255;
    }
    const file = join(dir, name);
    await writeFile(file, PNG.sync.write(image));
    return file;
  }
  const red = await png('red.png', 255, 0);
  const blue = await png('blue.png', 0, 255);
  const c = config();
  c.scenarios[0].screenshot = true;
  const a = capture(true, { screenshotPath: red });
  const b = capture(true, { screenshotPath: blue });
  const same = await compareCaptures(c, [a], [a], [a]);
  assert.equal(same.exitCode, 0);
  const unknown = await compareCaptures(c, [a], [b]);
  assert.equal(unknown.exitCode, 2);
  assert.equal(unknown.findings[0].kind, 'needs-review');
  assert.deepEqual(unknown.findings[0].evidenceIds, ['base-panel', 'head-panel']);
  const unstable = await compareCaptures(c, [a], [b], [b]);
  assert.equal(unstable.exitCode, 2);
  assert.ok(unstable.findings.some(f => f.kind === 'baseline-issue'));
  assert.equal(unstable.findings.some(f => f.kind === 'regression'), false);
  const unstableHead = await compareCaptures(c, [a], [b], [a], [a]);
  assert.equal(unstableHead.exitCode, 2);
  assert.ok(unstableHead.findings.some(f => f.kind === 'inconclusive'));
  assert.equal(unstableHead.findings.some(f => f.kind === 'needs-review'), false);
  const missing = await compareCaptures(c, [a], [capture(true)]);
  assert.equal(missing.exitCode, 2);
  assert.ok(missing.findings.some(f => f.kind === 'inconclusive'));
  c.intendedChanges = [{ scenarioId: 'panel', assertionId: 'shown', expected: false, reason: 'The panel is intentionally hidden.' }];
  const intendedVisual = await compareCaptures(c, [a], [capture(false, { screenshotPath: blue })]);
  assert.equal(intendedVisual.exitCode, 2, 'an assertion intent cannot approve every visual change');
  assert.ok(intendedVisual.findings.some(f => f.kind === 'intended-change'));
  assert.ok(intendedVisual.findings.some(f => f.kind === 'needs-review'));
});

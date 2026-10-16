import { readFile } from 'node:fs/promises';
import pixelmatch from 'pixelmatch';
import { PNG } from 'pngjs';
import type { Check, Comparison, Finding, ScenarioCapture, WitnessConfig } from './types.js';

type VisualResult = 'same' | 'different' | 'unavailable';

async function compareImages(first: string | undefined, second: string | undefined, maxRatio: number): Promise<VisualResult> {
  if (!first || !second) return 'unavailable';
  try {
    const [a, b] = await Promise.all([readFile(first).then(buffer => PNG.sync.read(buffer)), readFile(second).then(buffer => PNG.sync.read(buffer))]);
    if (a.width !== b.width || a.height !== b.height) return 'different';
    const pixels = a.width * a.height;
    if (!pixels) return 'unavailable';
    const changed = pixelmatch(a.data, b.data, undefined, a.width, a.height, { threshold: 0.1 });
    return changed / pixels > maxRatio ? 'different' : 'same';
  } catch {
    return 'unavailable';
  }
}

function onlyCapture(captures: ScenarioCapture[], scenarioId: string): ScenarioCapture | undefined {
  const matches = captures.filter(capture => capture.scenarioId === scenarioId);
  return matches.length === 1 ? matches[0] : undefined;
}

function onlyCheck(capture: ScenarioCapture | undefined, assertionId: string): Check | undefined {
  const matches = capture?.checks.filter(check => check.id === assertionId) ?? [];
  return matches.length === 1 ? matches[0] : undefined;
}

export async function compareCaptures(config: WitnessConfig, base: ScenarioCapture[], head: ScenarioCapture[], baseRepeat?: ScenarioCapture[], headRepeat?: ScenarioCapture[]): Promise<Comparison> {
  const findings: Finding[] = [];
  const coverage = { planned: config.scenarios.reduce((total, scenario) => total + scenario.assertions.length * 2, 0), passed: 0, failed: 0, notRun: 0 };
  const add = (finding: Omit<Finding, 'id'>) => findings.push({ id: `finding-${findings.length + 1}`, ...finding });

  for (const scenario of config.scenarios) {
    const before = onlyCapture(base, scenario.id);
    const after = onlyCapture(head, scenario.id);
    const repeated = baseRepeat ? onlyCapture(baseRepeat, scenario.id) : undefined;
    const headRepeated = headRepeat ? onlyCapture(headRepeat, scenario.id) : undefined;
    const evidenceIds = [before?.screenshotPath ? `base-${scenario.id}` : undefined, after?.screenshotPath ? `head-${scenario.id}` : undefined, repeated?.screenshotPath ? `base-repeat-${scenario.id}` : undefined, headRepeated?.screenshotPath ? `head-repeat-${scenario.id}` : undefined].filter((id): id is string => Boolean(id));
    const detail = { scenarioId: scenario.id, evidenceIds };
    let unstable = false;
    let headUnstable = false;
    if (!before || !after || before.error || after.error) {
      add({ ...detail, kind: 'inconclusive', observation: 'One or both scenario captures are missing, duplicated, or incomplete. No clean result can be established.' });
    }
    if (baseRepeat && (!repeated || repeated.error)) {
      unstable = true;
      add({ ...detail, kind: 'inconclusive', observation: 'The requested baseline repeat could not be completed; stability is unverified.' });
    }
    if (headRepeat && (!headRepeated || headRepeated.error)) {
      headUnstable = true;
      add({ ...detail, kind: 'inconclusive', observation: 'The requested head repeat could not be completed; repeatability is unverified.' });
    }

    for (const assertion of scenario.assertions) {
      const a = onlyCheck(before, assertion.id);
      const b = onlyCheck(after, assertion.id);
      const r = onlyCheck(repeated, assertion.id);
      const h = onlyCheck(headRepeated, assertion.id);
      const intent = config.intendedChanges.find(item => item.scenarioId === scenario.id && item.assertionId === assertion.id);
      const headExpected = intent?.expected ?? assertion.expected;
      for (const [check, expected] of [[a, assertion.expected], [b, headExpected]] as const) {
        if (!check || check.error) coverage.notRun++;
        else if (check.actual === expected) coverage.passed++;
        else coverage.failed++;
      }
      const checkDetail = { ...detail, assertionId: assertion.id, base: a?.actual ?? null, head: b?.actual ?? null, expected: headExpected };
      if (!a || !b || a.error || b.error) {
        add({ ...checkDetail, kind: 'inconclusive', observation: 'This assertion did not produce a usable observation in both states.' });
        continue;
      }
      if (baseRepeat && (!r || r.error || r.actual !== a.actual)) {
        unstable = true;
        add({ ...checkDetail, kind: !r || r.error ? 'inconclusive' : 'baseline-issue', observation: !r || r.error ? 'The baseline repeat did not evaluate this assertion.' : 'The baseline assertion changed between repeated captures; a stable comparison is unavailable.' });
        continue;
      }
      if (a.actual !== assertion.expected) {
        add({ ...checkDetail, kind: 'baseline-issue', observation: 'The original baseline already violates the declared expectation; this is not established as a new regression.' });
        continue;
      }
      if (headRepeat && (!h || h.error || h.actual !== b.actual)) {
        add({ ...checkDetail, kind: 'inconclusive', observation: !h || h.error ? 'The head repeat did not evaluate this assertion; a repeatable outcome is unavailable.' : 'The head observation changed on repeat; a repeatable outcome is unavailable.' });
        continue;
      }
      if (b.actual !== headExpected) {
        add({ ...checkDetail, kind: 'regression', observation: 'The baseline met its expectation and the head did not meet the declared head expectation. This observation does not establish dependency causality.' });
      } else if (intent && b.actual !== a.actual) {
        add({ ...checkDetail, kind: 'intended-change', observation: 'The observed change matches the declared intended expectation.', reason: intent.reason });
      }
    }

    if (scenario.screenshot) {
      if (baseRepeat && repeated && !repeated.error) {
        const repeatVisual = await compareImages(before?.screenshotPath, repeated.screenshotPath, scenario.maxDiffPixelRatio);
        if (repeatVisual !== 'same') {
          unstable = true;
          add({ ...detail, kind: repeatVisual === 'different' ? 'baseline-issue' : 'inconclusive', observation: repeatVisual === 'different' ? 'Baseline screenshots differ beyond the configured tolerance; visual stability is unverified.' : 'Baseline-repeat screenshot evidence is missing or unreadable.' });
        }
      }
      if (headRepeat && headRepeated && !headRepeated.error) {
        const repeatVisual = await compareImages(after?.screenshotPath, headRepeated.screenshotPath, scenario.maxDiffPixelRatio);
        if (repeatVisual !== 'same') {
          headUnstable = true;
          add({ ...detail, kind: 'inconclusive', observation: repeatVisual === 'different' ? 'Head screenshots change between repeated captures; the visual difference is not repeatable.' : 'Head-repeat screenshot evidence is missing or unreadable.' });
        }
      }
      const visual = await compareImages(before?.screenshotPath, after?.screenshotPath, scenario.maxDiffPixelRatio);
      if (visual === 'unavailable') add({ ...detail, kind: 'inconclusive', observation: 'Required screenshot evidence is missing or unreadable.' });
      else if (visual === 'different' && !unstable && !headUnstable) add({ ...detail, kind: 'needs-review', observation: 'Screenshots differ beyond the configured tolerance. Visual evidence alone cannot confirm a regression or prove that an intended assertion explains the whole image.' });
    }
  }

  const unresolved = findings.some(finding => ['inconclusive', 'needs-review', 'baseline-issue'].includes(finding.kind));
  const regression = findings.some(finding => finding.kind === 'regression');
  return { complete: !unresolved && coverage.notRun === 0, exitCode: regression ? 1 : unresolved || coverage.notRun > 0 ? 2 : 0, findings, coverage };
}

export type Assertion = {
  id: string;
  selector: string;
  kind: 'visible' | 'text' | 'attribute' | 'style';
  name?: string;
  expected: string | boolean;
};
export type Action = { type: 'click' | 'fill' | 'press'; selector: string; value?: string };
export type Scenario = {
  id: string;
  route: string;
  viewport: { width: number; height: number };
  actions: Action[];
  assertions: Assertion[];
  mask: string[];
  screenshot: boolean;
  maxDiffPixelRatio: number;
};
export type Intent = { scenarioId: string; assertionId: string; expected: string | boolean; reason: string };
export type WitnessConfig = {
  schemaVersion: 1;
  project: { startScript: string; buildScript?: string };
  scenarios: Scenario[];
  intendedChanges: Intent[];
  timeoutMs: number;
};
export type Check = { id: string; expected: string | boolean; actual: string | boolean | null; passed: boolean; error?: string };
export type ScenarioCapture = {
  scenarioId: string; checks: Check[]; screenshotPath?: string; error?: string;
  browserVersion: string;
};
export type FindingKind = 'regression' | 'intended-change' | 'needs-review' | 'baseline-issue' | 'inconclusive';
export type Finding = {
  id: string; scenarioId: string; assertionId?: string; kind: FindingKind;
  observation: string; base?: string | boolean | null; head?: string | boolean | null;
  expected?: string | boolean; reason?: string; evidenceIds: string[];
};
export type Evidence = { id: string; path: string; sha256: string; mediaType: string };
export type RunReport = {
  schemaVersion: '0.1'; toolVersion: string; runId: string; createdAt: string;
  baseSha: string; headSha: string; contractSha256: string;
  config: WitnessConfig; environment: { node: string; platform: string; browser: string };
  complete: boolean; exitCode: number; findings: Finding[]; evidence: Evidence[];
  coverage: { planned: number; passed: number; failed: number; notRun: number };
  limitations: string[];
};

export type Comparison = Pick<RunReport, 'complete' | 'exitCode' | 'findings' | 'coverage'>;

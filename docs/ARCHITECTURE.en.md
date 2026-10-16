> Initial design snapshot, 16 October 2026. See [README](../README.md) and [STATUS](STATUS.md) for current implementation status; this document is not an execution log.

# Upgrade Witness — proposed architecture

[Tiếng Việt](ARCHITECTURE.vi.md) · Design dated 16 October 2026

**Status: design proposal.** No CLI, package, compatibility claim, or benchmark result in this document represents released software. Example commands describe the interface to build.

## 1. Product boundary

Upgrade Witness helps a developer answer: “What observable behavior changed during this upgrade, which changes violate our expectations, and can someone reproduce the evidence?” Its first supported migration should be **React + Vite + Tailwind CSS 3 → 4, npm, one application, Chromium**. Linux CI and Windows local execution are separate supported environments; each comparison runs both revisions in the same environment. Next.js, monorepos, pnpm, additional browsers, other agent clients, and MCP follow validated demand.

There are two entry points: help perform an upgrade, or verify an upgrade already committed. Both converge on two pinned Git revisions and one frozen verification contract. Preserve the user's own tests and use the existing agent to inspect code, suggest scenarios, perform authorized edits, and explain findings. The CLI handles repeatable mechanics. Do not build a model router, autonomous LLM loop, or a general codemod framework.

Tailwind's official guide recommends reviewing the migration diff and testing the result in a browser; it also documents changes that may legitimately alter appearance. Those changes inform investigations, but do not automatically establish the product owner's intent. [Official upgrade guide](https://tailwindcss.com/docs/upgrade-guide)

## 2. Components and ownership

```text
Developer request
      │
Host coding agent + skill
      │ proposed structured commands
TypeScript CLI
  ├─ preflight and revision workspace manager
  ├─ React/Vite + npm adapter
  ├─ frozen scenario runner (Playwright)
  ├─ deterministic comparisons and evidence manifest
  └─ redacted JSON + static HTML + Markdown reports
```

Keep `skills/upgrade-witness/SKILL.md` concise: selection criteria, the two modes, evidence rules, stop conditions, and references loaded only as needed. Store the scenario contract, investigation procedure, and Tailwind-specific mapping in separate references. Put executable logic in one small `packages/cli` package with internal modules; a multi-package framework would be premature.

The core owns revision handling, contract validation, browser orchestration, artifact hashing, and reporting. The adapter owns project detection, supported-version checks, install/build/serve commands, readiness probes, and migration-specific risk hints. Its result is structured data, never a shell command assembled from issue text. Prefer process arguments over shell interpolation. The adapter must report unsupported configurations explicitly. The project uses the selected MIT license; preserve third-party notices and record fixture provenance.

## 3. Preconditions and immutable inputs

Require Git, an explicitly supported pinned Node/npm pair, an npm lockfile for each revision, a reproducible local start command, and deterministic fixture data. Select and test the exact runtime versions in week 1; do not advertise an untested matrix. Record the CLI version, browser revision, OS image, package versions, lockfile hashes, config digest, and both full commit SHAs.

Resolve refs once. Create detached temporary worktrees beneath a tool-owned run directory and install each revision independently. Git worktrees support separate working trees without replacing the user's current checkout. [Git worktree documentation](https://git-scm.com/docs/git-worktree)

A dirty source checkout remains untouched. Explicit committed base/head refs can still be verified; list all uncommitted files as excluded input. If the requested candidate exists only in uncommitted changes, explain the missing immutable input and stop that path. A later feature may create an explicitly requested snapshot without changing the user's index; v0.1 should not silently stash, reset, commit, clean, or copy ignored secrets. Cleanup deletes only validated tool-owned paths and refuses paths outside the run root.

Run `npm ci` in the temporary checkouts. A manifest/lockfile mismatch is a precondition failure, not something to “fix” while measuring the candidate. Dependency lifecycle scripts and project start commands execute project code; run only in the user's trusted development context, and never expose secrets to untrusted pull-request code in CI. [npm ci documentation](https://docs.npmjs.com/cli/v11/commands/npm-ci/)

## 4. End-to-end execution contract

1. **Discover:** identify the migration, supported stack, routes, meaningful interactions, existing tests, intended changes, and runtime constraints.
2. **Freeze expectations:** create a versioned scenario file from the base revision and developer requirements. Record fixture inputs, viewport, interaction steps, assertions, screenshot regions, masks, tolerances, and explicit intended-change entries. Hash this contract before migration or before the verification agent inspects candidate outcomes.
3. **Qualify the base:** run each required scenario twice. Broken assertions, missing fonts, unstable pixels beyond the declared threshold, unavailable data, or readiness failures prevent a trustworthy comparison. Show them as baseline problems.
4. **Produce or select the candidate:** for a new upgrade, let the host agent run the selected pinned official migrator and edit an isolated candidate workspace. Verify only after an immutable candidate SHA exists. For an existing upgrade, select its SHA; disclose that expectations were reconstructed after the upgrade and exclude head outputs from baseline contract construction.
5. **Run both sides:** execute the same frozen scenarios and original assertions. Candidate-only exploratory tests are useful but cannot replace the original oracle or turn a failed check into a pass.
6. **Compare and investigate:** rerun failing checks once, correlate evidence with the code diff and migration guide, and separate observed facts from explanations.
7. **Review and optionally repair:** present a minimal proposed patch when requested. Each repair creates a new candidate identity and a new report. Changing expectations requires an explicit recorded rationale; preserve the earlier failed result.
8. **Export:** produce coverage, findings, limitations, and reproducible commands with an integrity manifest. The report never asserts that the whole application is safe.

No automatic baseline refresh is allowed during verification. An upstream documented behavior change is a candidate for acceptance, not blanket permission to weaken a test.

## 5. Deterministic browser protocol

Pin the Playwright version and bundled Chromium revision. Fix viewport and device scale, locale, timezone, color scheme, reduced motion, font assets, clock and test data where supported. Use fixture-backed network responses; list any live dependency as a limitation. Wait for app-specific readiness and font loading. Disable animations consistently. Run both builds on the same local origin sequentially, with fresh storage and server teardown between them.

Compare screenshots only within one platform/browser configuration. Playwright explicitly notes that rendering varies with environment; Linux and Windows images are not interchangeable baselines. [Playwright visual comparisons](https://playwright.dev/docs/test-snapshots)

Begin with two widths, 375 and 1440 CSS pixels, and a small set of declared flows. Add 768/1024 widths and required VI/EN states in alpha where the pilot application uses them. Prefer role-based locators or stable test IDs. Collect focused computed CSS and DOM attributes around a failed assertion, not an entire application dump. Pixel changes alone mean “needs review” unless a frozen visual requirement or accepted human disposition identifies a violation.

## 6. Findings and evidence model

Proposed JSON structure:

```json
{
  "schemaVersion": "0.1",
  "runId": "local-run-id",
  "baseSha": "full-sha",
  "headSha": "full-sha",
  "contractSha256": "digest",
  "complete": false,
  "findings": [{
    "id": "UW-001",
    "scenarioId": "navigation.mobile",
    "kind": "regression",
    "observation": "The menu opens on base and remains hidden on head.",
    "assertionId": "menu.visible-after-click",
    "suspectedCause": {"text": "Changed hidden/display behavior", "status": "unverified"},
    "evidenceIds": ["base-dom", "head-dom", "head-shot"],
    "intentDecision": null,
    "reproducedRuns": 2
  }],
  "coverage": {"planned": 8, "passed": 6, "failed": 1, "notRun": 1}
}
```

Allowed finding kinds: `regression`, `intended-change`, `needs-review`, `baseline-issue`, `inconclusive`. Observation is a measured fact. Suspected cause is separately qualified and linked to candidate code. Intended changes carry the approving decision and scope. Coverage records every scenario/environment outcome and exclusion; percentages always name their denominator. A confirmed assertion failure proves the observed regression, not its root cause.

Each evidence item records a relative path, media type, SHA-256 of the sanitized bytes, capture step, source revision, and environment digest. Hash the frozen contract and final manifest separately. Hashes detect modification; they do not prove truthful collection or code causality.

## 7. Proposed command and CI contract

These commands are **design examples, unavailable until implemented**:

```text
uw doctor --config witness.config.json
uw plan --base <sha> --out <run-directory>
uw capture --run <run-directory> --revision base
uw compare --run <run-directory> --head <sha>
uw report --run <run-directory> --format html,markdown,json
```

`plan` freezes the base contract; `compare` refuses changed contracts and records head identity. All commands emit a stable machine-readable result. Proposed exit codes: **0** = every required check completed, no regression or unresolved difference; **1** = at least one confirmed regression; **2** = inconclusive, incomplete, unstable, unsupported, or preflight failure without a confirmed regression; **64** = invalid CLI usage. If regressions and incompleteness coexist, return 1 and `complete:false`; never hide the incomplete checks. CI publishes artifacts on failure, fails on 1/2/64, and treats baseline update as a separate reviewed change.

## 8. Privacy and validation gates

Default to fixture data and no HAR/full tracing. Mask sensitive selectors before screenshots; allowlist DOM fields; redact logs, paths, and URLs before model access and export. Store only sanitized artifacts by default, then compute their hashes. A redaction failure blocks export. Render untrusted text with HTML escaping and never execute report content. Document local retention and explicit deletion; telemetry is off by default.

The CLI runs locally. The host agent may transmit selected context or artifacts to its model provider under that client's configuration. Therefore “local runner” must never be marketed as “your code never leaves your machine.” Raw diagnostic capture, if later added, needs a separate local-only workflow and clear handling rules.

Plan around eight weeks, approximately 180 working hours at 20–25 hours per week, plus a 25% contingency allowance of 45 hours. Using the full 225-hour allowance extends the calendar to roughly 9–12 weeks at that pace. Reduce scope or move the release if a gate fails; do not claim the calendar itself proves readiness.

| Phase | Technical gate |
|---|---|
| W1 discovery | Freeze supported versions, three realistic scenarios, contract and adapter boundaries. |
| W2 vertical slice | One real before/after fixture produces a repeatable report in one primary environment. |
| W3–4 alpha | Complete schema, redaction, dirty-tree protection, intended-change decisions, timeouts and cleanup; validate both Windows local and Linux CI. |
| W5 evaluation | Held-out bugs plus clean upgrades; compare the same agent with and without the skill; measure detection, false alarms, completion, time and cost. |
| W6 pilot | Three external projects or maintainer-guided trials; document setup failures and time to first useful finding. |
| W7 release candidate | Reproduce results from clean machines, prepare limitations and evidence, and validate bilingual instructions. |
| W8 conditional public beta | Release only if readiness gates pass, and only for the tested matrix; otherwise extend validation or narrow scope. |

Use unit tests for schema, exit-code precedence, path boundaries and redaction; integration tests for base failure, unstable output, no-op upgrade, real regression and intended changes. Test process cleanup and Windows paths with spaces. Independent agent trials should receive the skill and raw fixture, without being told the hidden bug. MCP and additional clients enter the roadmap only after the core report and CLI contract are stable.

> Initial design snapshot, 16 October 2026. See [README](../README.md) and [STATUS](STATUS.md) for current implementation status; this document is not an execution log.

# Upgrade Witness — evidence and validation plan

[Tiếng Việt](BENCHMARK.vi.md) · Planning snapshot: 16 October 2026

**Status: proposed protocol; no benchmark or pilot results yet.** Scope: AI skill plus small CLI for React/Vite, npm, Tailwind v3→v4. Test preservation of observable UI behavior and reproducible evidence, without asserting whole-application correctness.

## Evidence registry

These qualitative signals are not a representative survey or market-size estimate. Status checked on 16 October 2026; preserve retrieval dates with each release.

| Source | Date and checked status | Supported observation | Limitation |
|---|---|---|---|
| [Tailwind #20435](https://github.com/tailwindlabs/tailwindcss/issues/20435) | Opened 27 Aug 2026; Open | Upgrade rewriting can affect conditional `variant` values. Typed props can fail typecheck; a data attribute can escape it. | Reporter evidence; our fixture must independently reproduce the behavior. |
| [Tailwind #19555](https://github.com/tailwindlabs/tailwindcss/issues/19555) | Opened 12 Jan 2026; Open | Migrating custom `maxWidth` configuration can affect column layout. | Specific configuration, not every migration. |
| [MCP servers #4856](https://github.com/modelcontextprotocol/servers/issues/4856) | Opened 26 Sep 2026; closed as completed 7 Oct 2026 | An SDK migration explicitly relies on characterization tests to preserve behavior. | Adjacent workflow evidence; outside this frontend MVP and not an unresolved bug. |

[Codemod AI](https://codemod.com/blog/npx-codemod-ai) already combines migration skills, tools, and tests. [Playwright](https://playwright.dev/docs/test-snapshots) already supports screenshot comparisons and documents environment sensitivity. Integrate these capabilities. The hypothesis to test is better selection, explanation, and replay of migration evidence; do not claim invention of AI migration or visual testing.

## Twenty fixtures and independent ground truth

Create 10 development and 10 held-out fixtures, each split containing:

| Category | Per split | Expected result |
|---|---:|---|
| Actual regression | 5 | Find one primary defect per fixture and demonstrate it before/after |
| Clean migration | 2 | No regression claim; report checked coverage |
| Intended change | 2 | Explain the documented/approved change without filing a regression |
| Infrastructure failure | 1 | Inconclusive with the failing prerequisite and recovery action |

Candidate defect families: configuration/layout interaction, incorrect prop or data-attribute rewrite, responsive overflow, focus/interaction state, and CSS utility semantics. Author minimal independent examples; public issues are inspiration rather than permission to copy repositories. Include migrations where static checks pass. Infrastructure cases should include a controlled server or dependency failure, not an application defect relabeled after a failed run.

For each fixture, store pinned before/after commits, npm lockfiles, route/state requirements, intended-change notes, trusted assertions, and a private adjudication manifest. A second reviewer validates the expected behavior. Use different components and mechanisms across splits; near-duplicate templates are leakage. Keep bug labels, reference patches, oracle tests, revealing Git history, and adjudication files outside **all agent-accessible filesystem, tool, and network scope**, in a separate adjudication environment with access controls. A sibling folder outside the checkout is insufficient. Intent requirements remain visible to both arms because users need a way to express desired behavior.

## Paired experiment

Three repetitions per fixture per arm: **20 × 3 × 2 = 120 runs**, 60 development and 60 held-out.

- **A:** agent with shell, browser, Playwright, official migration tooling, and the agreed task prompt.
- **B:** the same configuration and base tools, plus the Upgrade Witness skill and CLI.

This estimates the package's contribution, not prompt text alone. A later ablation can share the CLI and vary only the skill. Record model/agent versions, settings, tools, prompts, package hashes, and billing. Use fresh sessions/checkouts, randomized paired order, identical cache policy, and no cross-run memory. Allow identical pinned documentation; block issue solutions and hidden benchmark answers during scoring.

Proposed per-run limits: 20 minutes and 40,000 aggregate input/output tokens, including skill text and tool context. Apply identical limits and preserve timeout failures. Calibrate on development fixtures before freezing; never change limits after viewing held-out outcomes. The primary experiment verifies supplied before/after states. Report agent-performed migration and repair separately so a lucky repair does not hide a missed original defect.

Pin OS image, browser, fonts, viewport, timezone, locale, data seed, and animation handling. Use 375/768/1024/1440px where required by each fixture. Record mandatory routes, states, and actions in advance. Baselines and candidates run in equivalent environments; no automatic snapshot acceptance.

## Scoring and proposed W5 gates

An adjudicator, blinded to arm where practical, matches findings, merges duplicates, and checks evidence. A screenshot difference alone is not a confirmed regression. Report every run and per-fixture majority; repetitions are not independent repositories.

| Metric | Definition | Held-out target, not an observed result |
|---|---|---|
| Regression recall | Confirmed primary defects found / 5 known defects; inconclusive misses remain in the denominator | Majority detection on ≥4/5 fixtures |
| Finding precision | True regression findings / all regression findings; zero findings means undefined | ≥80%; publish raw numerator/denominator per run and pooled |
| Clean false positives | Clean fixtures with any false regression claim / 2 | 0/2 by majority |
| Intended changes | Fixtures correctly explained as intended / 2 | 2/2 by majority |
| Infrastructure handling | Correctly inconclusive infrastructure runs / 3 | 3/3; never claim a pass |
| Audited coverage | Mandatory checkpoints executed on both states / planned checkpoints in 9 runnable fixtures | ≥90%; report missing checkpoints |
| Replay success | Evidence packets independently reproduced / packets submitted | ≥90%, requiring `ceil(0.9 × n)` successes; show n |
| Added value | Paired majority detection and clean classification versus A | Improve detection on ≥1 regression fixture without worsening clean classification |

Also publish inconclusive fixture/run counts by cause, time to first usable evidence, total wall time, token cost, support minutes, and failed runs. Precision without findings and replay without packets cannot pass. Use raw denominators and descriptive uncertainty intervals where appropriate; five positive held-out fixtures cannot establish broad superiority. Cluster any resampling by fixture, not repetition. Freeze the protocol before W5. After exposure, the W5 suite becomes a regression suite. A failed gate or material capability change requires revision and a fresh holdout before claiming new unseen performance; ordinary replay does not. Fresh-holdout runs and costs are additional to the initial 120-run budget. Never tune on exposed cases and label them unseen.

## Timeline, pilot, and decision

W1 defines hypotheses and licenses. W2 builds ground truth and deterministic replay. W3–4 uses only development fixtures. W5 freezes and runs held-out evaluation; all gates must pass before external beta. W6 recruits **five opt-in developers across at least three repositories** for a supported private pilot. W7 prepares a release candidate. W8 replays the exposed W5 suite for regression checking, not new unseen evidence. Public beta is conditional on W5 gates, regression replay, and resolution of pilot blockers. Failed gates or material capability changes extend the schedule for a fresh holdout.

Pilot targets: ≥4/5 developers complete setup without live maintainer intervention and reach usable first evidence within 20 minutes; median support ≤30 minutes/developer; ≥3/5 voluntarily use it on a different change within 14 days, checked by the end of W8. For repeat use, report eligible opportunities separately: no second migration opportunity is not failure. If fewer than five have another relevant change, publish the actual denominator. Repeat use informs whether to refine or change direction; it is descriptive, not a hard release rejection without eligible opportunities. Log unsuccessful starts and withdrawals. Five friendly participants cannot establish retention or willingness to pay. Collect usefulness and missing-coverage feedback, including reasons for not returning.

## Resources, governance, and publication

The nominal eight-week plan assumes one maintainer plus AI, approximately 180 base hours at 22.5 hours/week. Adding 25% contingency gives 225 hours: allow **9–12 calendar weeks at 20–25 hours/week**, not a guaranteed eight-week deadline. Initial benchmark execution ceilings are **40 cumulative runner-hours and 4.8 million aggregate tokens** for 120 runs; these are caps, not measurements. W8 regression replay, retries, fresh holdouts, and extra experiments add runs and costs beyond those 120. Estimate cache/setup, authoring, review, and pilot support separately. Estimate money from recorded input/output/cache usage and current provider rates; do not invent a dollar result.

Project code uses MIT, copyright © 2026 Upgrade Witness contributors. Track fixture provenance, permission, license, attribution, and third-party notices separately; the project license does not relicense dependencies or copied examples. Prefer original synthetic fixtures. Publish only consented, sanitized reports; never upload private code, credentials, screenshots, or logs by default. Agent-provider data handling is separate from CLI telemetry policy.

Publish a bilingual benchmark card with scope, versions, all results, failures, methodology, replay instructions, and limitations. Until executed, display **“Not yet measured”**. Version exposed suites for regression replay; introduce fresh held-out cases when failed gates or material capability changes require new unseen evaluation.

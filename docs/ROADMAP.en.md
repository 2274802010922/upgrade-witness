> Initial design snapshot, 16 October 2026. See [README](../README.md) and [STATUS](STATUS.md) for current implementation status; this document is not an execution log.

# Upgrade Witness — end-to-end roadmap

[Tiếng Việt](ROADMAP.vi.md) · [Start here](../README.md) · Planning baseline: 16 October 2026

**Decision:** build a focused AI skill with a small execution toolkit that helps developers verify UI behavior after a dependency migration. Initial wedge: React + Vite + Tailwind CSS v3 → v4, npm, Chromium. MIT is the selected license. The product name is provisional; repository/package/domain availability has not been checked.

**Status:** this package contains plans and repository presentation assets. It contains no implemented verifier, released package, measured benchmark results, or published GitHub repository.

## 1. Product promise and customer

**English:** “Upgrade your dependencies. See what changed.”

**Vietnamese:** “Nâng cấp thư viện. Thấy rõ điều gì thay đổi.”

The first user is a maintainer of a small or medium React application who needs to upgrade Tailwind but has limited visual or end-to-end test coverage. The first job is to compare an agreed set of routes and component states before and after the upgrade and produce findings a reviewer can reproduce. A secondary user is the developer reviewing a dependency-update PR.

The report must separate observed differences, intended changes, suspected regressions, confirmed regressions, and untested areas. A green build alone is insufficient evidence of preserved behavior. A screenshot difference alone is insufficient evidence of a bug.

The differentiation hypothesis is the connection between migration-specific risks, targeted checks, and reviewable evidence. [Codemod AI](https://codemod.com/blog/npx-codemod-ai) already provides migration skills, tooling, and tests; [Playwright](https://playwright.dev/docs/test-snapshots) already provides image comparison. Integrate these capabilities and demonstrate added value through evaluation. Do not claim a new category or universal correctness.

## 2. Evidence and discovery

Start from the source-linked registry in [BENCHMARK](BENCHMARK.en.md). [Tailwind #20435](https://github.com/tailwindlabs/tailwindcss/issues/20435) reports incorrect migration of prop/data-attribute values; [#19555](https://github.com/tailwindlabs/tailwindcss/issues/19555) reports changed column behavior after configuration migration. These examples establish concrete failure modes, not market size.

In week 1, review 20 relevant public issues across at least three repositories, recording date, state, environment, workaround, maintainer response, and whether the report contains runnable evidence. Deduplicate repeated symptoms. Seek five consenting developer conversations through existing contacts or community-approved channels. Do not post generic promotional replies to issues.

Ask about the last upgrade: what broke, how it was detected, time spent, current checks, whether sharing a sanitized example is possible, and whether a proposed report would change a merge decision. Record observations separately from enthusiasm about the idea.

**Discovery gate:** at least three of five participants describe a concrete recent verification problem, and at least two offer an authorized project or fixture for a pilot. These are proposed thresholds. If unmet, extend discovery or narrow the problem before expanding implementation.

## 3. Scope decisions

| Included in the first release | Deferred until evidence justifies it |
|---|---|
| React/Vite, Tailwind v3→v4; one pinned npm-based fixture path | Next.js SSR/RSC, Vue, Angular, backend or database migration |
| Existing baseline/head revisions or a migration in an isolated working copy | Production writes, automatic merges, fleet-wide autonomous updates |
| Agreed routes and states, deterministic fixtures, DOM/CSS assertions, screenshots and interactions | Automatic discovery of every possible state, performance guarantees |
| Human-readable Markdown/HTML report and versioned JSON evidence | Hosted dashboard, accounts, billing, remote telemetry |
| A concise skill plus a small TypeScript CLI; one host agent validated first | MCP server, multiple agent integrations, pnpm/yarn matrix |
| Linux CI and Windows local execution tested independently | Claims that screenshots are comparable across operating systems |

Choose the first agent host at kickoff based on available tools; publish its exact tested version only after a real run. The architecture is host-neutral. New integrations need their own acceptance tests.

## 4. User journey and acceptance

1. The developer supplies a repository, the migration or two revisions, and key routes/states.
2. The skill checks the environment and defines the verification boundary. It records unsupported prerequisites and existing failures.
3. Capture and freeze expected behavior from the baseline. Keep assertions independent from the candidate implementation.
4. Run the migration in an isolated copy, or inspect the supplied candidate revision. Do not silently change the user's branch or discard uncommitted work.
5. Execute identical scenarios on both revisions in the same environment. Recheck unstable signals before classifying them.
6. Produce a report with code locations, before/after evidence, commands, expected-change references and explicit coverage gaps.
7. If fixing is requested, create the smallest patch and rerun the original frozen checks. Preserve the original failure evidence.

The first successful experience is a reproducible finding or a bounded “no regression observed” report. A setup failure must return an inconclusive result, not success. See [ARCHITECTURE](ARCHITECTURE.en.md) for contracts and exit behavior.

## 5. Eight-week execution plan

Assumption: one maintainer working with AI, 20–25 focused hours a week. Estimate: **180 hours plus 25% contingency = 225 hours**. The base plan targets eight weeks; using the full contingency extends the calendar to approximately 9–12 weeks at that pace. This is a planning allowance, not measured productivity. Weeks start at the actual kickoff; no calendar automation or deadline is created.

| Week / effort | Outcome and work | Exit condition | Dependency |
|---|---|---|---|
| W1 / 20h | Discovery, positioning, initial fixture inventory, name check; establish MIT and contribution rules | Discovery gate met; scope and top risks recorded | None |
| W2 / 24h | Vertical slice: one fixture, two revisions, one scenario, one finding with evidence | Another developer can replay it from a clean checkout | W1 |
| W3 / 24h | Baseline contract, scenario runner, deterministic capture, DOM/CSS and interaction checks | Known failures detected; a clean control stays clean; setup failure is inconclusive | W2 |
| W4 / 24h | Skill workflow, report renderer, redaction, run manifest, Windows/Linux checks, repair-and-recheck flow | Ten development fixtures exercised; package can be tested from its archive | W3 |
| W5 / 24h | Freeze skill/tool version; run held-out benchmark against the same agent without the skill | Gates in BENCHMARK met, raw denominators and failed runs retained | W4 |
| W6 / 20h | Five opt-in pilots across three repos; measure onboarding and report usefulness | Initial pilot results recorded; 14-day follow-up window starts | W5 |
| W7 / 20h | Fix pilot blockers, release-candidate packaging, real demo, EN/VI docs, release dry run | Fresh install/replay succeeds; all public claims trace to evidence | W6 |
| W8 / 24h | Finish pilot follow-up, replay the published W5 suite on the release candidate, publish beta when ready | Quality + pilot decision recorded; tagged artifact, docs and support process ready | W7 + follow-up |

The W8 replay checks regressions against the published suite; it is not fresh held-out evidence. A failed quality gate or material capability change requires a new held-out set and an explicit allowance for additional runs, review and cost beyond the initial 120-run experiment.

The maintainer owns priorities, review and release decisions. AI can help implement, inspect and test; its claims still need observable evidence. A reviewer who did not author the implementation should audit a sample of findings and the held-out scoring.

If a gate fails, do not compensate by adding more frameworks or a prettier launch. Fix the failing slice, document the decision and move the release date. Public beta is an outcome of the gates, not the eighth-week entitlement.

## 6. Implementation sequence and project board

Use the issue-sized tasks in [BACKLOG](BACKLOG.md). Suggested board columns: Backlog → Ready → In progress → Review → Verified → Done. Maintain at most one core implementation task and one documentation task in progress per maintainer.

Every issue carries a user outcome, supported scope, dependency IDs, acceptance evidence and an estimate. A task is done when the outcome is verified and its docs match reality. Organize milestones as `discovery`, `vertical-slice`, `alpha`, `evaluation`, `pilot`, and `public-beta`; create actual versions only when release artifacts exist.

Recommended branch workflow: small feature branch → focused checks → reviewer inspection → squash merge. New commit titles and bodies use Vietnamese, retaining technical names and Conventional Commit prefixes, for example `feat: thêm đối chiếu thuộc tính DOM`. International contributors may submit descriptions in English; the maintainer can help prepare Vietnamese commit text.

## 7. Quality, costs and failure boundaries

The benchmark protocol owns numerical gates to avoid contradictory targets between documents. Test fixture construction, runner failures, environment drift, false positives, intended changes, report evidence and replayability—not merely generated wording. Compare the same model, tools and task budget with and without the skill; keep test answers out of prompts and references.

Track engineering hours, model/API usage, CI minutes, artifact storage and pilot support separately. First run a three-fixture cost calibration; set a per-run cap from that measurement before running the full evaluation. No paid service is needed for the proposed basic architecture. Account quotas, model costs and hosting choices remain deployment-specific and must be checked at kickoff.

| Risk | Early response | Stop or narrow when |
|---|---|---|
| Visual noise overwhelms useful findings | Same OS/browser/fonts/data; stable assertions before pixel comparison | Clean controls fail repeatedly |
| Agent rewrites checks to match the bug | Freeze baseline checks; version expected-change decisions | A failed check is silently weakened |
| Private artifacts leak | Synthetic data, redaction before agent/export, local raw evidence excluded from Git | Export cannot be reviewed or sanitized |
| Setup effort exceeds benefit | One stack and package manager; explicit preflight diagnostics | Pilot support dominates useful verification |
| Tool adds little beyond baseline agent | Paired evaluation and ablation | No additional useful finding at acceptable cost |
| Maintainer workload grows | Narrow compatibility matrix, issue templates, release cadence | Support exceeds available weekly capacity |

Source repository install scripts execute code. Use a disposable, trusted environment for unfamiliar projects; a Git worktree is not a security sandbox. Local execution does not mean model-provider traffic stays local.

## 8. GitHub, MIT and contribution lifecycle

Use English as the repository entrypoint with a visible Vietnamese switch. Put the outcome, honest status and one real demonstration before architecture detail. [GITHUB](GITHUB.en.md) specifies README layout, demo storyboard, About/topics, social card, badges, launch and localization. The [repo preview](../README.md) is usable planning-stage copy today.

The selected [MIT license](../LICENSE) permits reuse and contributions under its terms. Keep the standard English license text; the Vietnamese docs explain it without replacing the legal text. Use `Copyright (c) 2026 Upgrade Witness contributors`. Keep third-party notices and fixture licenses; MIT for this project does not relicense dependencies or imported examples. See the [OSI MIT text](https://opensource.org/license/mit).

Welcome bug reports, minimal fixtures, documentation, translations and supported adapters. Start with contribution guides and bilingual issue/PR templates provided in the preview. A maintainer reviews incoming rights/provenance and test evidence. AI-assisted submissions are welcome when the submitter understands and has verified them; credit actual contributors.

Before launch: choose owner/repo identity, confirm name availability, move plan docs into the chosen repo structure, repair links, set up real CI, test packaged installation, configure a private vulnerability-reporting route, record three real demonstrations and complete a release dry run. No repository is created or publication performed by this planning package.

## 9. Release, first 30 days and later decisions

Publish an initial beta only after gates pass. Include exact supported versions, known limitations, reproducible case studies, checksums, installation/uninstallation instructions verified on a clean machine, and a changelog tied to real commits. Keep manual operation possible if an installer fails. Never show a passing badge or npm install command for an artifact that does not exist.

During the first 30 days, triage at a sustainable cadence, acknowledge reproducible issues, track setup success and repeat use from voluntary feedback, and ship focused bug fixes. Re-run relevant fixtures whenever captures, assertions, dependencies or prompt routing change. Version JSON reports and fixture manifests. Update EN and VI documentation together when user-visible behavior changes.

At day 30, decide whether to deepen Tailwind coverage, add pnpm, or validate a Next.js adapter. Add MCP only when users need a tool interface that the CLI cannot serve conveniently. Consider hosted collaboration only after repeated demand; it is outside this roadmap's implementation commitment.

**Success means developers can make better migration decisions with reproducible evidence.** Stars and downloads can help discoverability, but they do not replace that measure.

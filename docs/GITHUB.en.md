> Initial design snapshot, 16 October 2026. See [README](../README.md) and [STATUS](STATUS.md) for current implementation status; this document is not an execution log.

# GitHub presentation and launch plan

[Tiếng Việt](GITHUB.vi.md) · [Roadmap](ROADMAP.en.md) · [README preview](../README.md)

**Planning snapshot: 16 October 2026.** This is an editorial and release plan. The supplied README describes the project's actual planning stage; it does not claim that a working package or benchmark exists.

## 1. What to learn from REA

The reviewed [REA README](https://github.com/morluto/rea) moves from a clear promise and language links to setup, a sample agent request, workflow, capabilities, showcases, FAQ, contribution guidance, and license. The useful pattern is a short path from understanding the tool to inspecting evidence. This is an observation, not proof that its layout caused adoption.

Our proposed interpretation: one concrete migration problem, one reproducible example, and one clear next action. Use original copy, graphics, recordings, and fixtures. Avoid REA's name, logo, screenshots, and popularity as borrowed credibility.

## 2. Positioning and the first screen

Working repository name: **upgrade-witness**. Check name and package availability before reserving either; neither is claimed by this plan.

Recommended English description:

> Evidence for frontend upgrades. An AI skill and CLI for checking behavior after dependency migrations. Starting with React + Tailwind.

Recommended Vietnamese description:

> Bằng chứng cho mỗi lần nâng cấp frontend. Skill AI và CLI kiểm tra hành vi sau migration, bắt đầu với React + Tailwind.

The first screen should contain the language switch, project name, one-sentence promise, compact hero, explicit **Planning / Pre-release** status, and two links: **Read the roadmap** and **Contribute a migration case**. Preserve text outside the image so the promise remains readable on mobile and with assistive technology. The supplied hero is a concept illustration, not a product screenshot.

Use the established light Swiss/editorial direction: warm white, ink typography, restrained blue/lime accents, clear alignment, and generous spacing. Keep wording short enough to survive GitHub's narrow rendering. Evaluate at a phone width and in GitHub light/dark themes. Any light-background SVG should include its own background.

## 3. README page order

| Position | Reader's question | Content |
|---|---|---|
| 1 | What is this? | Promise, scope, project stage, language links |
| 2 | Why should I care? | A migration that builds but changes an important flow |
| 3 | Can I use it today? | Current/planned table; honest installation status |
| 4 | What would I ask? | One realistic agent prompt |
| 5 | What comes back? | Findings, evidence, rerun instructions, coverage gaps |
| 6 | Does it work? | A real case study once reproducible; methodology link |
| 7 | What is supported? | Small compatibility matrix with tested versions |
| 8 | How do I help? | Small tasks, contribution guide, MIT license |

At alpha, replace the planning CTA with an installation command verified from a fresh checkout. Link its exact release and prerequisites. Show one supported path first; move alternatives into installation documentation. Do not publish a guessed `npx` command before a package exists.

## 4. Demo storyboard: 55 seconds

Record only after the W2 slice reproduces an actual failure. Choose a project fixture that permits publication. Capture its commit, lockfiles, browser/runtime versions, and report artifacts.

| Time | Screen | Caption |
|---|---|---|
| 0–7 s | Original UI, then migrated UI | “The build passes. The menu changes.” |
| 7–15 s | User's agent request | “Check the flows we need to preserve.” |
| 15–28 s | Before/after at the same viewport and state | “One observable change, with evidence.” |
| 28–40 s | Finding linked to a DOM/CSS assertion and code diff | “Inspect what changed and rerun it.” |
| 40–49 s | Fix and repeated check | “The same check now passes.” |
| 49–55 s | Report with tested and untested areas | “Review the evidence. Keep the limits visible.” |

This is proposed narration, not a claim about an existing run. Replace every line with what the fixture demonstrates. Preserve waiting-time cuts as labeled edits. Provide a poster image, captions, text transcript, downloadable report, and reproduction steps. Avoid an endless animation above the README's first useful text.

## 5. Assets, About, and badges

Create an original 1280 × 640 social card with a solid background, project name, short promise, and a simple before → after → evidence motif. GitHub accepts PNG/JPG/GIF below 1 MB; its recommended best-display size is 1280 × 640, with 640 × 320 as the minimum recommendation. The README SVG and uploadable social image are separate assets. [GitHub social preview documentation](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/customizing-your-repositorys-social-media-preview)

Initial suggested topics: `agent-skills`, `ai-agents`, `developer-tools`, `tailwindcss`, `react`, `migration`, `playwright`, `visual-regression`. Add `cli` when runnable; add a client topic only after integration tests. Do not label the project `mcp` while MCP is deferred.

Use at most four informative badges. MIT is accurate when linked to the supplied license. Stage can be static. CI must link to a real workflow; release must link to a real tag. Omit downloads, coverage, compatibility, performance, and star badges until their claims can be verified. A benchmark number must link to raw results and a named baseline.

## 6. Target repository organization

This is the **future target tree**, not the inventory of this planning package:

```text
README.md / README.vi.md
LICENSE / CONTRIBUTING.md / CONTRIBUTING.vi.md
SECURITY.md / CODE_OF_CONDUCT.md / CHANGELOG.md
.github/ISSUE_TEMPLATE/ / pull_request_template.md / workflows/
skills/upgrade-witness/SKILL.md / references/
packages/cli/                 # core modules remain inside the single package
fixtures/ / benchmarks/ / tests/
docs/en/ / docs/vi/ / docs/cases/
assets/hero.svg / hero-vi.svg / social-card.png / demo-poster.png
```

Move the planning documents into `docs/en/` and `docs/vi/` when creating that repository. Rewrite the preview README's `../ROADMAP.*`, `../ARCHITECTURE.*`, and `../BENCHMARK.*` links to their new locations. Keep large generated browser artifacts outside Git history; publish selected redacted reports as release assets or case-study attachments.

## 7. Community and bilingual workflow

English is the primary landing page; Vietnamese remains a complete supported documentation path. Pair changed feature/status sections in the same PR, retain identical technical identifiers, and review commands once for both languages. Record translation review dates. If a page falls behind, disclose the outdated section and link to the current version.

Use issue forms for bug reproduction, migration fixtures, and feature proposals. The first contribution queue should include a small licensed fixture, one failing assertion, a report clarity improvement, and translation parity. New commit titles and bodies follow the user's Vietnamese convention. Contributor guidance must explain local checks once implementation exists. MIT is selected: use `Copyright (c) 2026 Upgrade Witness contributors`; retain applicable third-party notices separately.

## 8. Launch and release checklist

W1–2: prepare the repo story and invite a few relevant reviewers when authorized. W3–4: share a working alpha with reproducible instructions. W5: publish the benchmark protocol and all results, including failures. W6: start pilots and obtain permission for each case study. W7: prepare the release candidate and launch assets. W8: record the 14-day pilot reuse observation and publish beta only if the roadmap gates pass. Repeat-use results are descriptive, with their actual eligible denominator; having no second migration opportunity is not a failed launch gate.

- [ ] Fresh-checkout install and documented example succeed on the claimed platforms.
- [ ] EN/VI links, status, support matrix, license, and screenshots agree.
- [ ] Demo uses a real tagged run; no invented users, results, or testimonials.
- [ ] Release notes identify supported scope, limitations, and known regressions.
- [ ] Maintainer can triage reproducible issues and mark unsupported requests clearly.

Potential channels: an own GitHub Discussion, a technical migration write-up, and relevant React/Tailwind/agent communities whose rules allow project sharing. Tailor each post to an actual case; disclose authorship, avoid unsolicited issue advertising, and never purchase or exchange stars. Check each community's current rules before posting. This plan authorizes no external publication. Measure completed first runs, useful reports, accepted fixes, returning pilot users, and contribution quality; treat stars as a secondary discovery signal.

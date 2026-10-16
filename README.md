<p align="center"><a href="README.md">English</a> · <a href="README.vi.md">Tiếng Việt</a></p>

# Upgrade Witness

**Upgrade your dependencies. See what changed.**

An AI skill and local CLI that compare two committed frontend revisions and produce reviewable evidence. Start with **React + Vite, npm, Tailwind 3 → 4 and Chromium**.

![Original concept illustration: baseline, upgrade, evidence.](assets/hero.svg)

[![CI](https://github.com/2274802010922/upgrade-witness/actions/workflows/ci.yml/badge.svg)](https://github.com/2274802010922/upgrade-witness/actions/workflows/ci.yml)
**Early alpha** · [MIT](LICENSE) · [Roadmap](docs/ROADMAP.en.md) · [Contribute](CONTRIBUTING.md)

## Try a real before-and-after run

Prerequisites: Git, Node.js 24 and npm. The current development/runtime checks use Node 24.16.0. The fixture and browser are downloaded during setup; run only projects you trust.

```sh
git clone https://github.com/2274802010922/upgrade-witness.git
cd upgrade-witness
npm ci
npx playwright install chromium
npm run demo
```

On Linux, use `npx playwright install --with-deps chromium` if browser system libraries are missing.

The demo creates an isolated Git repository, commits original synthetic Tailwind 3 and Tailwind 4 states, runs the verifier and checks its result. It intentionally introduces two behavior changes: a lost column configuration and a changed `data-variant`. **A regression result is the expected demo outcome.** The wrapper exits successfully only when the expected findings are present. It prints the generated report location. See [the example and its provenance](examples/tailwind-migration/README.md).

This is a runnable example, not a benchmark of real-world accuracy or of the current Tailwind upgrade tool.

[See the actual demo report and observations](docs/cases/tailwind-demo.md).

[See the actual demo report and observations](docs/cases/tailwind-demo.md).

## Use it with your agent

The repository includes [the skill](skills/upgrade-witness/SKILL.md). Make the `skills/upgrade-witness` folder available through your agent's documented skill mechanism, or ask the agent to read that file. The host needs access to this checkout, a shell and the installed CLI dependencies. The skill does not install a model, register an MCP server or grant additional permissions.

> Use Upgrade Witness to verify this Tailwind migration. Preserve the navigation and layout behavior we specify. Show reproducible findings and anything you could not verify.

The host agent can help perform a requested migration using official tooling. The CLI itself **verifies committed revisions**; it does not automatically migrate or merge your application.

## Verify your project

Start from [the example configuration](examples/tailwind-migration/witness.config.json). Define your baseline expectations before inspecting candidate outcomes. Both revisions need an npm lockfile and a Vite-compatible development script.

```sh
npm run build
node dist/src/cli.js doctor
node dist/src/cli.js verify --repo /path/to/trusted-app --base BASE_COMMIT --head HEAD_COMMIT --config /path/to/witness.config.json --out .uw/my-first-run --trust-project
```

Use quotes for Windows paths with spaces. `--out` must not already exist. The explicit trust flag acknowledges that the selected repository's install, build and start scripts execute code.

The verifier resolves immutable SHAs, creates detached worktrees, captures baseline and candidate scenarios, and writes `report.html`, `report.md`, `report.json`, screenshots and hashed evidence. Uncommitted source changes are excluded and left untouched. [Configuration and commands](docs/USAGE.md)

| Exit | Meaning |
|---|---|
| `0` | Required checks completed with no regression or unresolved difference |
| `1` | At least one confirmed assertion regression; other unresolved findings may remain |
| `2` | Inconclusive, unstable baseline, infrastructure failure or visual difference requiring review |
| `64` | Invalid usage or configuration |

Read `complete` and coverage as well as the exit code. A screenshot difference is a review item, not automatic proof of a bug. An expected attribute change does not automatically approve every visual change on that page.

## What is implemented

- Detached revision workspaces, lockfile-based installs, readiness checks and bounded process lifetimes.
- Configured clicks, fills and keypresses; assertions for visibility, text, attributes and computed styles.
- Baseline stability checks, intended-change decisions, image comparison and evidence integrity hashes.
- Static HTML/Markdown/JSON reports with observations separated from causality.
- Original synthetic migration demo, focused tests and Windows/Linux CI configuration.

## Boundaries that matter

This alpha checks only the declared routes, states and assertions. It does not prove whole-application equivalence. Dynamic clocks, random values and live network data are not automatically frozen; use deterministic fixtures. There is no Next.js/SSR, monorepo, pnpm, MCP or production automation support yet.

Use synthetic non-sensitive data. Screenshots mask input fields and configured selectors; arbitrary page text/images and explicitly collected assertion values can still contain private data. Token redaction is best effort. Review all artifacts before sharing. The CLI has no model calls or uploads; your host agent may transmit context to its model provider. Worktrees are not security sandboxes, and deliberately detached server daemons may escape cleanup.

Independent held-out evaluation and external pilots are **not yet completed**. The [evaluation protocol](docs/BENCHMARK.en.md) contains targets, not measured performance claims. [Implementation status](docs/STATUS.md)

## Build with us

Contribute a minimal migration case, a clean control, an assertion, a report improvement or a translation. Read [CONTRIBUTING](CONTRIBUTING.md). Original fixtures, honest evidence and small PRs are especially useful.

[Architecture](docs/ARCHITECTURE.en.md) · [Bilingual backlog](docs/BACKLOG.md) · [GitHub presentation plan](docs/GITHUB.en.md) · [Security](SECURITY.md)

[MIT](LICENSE) · Copyright (c) 2026 Upgrade Witness contributors. Third-party material retains its applicable terms.

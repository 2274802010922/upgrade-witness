# Rust implementation update / Cập nhật triển khai Rust

[English README](../README.md) · [README tiếng Việt](../README.vi.md) · [Current status](STATUS.md)

Updated 17 October 2026. The first local alpha used a TypeScript CLI. After the request to make the whole project Rust, v0.2.0 replaces the tool's executable core and tests with Rust. The TypeScript prototype is kept only in Git history; it is not installed or used by the CLI. New commits preserve the existing history.

## Current architecture

```text
AI host ── reads the Markdown skill/references
                │ runs `uw verify`
Native Rust CLI: clap / serde / anyhow
 ├─ Git: detached worktrees and immutable SHAs
 ├─ std::process: npm CI/build/dev server for target app only
 ├─ Rust CDP driver: fresh Chrome contexts, actions, assertions and capture
 ├─ Rust image comparison: pixel difference threshold for review
 ├─ sha2: local artifact/config digests
 └─ serde_json + Rust templates: JSON, Markdown, self-contained HTML

Original React/Vite fixture — the subject app, deliberately Node/npm-based
```

The tool runtime, CLI entrypoint, config validation, Git/process lifecycle, browser automation, comparison, reporting and automated tests are Rust. CSS/HTML/assets and the Markdown skill/docs remain the content formats they represent. JavaScript source and npm lockfiles under `examples/` are target-app fixtures: this product must verify a frontend project, so keeping a React/Vite fixture provides a real local integration test. Runtime project code does not use Node, npm, TypeScript or Playwright to implement the verifier.

## Reasons and tradeoffs

The request for Rust supersedes the TypeScript implementation decision in the initial roadmap. `headless_chrome` drives the installed browser through Chrome DevTools Protocol in the Rust process. Users install the Rust toolchain, Git and Chrome/Chromium; Node/npm are needed only when the selected target app uses them. The runner does not download a browser. `doctor` proves it can launch the detected executable.

Browser-rendered DOM operations still use Chrome's DevTools interface and a small JavaScript expression inside the page to evaluate selectors/styles and cover sensitive regions. No JavaScript engine/runtime harness is packaged as the product. The target application is separate and executes its own build/dev scripts only after `--trust-project`.

The Rust screenshot comparator records a pixel as changed when any RGBA channel differs by more than 25/255, then applies `maxDiffPixelRatio`. This is intentionally identified as a small deterministic heuristic. It does not claim pixelmatch equivalence or general visual accuracy. Unknown navigation status is inconclusive. Live clocks, randomness, networks and fonts are not frozen automatically.

Screenshot inputs and explicitly collected assertion values may contain sensitive data. The runner hides automatic/custom mask targets before capture and paints black rectangles into PNG memory before saving. Keep synthetic data and inspect every report. Best-effort token filtering cannot identify every secret. The local runner does not upload evidence, but the host agent follows its model provider's data policy.

## Build and verify from source

Install a current stable Rust toolchain with the platform linker, then run:

```sh
cargo build --locked
cargo run --locked -- doctor
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo run --locked -- demo --out .uw/rust-demo
```

`doctor` launches/closes Chrome and checks Git. The browser integration tests require installed Chromium; Git worktree/npm-server tests also need Git/Node because they execute the sample target app. The CLI itself builds and starts natively. A demo run creates a disposable repository under a new, tool-owned output directory, records synthetic commits using the explicitly labeled demo identity and retains its report. Never point the demo at a user's project.

The old `scripts/demo.mjs`, `package.json`, TypeScript CLI and TS tests were removed from the current source tree. The npm-based fixture remains under `examples/tailwind-migration/`. v0.1.0-alpha.1 was an untagged Git implementation only; it was not published to npm.

## Quality gates that remain

The locally verified Windows sample is a positive control: 14 assertion checkpoints, 12 pass, two deliberately injected assertion regressions, no missing checkpoints; screenshot changes remain review findings (`complete:false`). Two additional browser tests cover collapsed zero-area elements and masks inside a top-layer modal. These tests validate known scenarios, not independent accuracy.

The original roadmap still calls for the 20-fixture held-out evaluation and five consenting external pilots with a 14-day follow-up before a public beta. They have not happened. Re-run Rust CI on both Ubuntu and Windows for the Rust commit; the earlier TypeScript workflow result does not validate this change. Record actual runs and replace no targets with invented results.

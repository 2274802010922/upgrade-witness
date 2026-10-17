# Changelog

## 0.2.0-alpha.1 — 2026-10-17

- Make the user-facing CLI, worktree manager, browser capture, comparator and reporters a native Rust application.
- Control local Chrome/Chromium with the Rust `headless_chrome` CDP driver; no Node/TypeScript runtime is used by the core CLI.
- Preserve repeated, same-origin baseline/candidate checks, full SHA identities, report manifests and the trusted-project boundary.
- Detect zero-area elements as not visible and hide screenshot mask targets before capturing; paint opaque mask rectangles into PNG memory before saving.
- Add Rust configuration, comparison, report, workspace, CLI and browser integration tests.
- Keep Node/npm limited to installing, building and serving the target React/Vite projects and synthetic fixtures.
- Add pinned Linux and Windows Rust CI for format, lint, tests, the synthetic demo, source packaging and native binaries; use the Actions check for the exact commit before distributing its artifacts.

This is an early alpha. The independent evaluation and external pilot in the roadmap have not been performed. The demo is an authored positive control, not a real-world accuracy claim.

## English

Version 0.1.0-alpha.1 was a short-lived TypeScript implementation recorded in Git history. It was not released as an npm package. Version 0.2.0-alpha.1 replaces that core with Rust; historical code is not the current product/runtime.

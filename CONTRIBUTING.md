# Contributing to Upgrade Witness

[Tiếng Việt](CONTRIBUTING.vi.md) · [Project overview](README.md) · [MIT](LICENSE)

Upgrade Witness v0.2.0-alpha.1 is a native Rust CLI with an AI skill. Useful contributions include minimal migration cases, test expectations, Rust implementation, and equivalent English/Vietnamese documentation.

## Set up and check a change

Install a current stable Rust toolchain, platform linker/build tools, Git and Chrome/Chromium. Node.js/npm are needed only for target frontend applications and the test/demo fixtures that exercise them. The core uses a Rust browser driver; no Playwright installation is required. See [USAGE](docs/USAGE.md) if browser detection needs an override.

```sh
cargo build --locked
cargo run --locked -- doctor
cargo fmt --all -- --check
cargo test --locked
```

The full test suite includes real browser and npm-served fixture tests. A compile-only result does not establish that those checks passed. Run `cargo run --locked -- demo` when changing the capture/comparison pipeline, workspace lifecycle or migration fixture. Review its report as well as the demo exit status. The demo intentionally produces assertion regressions.

Keep `Cargo.lock` in version control and include lockfile changes when updating dependencies. Use `cargo build --locked --release` to check a release build. Report which commands ran and which were skipped or blocked; do not advertise untested platforms or versions.

## Useful first contributions

- A small original React/Vite + npm example showing a change after Tailwind 3→4 migration.
- A clean control or an intended change with a clear behavioral requirement.
- A CSS-selector assertion or interaction distinguishing correct and incorrect behavior.
- A Rust runner/report fix supported by a reproducible failure.
- A missing limitation or matching English/Vietnamese improvement.

Discuss substantial scope changes before implementation. Small fixes can be proposed directly. The current alpha covers one npm-based app and Chromium; adding a framework or package manager requires its own fixtures and acceptance evidence.

## Bugs and fixtures

Use [the repository's issue forms](https://github.com/2274802010922/upgrade-witness/issues/new/choose). Include tool/browser versions, OS, before/after commits, target npm lockfiles, steps, expected behavior, actual behavior and minimal sanitized evidence. Record the exact command and whether `doctor` succeeded. Mark a suspected cause as a hypothesis.

Use synthetic data. Do not submit credentials, private source, cookies or private screenshots. Public source is not automatically available for relicensing: record source, author, license and retained notices for imported material. Prefer an original minimal example.

## Pull requests

Keep each PR focused on one outcome. Explain concrete before/after behavior and preserve the failing case. New expectations or image thresholds require a reason tied to intended behavior; never change the baseline just to hide a failure. Compare images with the Rust threshold semantics documented in [USAGE](docs/USAGE.md), not an assumption that pixelmatch yields identical results.

Update matching EN/VI sections together and retain identical identifiers. Flag translation help if needed. New commit titles and bodies use Vietnamese; Conventional Commit prefixes and technical names may remain, for example `fix: giữ nguyên data-variant khi nâng cấp`. English issues and PR discussion are welcome; the maintainer can help prepare commit wording. Do not rewrite old history only to translate it.

AI assistance is welcome. Disclose material AI assistance in the PR, understand the changes and verify their evidence. Credit actual contributors; do not invent co-authorship or checks.

## License and review

Contributions intentionally submitted for inclusion must be distributable under the project's [MIT license](LICENSE), while contributors retain their copyright. Submit only material you have the right to contribute. Third-party components retain their own terms. This project has no separate CLA or DCO signing workflow.

Keep discussion respectful and specific. Maintainers may close duplicate or out-of-scope work with a reason; volunteer review timing is not a service guarantee. For a security-sensitive finding, use [SECURITY.md](SECURITY.md) rather than public credentials or exploit details.

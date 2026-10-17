# Contributor instructions

- The executable is native Rust. Keep runtime code in `src/*.rs`; do not reintroduce a Node/TypeScript or Playwright runner. Node/npm are used only for target frontend apps and fixtures.
- Write new commit titles and bodies in Vietnamese. Conventional Commit prefixes and technical names may remain. Do not rewrite existing history only to translate it.
- Keep measured results separate from proposed targets. Update equivalent English/Vietnamese documentation together for user-visible changes.
- Run `cargo fmt --all -- --check` and `cargo test --locked` for implementation changes. Run `cargo run --locked -- doctor` when browser setup changes and `cargo run --locked -- demo` when the pipeline, workspace lifecycle or migration fixture changes. The full suite requires Git, Chromium and Node/npm for fixture execution.
- Preserve the original baseline contract. Do not alter assertions, masks or image tolerances merely to hide a regression. Rust screenshot comparison uses a per-channel threshold, not pixelmatch.
- Use original synthetic fixtures, preserve third-party notices, and keep hidden evaluation answers outside the evaluated agent's accessible resources.
- Do not commit `.uw`, `target`, credentials, private captures or target-app `node_modules`. Keep `Cargo.lock` committed.
- Local execution does not define the host agent's model-provider data policy. Keep report claims and privacy limitations accurate.

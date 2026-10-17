# Third-party components

Upgrade Witness's original Rust code, instructions, reports and original artwork are licensed under the repository's MIT License. Dependencies retain their own terms; the project MIT license does not replace them. The project does not copy REA source, artwork, or issue reproduction code.

Direct runtime dependencies are declared in `Cargo.toml` and pinned in `Cargo.lock`. Their package licenses are:

| Crate | Locked version | License recorded by the crate |
|---|---:|---|
| `headless_chrome` | 1.0.22 | MIT |
| `anyhow` | 1.0.104 | MIT OR Apache-2.0 |
| `base64` | 0.22.1 | MIT OR Apache-2.0 |
| `chrono` | 0.4.45 | MIT OR Apache-2.0 |
| `clap` | 4.6.7 | MIT OR Apache-2.0 |
| `image` | 0.25.9 | MIT OR Apache-2.0 |
| `regex` | 1.13.1 | MIT OR Apache-2.0 |
| `serde`, `serde_json` | 1.0.229, 1.0.154 | MIT OR Apache-2.0 |
| `sha2` | 0.10.9 | MIT OR Apache-2.0 |
| `tempfile` | 3.27.0 | MIT OR Apache-2.0 |
| `url` | 2.5.8 | MIT OR Apache-2.0 |
| `uuid` | 1.26.1 | Apache-2.0 OR MIT |

Transitive licenses are recorded in the locked dependency registry metadata and must be preserved when redistributing their code. The CLI uses the installed system Chrome/Chromium; it does not bundle the browser binary. The React/Vite/Tailwind fixture includes original synthetic source and separate npm lockfiles. npm dependencies keep the licenses in their respective packages.

The license text follows the [Open Source Initiative MIT license](https://opensource.org/license/mit). Research links are references, not endorsements.

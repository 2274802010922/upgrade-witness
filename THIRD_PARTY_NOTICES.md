# Third-party components

Upgrade Witness's original code, documentation and concept artwork use MIT.
Dependencies are distributed by their own packages under their own licenses;
the project license does not replace them. No REA source, artwork or issue
reproduction code is copied into this repository.

Direct runtime dependencies and their installed package metadata at this alpha:

| Package | Version | License |
|---|---|---|
| playwright | 1.64.1 | Apache-2.0 |
| playwright-core (transitive browser driver) | 1.64.1 | Apache-2.0 |
| pixelmatch | 8.0.0 | ISC |
| pngjs | 7.0.0 | MIT |
| zod | 4.6.7 | MIT |

Keep each dependency's license and notices when redistributing its material.
Browser binaries are downloaded separately by Playwright and carry their own
notices. Development and example dependencies are recorded in their respective
lockfiles; preserve their package-provided licenses. The example React/Vite app
is original synthetic test code; its known changes are documented openly.

The MIT license text follows the [Open Source Initiative text](https://opensource.org/license/mit).
Research links in the design documents are references, not endorsements.

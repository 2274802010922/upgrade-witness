# Contributing to Upgrade Witness

[Tiếng Việt](CONTRIBUTING.vi.md) · [Project overview](README.md) · [MIT](LICENSE)

Upgrade Witness is an early alpha. Contributions to minimal migration cases, test expectations, implementation and English/Vietnamese documentation are welcome. Set up with npm ci and npx playwright install chromium. Run npm run typecheck and npm test; run npm run demo for pipeline or fixture changes. See README for platform prerequisites.

## Useful first contributions

- A small React/Vite + npm example showing a behavior change after Tailwind 3→4 migration.
- A clean upgrade that should produce no regression, or an intentionally changed behavior with a clear requirement.
- A DOM/CSS assertion or interaction that distinguishes correct and incorrect behavior.
- A clarification, missing limitation, or equivalent English/Vietnamese correction.

Discuss large changes before implementing them so the maintainer can confirm scope. A small documentation fix can be proposed directly.

## Report a bug or contribute a fixture

Use the provided issue forms after these files are placed in the real repository. Include exact versions, operating system, before/after revisions, npm lockfiles, steps, expected behavior, observed behavior, and the smallest relevant sanitized evidence. Say which commands you actually ran. Mark hypotheses as hypotheses. An issue is useful even when the cause is unknown.

Use synthetic data. Do not submit credentials, private source, session cookies, or screenshots of private user data. Public availability of another repository is not permission to relicense its contents; include source, author, applicable license and retained notices for imported material. Prefer an original minimal example.

## Pull requests and verification

Keep a PR focused on one outcome. Explain the concrete before/after behavior and link an issue where relevant. Preserve the original failing case and report the checks executed, skipped, or blocked. Changing an expected screenshot or assertion needs a reason tied to intended behavior; do not silently accept a new baseline to make a failure disappear.

Update matching EN/VI sections together. Keep technical identifiers identical. If you cannot translate a change accurately, flag the affected section for maintainer help rather than claiming parity.

New commit titles and bodies use Vietnamese. Conventional Commit prefixes and technical names may stay in English, for example `fix: giữ nguyên data-variant khi nâng cấp`. English issue and PR discussion is welcome; maintainers can help with commit wording. Do not rewrite existing history just to translate it.

AI assistance is welcome. State material AI assistance in the PR, understand the submitted changes, and verify the evidence. Credit actual contributors; do not invent co-authorship or claim checks an agent did not run.

## License and review

By intentionally submitting a contribution for inclusion, you agree it can be distributed under this project's [MIT license](LICENSE), while retaining your copyright. Submit only material you have the right to contribute. Third-party components retain their own applicable terms. No separate CLA or DCO signing workflow is required by this starter policy.

Be respectful and specific. Discuss code and evidence without personal attacks. Maintainers may close duplicate or out-of-scope work with a reason. Volunteer review timing is not a service guarantee.

For a security-sensitive finding, avoid public exploit details or credentials. Use the private reporting route in [SECURITY.md](SECURITY.md).

# Original synthetic Tailwind migration case

This fixture is original project code, covered by the repository's MIT license.
It is a deliberately incomplete React + Vite migration from Tailwind CSS 3.4.17
to 4.3.3. React, React DOM and Vite stay pinned at 19.3.1, 19.3.1 and 8.3.4.
Each side has an npm lockfile. Exact transitive versions are recorded there.

The example illustrates two migration risk categories discussed in
[Tailwind #19555](https://github.com/tailwindlabs/tailwindcss/issues/19555)
(lost layout configuration) and
[Tailwind #20435](https://github.com/tailwindlabs/tailwindcss/issues/20435)
(changed component/data-attribute values). It contains no copied issue code.
**It does not run or evaluate the current Tailwind migrator, reproduce either
issue exactly, or establish a benchmark result.** Its altered head is authored
on purpose so the verification pipeline has a known positive control.

## Contract and expected observations

`witness.config.json` states the base app's intended behavior without diagnosing
the head. At 1280px, notes use three columns and the action keeps its `outline`
variant. On mobile, the menu opens with the expected label and notes use one
column. Baseline assertions must all pass before a failure can be a regression.

The head omits the custom `columns-dashboard` theme mapping and changes the
action's `data-variant` to `outline-hidden`. The expected assertion regressions
are `desktop-overview/column-count` (`3` → `auto`) and
`desktop-overview/primary-variant` (`outline` → `outline-hidden`). The mobile
menu assertions remain passing. Screenshots also change; those differences
must remain separate review observations instead of being called extra bugs.

## Run

After the repository's install and Chromium setup, run `npm run demo` from its
root. The demo creates a new isolated Git repository below `.uw/demo-project/`,
records two commits under the explicit **Upgrade Witness Synthetic Demo**
identity, and invokes the built CLI with `--trust-project`. Both commit titles
are Vietnamese. No remote is configured and no files are removed from the
user's project.

The demo succeeds only if the verifier exits `1`, every assertion ran, and both
expected assertion regressions are present. Screenshot review can leave
`complete: false` even when all assertions ran. It prints the actual findings
and report path. The wrapper exits `0` for this expected result so CI can test
the positive control; the verifier itself still returns `1`. Each run writes a
new directory and retains the report for inspection.

This is a public demo fixture. Keep it separate from held-out benchmark cases
and do not use its known answers when evaluating independent detection.

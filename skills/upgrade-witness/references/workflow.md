# Running a review

## Prerequisites and scope

Locate a built checkout of Upgrade Witness or a verified local installation.
Use its `doctor` command and help output to inspect prerequisites. The current
runner targets one npm-based React/Vite app, a committed package-lock, and
Chromium. A skill-only install is insufficient. Match any compatibility claim
to the release being used; do not assume Next.js, pnpm, monorepos or MCP support.

The CLI creates isolated copies of committed Git revisions. Uncommitted work
is not part of a SHA. Preserve the user's changes, select explicit revisions,
and prepare a separate candidate commit when the authorized work requires one.
Repository commit conventions still apply. Do not create fabricated coauthors.

## Define the contract

Store the JSON contract outside the compared project if convenient. `schemaVersion`
is `1`. `project.startScript` is an existing npm script, usually `dev`;
`project.buildScript` is optional. Scenarios have an `id`, relative `route`,
viewport, actions and assertions. Supported action types are `click`, `fill`
and `press`; `fill` and `press` require `value`.

Assertions use `id`, `selector`, `kind` and `expected`. Kinds are `visible`
(boolean), `text` (string), `attribute` (string plus `name`), and `style`
(computed CSS string plus `name`). Use a single, stable target per assertion.
Express relevant user behavior, not the answer from a known faulty candidate.

```json
{
  "schemaVersion": 1,
  "project": { "startScript": "dev", "buildScript": "build" },
  "scenarios": [{
    "id": "mobile-menu",
    "route": "/",
    "viewport": { "width": 390, "height": 844 },
    "actions": [{ "type": "click", "selector": "[data-testid=menu-toggle]" }],
    "assertions": [{
      "id": "menu-open",
      "selector": "[data-testid=menu-toggle]",
      "kind": "attribute", "name": "aria-expanded", "expected": "true"
    }],
    "screenshot": true
  }],
  "intendedChanges": []
}
```

Mask only genuinely unstable regions with `mask` selectors; describe what their
exclusion hides. An optional `maxDiffPixelRatio` controls screenshot sensitivity,
not the truth of semantic assertions. An intended change names `scenarioId`,
`assertionId`, its new `expected` value and a meaningful `reason`. Get its intent
from the user's request or review context, not from the fact that head differs.

## Execute

From the CLI checkout, use the actual paths and full selected commit hashes:

```text
node dist/src/cli.js doctor
node dist/src/cli.js verify --repo <project-path> --base <base-sha> --head <head-sha> --config <contract-json> --out <new-report-directory> --trust-project
```

The angle-bracket values are placeholders; replace them and quote paths for the
current shell. Select a new output directory. Review project scripts before
executing an unfamiliar project. Existing task authorization may be sufficient;
the flag does not create new permission for unrelated code or external actions.

Compare on the same platform and browser revision. Use stable local data and
assets. Distinguish report files generated locally from data sent by the agent
to its model provider. Do not publish artifacts without checking the requested
scope and their contents.

## Interpret and follow up

Read `report.json`, not just terminal output. Exit `0` means all required checks
completed without regressions or unresolved differences. Exit `1` means at
least one confirmed assertion regression; examine `complete` because other
checks may still be missing. Exit `2` means unresolved or incomplete verification
without a confirmed regression. Exit `64` indicates invalid CLI usage.

Show the affected scenario, expected/observed values and evidence for each
material finding. A baseline assertion failure is a baseline problem. A visual
diff requires review. Unexpected intended-change results still need attention.
Do not imply the tool proved the root cause or covered unlisted routes.

When a failure is environmental, resolve the concrete prerequisite and rerun
once it changes. Stop repetitive runs with unchanged inputs. If the user requests
a fix, preserve the original report, patch the candidate and rerun the same
contract against the new candidate SHA. Keep known synthetic demo answers out
of independent benchmark evaluation.

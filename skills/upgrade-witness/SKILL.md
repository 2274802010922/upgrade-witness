---
name: upgrade-witness
description: Verify observable frontend behavior across dependency upgrades using pinned Git revisions, explicit UI expectations and replayable evidence. Use for React/Vite Tailwind migrations or reviewing an existing upgrade diff; the CLI compares revisions and does not perform migrations.
---

# Upgrade Witness

Use this skill to make a dependency upgrade reviewable: state what behavior
should survive, run the same checks against both revisions, and explain findings
with linked evidence and coverage limits.

Read [references/workflow.md](references/workflow.md) when preparing a contract,
running the CLI, or interpreting an incomplete result. This skill requires the
native Rust `uw` binary; copying the skill folder does not install the runner or
Chromium.

## Choose the work from the request

- For an existing upgrade, verify its base and head revisions. Do not run another
  migrator or change dependencies merely to investigate the diff.
- If the user asks to perform the migration, preserve the original revision and
  use the official migration guidance for the requested versions. Run the
  appropriate migrator in a separate worktree when useful, inspect the patch,
  then verify the resulting revision. The CLI itself does not migrate code.
- Use the available `doctor` and `verify` commands. Check `--help` from the
  installed build rather than assuming planned commands exist.

## Preserve the verification boundary

Choose assertions from the app's current behavior and the user's expectations
before inspecting the candidate's differences. Use stable selectors and
meaningful interactions. Record intended changes with their reason; do not
rewrite expectations merely to make a failed candidate pass.

Run trusted project code only within the user's authorized task. `--trust-project`
acknowledges that dependency installation, build scripts and app startup execute
code; it is not a sandbox. Keep unknown third-party examples out of an ordinary
verification run until their execution is authorized.

Compare both revisions in the same browser/OS environment. A failed baseline,
missing assertion, startup failure or unfinished scenario limits the result.
Do not convert missing evidence into a successful check or present an image
difference alone as a confirmed regression.

## Deliver the result

Lead with observed behavior and the exact checks exercised. Link the report and
relevant before/after artifacts. Separate regression, intended change,
needs-review, baseline-issue and inconclusive findings. Attribute suspected
causes only as hypotheses unless further inspection confirms them.

If asked to fix a regression, make a focused patch and rerun the same contract.
Describe any changes to coverage. Report both process exit status and
`complete`; a regression can coexist with missing checks. Neither a green build
nor an empty findings list proves complete behavioral equivalence.

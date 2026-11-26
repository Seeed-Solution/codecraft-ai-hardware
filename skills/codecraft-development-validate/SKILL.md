---
name: codecraft-development-validate
description: Validate repository contracts, generated indexes, native builds and evidence.
---

# Validate

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md` for workspace and task routing.
When installed elsewhere, resolve links below from the matching Skill path in
that workspace. If the workspace is unavailable, ask for its location and
continue independent work that does not depend on it.

For generated applications, use `verify-project .` and task-appropriate host
checks; use `build-project .` when a build is required. Preserve deliberate
lock review and do not run the reference repository gate for application-only work.

For repository contributions, select checks using the [root validation policy](../../AGENTS.md).
Run `maintenance generate --check`, focused checks and `validate --static`. If firmware and
native build inputs/behavior are unchanged, skip native builds and toolchain
setup. Otherwise validate affected Targets, examples and variants, including
native project builds when export contents or build behavior changes. No-option
`validate` runs the complete software gate without device writes; it does not
filter by Git changes and is not the default delivery check. Inspect every
status and exit code. Report checks omitted by scope as skipped. Record device
observations only through `maintenance record-evidence` with real revision, source digest,
toolchain, operator and retained sanitized log. A simulator or build never
establishes physical behavior.

Commands use `codecraft <command>` from the selected repository or application root.
Read that workspace's AGENTS.md before acting.

The documentation gate requires paired README files and rejects Chinese copies of other engineering documents. Check both repository and exported-project navigation. Preserve original third-party evidence and fail on stale Skill inventories instead of silently accepting new fingerprints.

For changed standalone packages, follow docs/development/skill-releases.md. Check the exact candidate, review its digest and versions, then explicitly apply it; application reruns isolated client conformance. A passing candidate does not publish, authorize a live API task or replace the repository checks selected by the root validation policy.

For focused host checks, use `codecraft maintenance test --path <owner>`; use `maintenance test --list` to inspect collected IDs. Read docs/development/testing.md before moving tests. Preserve full scenario coverage; do not keep old forwarding wrappers that execute the same suite twice. Skips and fixture/collection failures must remain visible.

For ESP-IDF source selection and BSP dependencies, follow [native composition rules](../../docs/development/application-workflow.md#native-composition).

For assembly selection, export and revision-specific evidence, follow [hardware variant rules](../../docs/development/application-workflow.md#hardware-variants-and-evidence).

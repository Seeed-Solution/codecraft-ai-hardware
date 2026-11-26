---
name: codecraft-development-build
description: Build native firmware and run checks without flashing or publishing.
---

# Build

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md` for workspace and task routing.
When installed elsewhere, resolve links below from the matching Skill path in
that workspace. If the workspace is unavailable, ask for its location and
continue independent work that does not depend on it.

First select the workspace. For a generated application, follow its AGENTS.md
and use `codecraft build-project .` after reviewing and explicitly locking
intended application/toolchain changes. Do not run repository-wide static or
reference Target validation for ordinary application edits.

For reference repository contributions, apply the [root validation policy](../../AGENTS.md).
Use this workflow for requested builds or changes affecting firmware/native build
inputs or behavior. Otherwise use static/host checks without native builds or
toolchain setup. Run `validate --static`, then `validate --target <id>` for
affected Targets; narrow by `--example` / `--variant` where applicable. Use
`--projects` when native export contents or build behavior changes. No-option
`validate` runs the complete software gate; reserve it for an explicit full-build
request or native changes affecting every Target. A missing required prerequisite
is not_run, never passed. No build command flashes or publishes. Diagnose failures
before retrying.

Commands use `codecraft <command>` from the selected repository or application root.
Read that workspace's AGENTS.md before acting.

In a generated application, use `codecraft build-project .`; explicitly accept reviewed application/toolchain changes with `codecraft lock-project .`. Do not use reference-Target validation to bypass an application's lock.

For requested factory image delivery, keep the native toolchain activated and
use `firmware plan --project <application> --output <new-plan.json>`,
`firmware merge --plan <plan.json> --output <new-release-directory>`, then
`firmware verify <release-directory>`. Reference builds select --target,
--example and, when applicable, --variant instead of --project. The plan requires
a current successful build receipt. Never relock or rebuild implicitly after
drift. Current support is unsigned, unencrypted ESP-IDF factory BIN; other
platforms/formats return not_supported. Output includes firmware.bin and its
manifest; source package behavior is unchanged. These commands do not flash or
publish. See docs/development/firmware-images.md in the reference repository.

For ESP-IDF source selection and BSP dependencies, follow [native composition rules](../../docs/development/application-workflow.md#native-composition).

For assembly selection, export and revision-specific evidence, follow [hardware variant rules](../../docs/development/application-workflow.md#hardware-variants-and-evidence).

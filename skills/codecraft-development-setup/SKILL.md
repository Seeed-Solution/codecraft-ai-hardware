---
name: codecraft-development-setup
description: Install and activate the exact toolchain declared by a hardware Target.
---

# Setup

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md` for workspace and task routing.
When installed elsewhere, resolve links below from the matching Skill path in
that workspace. If the workspace is unavailable, ask for its location and
continue independent work that does not depend on it.

Read target.json for the exact toolchain and its source. Check `codecraft --help`: when
`doctor` is available, run `codecraft doctor --target <id>` from the selected
repository/application before changing the environment. Reuse an already matching
installation and activate it in the shell that will build. The published v0.5.0
CLI lacks doctor; use the native version checks below without requiring an upgrade.
Doctor only inspects the active environment; it never installs tools or updates locks.
For ESP-IDF, activate the declared installation and verify idf.py --version. Linux source validation needs Python 3 and dtc. Arduino needs the declared FQBN and exact core/library versions. Preserve other toolchains; install missing prerequisites within authorization and report unresolved dependencies.

Commands use `codecraft <command>` from the selected repository or application root.
Read that workspace's AGENTS.md before acting.

For an explicit SenseCraft CLI setup request or a task needing SenseCraft platform operations, follow [external tool setup](../../docs/development/application-workflow.md). Use the official published npm package and the matching bundled Skill. Verify the actual executable version and offline help; keep authentication and API implementation upstream. Native toolchain setup does not require SenseCraft login or a cloud call.

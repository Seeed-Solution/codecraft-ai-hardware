---
name: codecraft-development-debug
description: Diagnose hardware build, configuration and on-device failures.
---

# Debug

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md` for workspace and task routing.
When installed elsewhere, resolve links below from the matching Skill path in
that workspace. If the workspace is unavailable, ask for its location and
continue independent work that does not depend on it.

Begin with the existing failure output and selected project. Do not restart device
selection, reinstall a matching toolchain or run the complete repository gate
without evidence that it is relevant. Report the failing stage, first actionable
error, log path and the focused next action.

Reproduce the smallest failing check and classify environment, configuration, code, hardware, or source conflict. Read the declared Target and device context. Change one variable at a time and rerun the failed check. Missing tools are not unsupported hardware. Do not flash experimental fixes without the current session authorization.

Commands use `codecraft <command>` from the selected repository or application root.
Use explicit paths when the terminal is in another directory.

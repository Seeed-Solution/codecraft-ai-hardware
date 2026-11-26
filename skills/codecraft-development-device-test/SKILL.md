---
name: codecraft-development-device-test
description: Perform explicitly authorized device writes and record actual physical observations.
---

# Device Test

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md` for workspace and task routing.
When installed elsewhere, resolve links below from the matching Skill path in
that workspace. If the workspace is unavailable, ask for its location and
continue independent work that does not depend on it.

Require explicit session authorization before any device write, and verify board/port or UF2 drive identity. Existing authorization remains valid within scope. Do not require flash readback and never erase the full chip without an explicit request. Use the native deployment workflow, then observe the claimed physical behavior and recovery. ESP32 flashing does not update separate co-processors. File copy alone does not prove boot success. Keep original sanitized logs and actual observations in the application during private development. Reference repository evidence contributions use `codecraft maintenance record-evidence` and tests/devices/<device>/ under tests/AGENTS.md; maintenance commands are unavailable in application exports. Acceptance procedures remain in the Device AGENTS contract.

Commands use `codecraft <command>` from the selected repository or application root.
Use explicit paths when the terminal is in another directory.

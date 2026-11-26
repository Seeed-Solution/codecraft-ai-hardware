---
name: codecraft-development-discover
description: Discover registered devices, Targets and native interfaces with explicit unknowns.
---

# Discover

Start from the user's repository or application workspace, not this Skill's
installation directory. Read that workspace's AGENTS.md and
`docs/development/application-workflow.md` for workspace and task routing.
When installed elsewhere, resolve links below from the matching Skill path in
that workspace. If the workspace is unavailable, ask for its location and
continue independent work that does not depend on it.

For an existing application, reuse its project.yaml selection unless the user
requests a change. For a new application, preserve a device or stack the user has
already chosen. Compare registered example inputs and native interfaces against
the requested functions; explain tradeoffs using sourced facts, not guessed
compatibility. Ask only for constraints that materially change the selection.

Use `discover --interface <id>` (repeatable) and optional `--platform <id>`. Show matching candidates, limitations and unknowns. No matches means no registered implementation, not proof about all hardware. Load `context --target <id>` before development; do not guess missing pins or board revisions.

Commands use `codecraft <command>` from the selected repository or application root.
Use explicit paths when the terminal is in another directory.

For cloud inference, select an explicit standalone Skill by its documented operation. In the reference repository, docs/skill-inventory.json is generated from package contracts and release records. Read the selected SKILL.md and contract.json. Do not assume missing network, credential references, privacy decisions or resource bounds are satisfied. Peripheral interfaces remain hardware metadata.

# CodeCraft Agent Skills

Five skills govern AI-assisted development in this repository. Each lives in
its own directory as a `SKILL.md` with YAML frontmatter (`name`,
`description`).

| Skill | Covers | Does not cover |
|---|---|---|
| `codecraft-develop` | Workflow: routing, hardware-fact priority, BSP/app boundary, delivery format | building, flashing |
| `codecraft-setup` | Toolchain bootstrap (ESP-IDF per-board versions, Linux deps) | coding |
| `codecraft-build` | Building and validating firmware/examples per board | flashing, publishing |
| `codecraft-device-test` | Flashing and on-device verification | building |
| `codecraft-debug` | Diagnosing build, flash, and on-device failures | feature work |

Install all five into your agent environment before developing (see the root
`AGENTS.md`). Use only the skills matching the task at hand.

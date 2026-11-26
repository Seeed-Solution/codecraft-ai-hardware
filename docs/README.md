<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# Documentation

Only README files have Chinese counterparts. Engineering guidance, rules, Skills and reports are maintained in English.

## Current rules and implementation

- [Shared coding-agent setup](development/agent-client-setup.md) — one entry for Codex, Claude Code, Cursor, ZCode, WorkBuddy, Muse and other agents.
- [Repository and contribution contract](../AGENTS.md)
- [Architecture](architecture/overview.md)
- [CodeCraft hardware application platform architecture](architecture/codecraft-hardware-app-platform.md)
- [Standalone Skill contracts and release ownership](architecture/skill-contracts.md)
- [Simulation scope decision](architecture/simulation-scope.md)
- [Private project workspaces and briefs](development/project-workspaces.md)
- [Repository maintenance entry](../CONTRIBUTION.md)
- [External SenseCraft CLI setup and routing](development/sensecraft-cli.md)
- [Local CLI boundaries and Skill contract consolidation](development/cli-boundaries-review.md) — current command ownership and completed implementation acceptance.
- [Device documents and shared references](development/device-documents.md)
- [AI development guide](development/ai-guide.md)
- [Standalone Skill release workflow](development/skill-releases.md)
- [Host test ownership and focused execution](development/testing.md)
- [CI impact selection and aggregate result](development/ci.md)
- [ESP-IDF BSP composition and layout](development/native-compositions.md)
- [Hardware variants and native build selection](development/hardware-variants.md)
- [Firmware image delivery and TODOs](development/firmware-images.md)
- [Generated Skill inventory](skill-inventory.json)
- [Portable application workflow](development/application-workflow.md)
- [Generated native interface inventory](inventory.md) and [JSON bindings](interface-matrix.json)
- [Generated vendor index](vendor-sources.md)
- [Device acceptance](on-device-validation.md)

## Planning and open work

- [Agent client compatibility design](development/agent-client-compatibility-design.md) — proposed minimal Skill/document access improvements; no architecture or schema changes.
- [Local hardware architecture plan (adapted from Feishu)](architecture/hardware-architecture-plan.md)
- [Repository tools migration TODO](development/tools-rust-migration-todo.md)
- [Architecture alignment TODO](development/architecture-alignment-todo.md)

- [V0.2 implementation TODOs](development/v0.2-todo.md) — design alignment, completed work and pending acceptance.
- [reCamera SDK optimization TODOs](development/recamera-sdk-todo.md) — Skill/tool ownership, completed checks and future Device/Target integration; see [SDK maintenance](development/recamera-sdk-maintenance.md) for reference and candidate commands.

- [Architecture requirements](development/codecraft-hardware-distilled.md) — requirements for all three content owners.
- [Optimization plan and TODOs](development/optimization-plan.md) — remaining tooling, external acceptance and governance work.

Start from the [device directory](../devices/AGENTS.md). The [Device document guide](development/device-documents.md) defines the single maintained source, Markdown fields, shared references and export checks. Native layout is described in [architecture](architecture/overview.md).

- [Rust CLI installation, updates and Python boundaries](development/rust-cli.md)

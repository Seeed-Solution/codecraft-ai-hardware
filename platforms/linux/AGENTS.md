# linux platform rules

The [root contract](../../AGENTS.md) governs. Device facts belong in devices/<device>/AGENTS.md and its reference files.

These rules apply to devices/<device>/targets/. Each Device owns its overlays, native demos and Target configuration there. This Platform owns shared Linux validation and runtime adaptation.

Preserve overlays and native demos. GPIO uses chip/line addressing; clean up claims on exit. OS image/kernel are unknown until observed; dtc compilation is separate from device operation.

Every Target must declare examples, toolchain provenance, native interface references, and dependencies in target.json. Supported operations and their limits are in platform.json. Run `codecraft validate --target linux/<device>`. Never infer device verification from a build.

# AI development guide

Read both applicable parts of the root AGENTS.md. Use `codecraft context --target <platform>/<device>` to collect device facts, native implementations, dependencies, native interfaces and unknowns. Read the selected files for the task; a returned path is not evidence that its contents were checked. Consult primary vendor sources when hardware facts need confirmation. Keep the five required development Skills available and load the matching workflows.

## Application and BSP boundary

Reusable board drivers belong in devices/<device>/targets/bsp; ESP-IDF components shared by multiple boards belong in platforms/esp-idf/components. Pages, state machines, animations and application tasks belong in examples or a private user project under projects/<name> (or an explicitly selected external directory). See [project workspaces and briefs](project-workspaces.md). All repository-maintained tests, fixtures and mocks live under tests/, grouped by source owner. Follow [test ownership and execution](testing.md). Keep testable logic independent of ESP-IDF/LVGL.

For ESP-IDF builds, follow [native composition and layout](native-compositions.md). Match the example BSP source/requirement lists, managed manifest, Target input/dependency/interface scopes and regenerated lock. Keep the native registration in the BSP root; public include/ and private src/ have distinct visibility.

## ESP-IDF runtime invariants

- LVGL access outside the LVGL task must hold the BSP's LVGL lock.
- Button callbacks must remain non-blocking. Move audio, storage and other slow operations into worker tasks.
- Stop every task, timer, callback and event handler that can touch a screen before deleting it.
- Keep the default partition table minimal; validate custom partition and persistent-data changes explicitly.

## Adding a new board

1. Author devices/<device>/AGENTS.md following [Device documents and shared references][device-documents]. Keep hardware notes, sourced fact tables, acceptance procedures and real references in this single maintained entry.
2. Preserve the exact source facts, explicit unknowns and physical revision limits. Do not copy metadata, acceptance procedures or shared rules into parallel documents. Target manifests remain schema_version 3.
3. Add devices/<device>/targets/target.json with native examples, toolchain provenance, native interface references, common/per-example inputs and dependencies. Keep the BSP, examples and board configuration beside that manifest. Read platforms/<stack>/AGENTS.md; the shared adapter and build helpers stay in platforms/<stack>/. Preserve native BSP/build conventions. Lock ESP-IDF dependencies; Arduino needs exact CLI/core/library versions and an FQBN. A new stack must declare every operation in platform.json and pass adapter contract tests.
4. Keep peripheral interfaces in Target metadata. Define a cloud contract only in the consuming standalone Skill; include inputs/outputs, configuration, resources, errors, dependencies and integration tests. Put tests and test fixtures under tests/ according to the testing guide; retain assets and simulator implementations with their source owner. A simulator must declare its coverage; unsupported operations must explain why.
5. Run `generate`, `generate --check`, `validate --static`, then `validate --target <id> --projects`. These generate and check device indexes, Skill inventory and CI matrices. Never manually register a board in multiple lists. Missing build prerequisites prevent a passing gate; experimental support must state the reason.
6. Collect revision-specific device evidence only under authorization. Store observations, records and logs under tests/devices/<device>/ according to tests/AGENTS.md. Keep build, generated-project, simulator and physical results separate. Select any broader checks using the root validation policy below.

## Validation scope

Follow [the root validation policy](../../AGENTS.md#required-validation-and-delivery)
before setting up toolchains or choosing commands. Documentation/rule changes
and host-only changes with unchanged firmware and native build inputs need
static/host checks, not firmware compilation. Native changes require builds
only for affected Targets, examples and variants. A commit, push or merge does
not require rebuilding unchanged, already validated native inputs. The no-option
`codecraft validate` command remains a full-build operation, not the
default delivery check.

The [CI planner](ci.md) compares committed inputs on both sides of a change and
selects native Target jobs while retaining static/host checks. Review its reasons
and the fixed CI result job; it does not inspect uncommitted local work or turn
skipped builds into fresh verification evidence.

## Delivery format

Report static/knowledge/host checks, builds per Target with actual toolchain, clean generated-project checks, and on-device verification separately. Include failures, skips, unsupported operations and unperformed checks with their reasons. A build is not hardware validation. Use the root contract and PR template for exact fields.

Requested commits use an English imperative Conventional Commit subject. Preserve unrelated work, credentials stay out of source, and generated firmware stays untracked. A commit does not authorize push, PR or merge. Follow [.github/CONTRIBUTING.md](../../.github/CONTRIBUTING.md).

For sourced assembly/revision differences, read [hardware variants](hardware-variants.md). Device owns the physical facts; Target owns native selections. Preserve explicit variant identity through application export, locks and evidence. Do not derive a physical revision from a build default.

[device-documents]: device-documents.md

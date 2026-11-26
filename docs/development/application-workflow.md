# Portable application workflow

This guide applies to native projects and their exported workflow Skills. The
root AGENTS.md defines the current task scope. Repository contribution and release
maintenance additionally follow the reference repository's root routing table;
they are not application build prerequisites.

## Workspace entry and routing

Use the same workflow in Codex, Claude Code, Cursor, ZCode, WorkBuddy, Muse or
another agent with file and terminal access. The agent's Skill installation
directory is not a repository root. Start from the workspace explicitly supplied
by the user or the open project. From a subdirectory, find the containing
project.yaml or reference checkout; if more than one workspace is plausible,
clarify the location before writes. Do not scan unrelated personal directories.

- An application root contains project.yaml and codecraft.lock. Read its AGENTS.md,
  reuse its target/example/variant, and preserve its custom rules and current work.
- A reference checkout has the device/platform catalog and development Skills.
  Read its AGENTS.md, discover a Target if needed, and create a private application
  when the request is application development rather than a contribution.
- An empty destination is not a reference checkout. Locate the user's reference
  checkout before discovery or export; do not create files in an installed Skill.

For an installed Skill, a link such as ../../docs/development/application-workflow.md
means the corresponding document under the selected workspace. Read the matching
workspace file instead of assuming the client's installation directory has that
relative layout. Use the selected workspace's guidance and available CLI commands;
do not copy board knowledge or overwrite local rules to repair a reference.

Use codecraft-development-develop as the general application entry. Read the
matching specialized Skill only when the current task needs it; retain the root
contract's required Skill availability. Skill references are instructions to read
guidance, not an assumption that every client has the same Skill invocation tool.

Choose the next action from the user's current state: ideate/discover for an
unselected application, develop for implementation, setup for missing environment,
build for compilation, debug for a failure, validate for scoped checks and
device-test for authorized physical work. Skip already settled decisions.

Run context with the matching task and selection, for example:

```sh
codecraft --root /path/to/workspace context --target esp-idf/reterminal-e1001 --example epd_hello --task develop
codecraft verify-project /path/to/application
codecraft build-project /path/to/application
```

The last command is for an authorized build after the normal source/environment
review and lock steps below; these examples are not a mandatory sequence.
Inspect `codecraft --help` when a command is unavailable and follow the existing
native guidance for that installed release. A missing terminal limits execution,
not planning; report what actually ran. A remote shell does not establish access
to local USB/serial devices. Changing clients does not require recreating a project,
refreshing a lock or repeating authorization that is still valid within scope.

## Selection and context

Run `codecraft discover`, then `context --target <stack>/<device>`.
Use `--task ideate`, develop, setup, build, debug, device-test, discover or validate
to choose guidance. Every returned source is in the same dependency graph used
by exports, locks and evidence. Read the selected Device AGENTS, Target manifest,
Platform AGENTS and cited sources. Pins, revisions, resource bounds and accessory
compatibility remain unknown without the named source or observation.

## Project intent and source locks

Use `new --target <id> --example <name> --output <new-directory>` to create a
workspace; within the reference checkout use projects/<name>. `--brief` accepts
JSON with name, purpose, target_users, scenarios and bom. BOM entries contain
item, quantity and source; unknown quantity/source use null. project.yaml owns
the brief; relocking refreshes the generated AGENTS region and preserves custom
rules outside it. Keep application work outside the reference implementations.

Creator project profiles and locks use schema 4. Use the standalone Rust CLI; Python and Cargo are not required for discovery, creation or source verification. Select optional cloud clients with
`--skills <selection.json>`, containing explicit Skill IDs, configuration,
environment decisions/reference names and resource limits. No credential values
belong in metadata. Read each selected package's SKILL.md and contract.json;
selection never authorizes a service call. When executing from an exported project,
apply its reviewed Skill configuration through the documented client flags; stored
environment decisions do not establish current authorization or availability.
Native-only projects carry no cloud
registry or client. Original import provenance remains immutable.

## Build and validation scope

Check `codecraft --help` for `doctor`; when available, run
`codecraft doctor --target <stack>/<device>` from the selected root before setup.
It reports the active toolchain and declared requirements without building or
changing locks. The published v0.5.0 CLI does not include doctor; use the Setup
Skill native checks with that version. Reuse a matching installation and
activate the toolchain declared by the Target in the build shell. ESP-IDF accepts the declared
release with optional `-dirty` local edits; the lock still fingerprints those
edits. Prereleases and post-release commits are different SDK versions, not
equivalent release matches. For a deliberate experiment with another release,
change the private application's Target requirement and review/relock its
environment; this does not establish support for the reference Target. An ESP32 Arduino
project uses its Arduino core/FQBN/library pins, not ESP-IDF commands. Use
`verify-project .` to inspect source integrity. After reviewed application or
environment changes, use `lock-project . --toolchain`, then `build-project .` for an
authorized build. Never refresh locks or rebuild implicitly to hide drift.

Unchanged firmware and native build inputs/behavior need static/host checks,
not firmware compilation or toolchain installation. Repository contributors use
`validate --static`; affected native changes use scoped Target/example/variant
validation. Reference validation is unavailable in exported applications;
`build-project` enforces the application lock. The no-option repository
`validate` command runs all native builds and is reserved for explicit full-build
requests or changes affecting all Targets.

## Native composition

Keep board-specific BSP logic, examples and configuration directly under
devices/<device>/targets/. Keep shared stack helpers and components under
platforms/<stack>/. Application pages, state machines and tasks belong in the
application; do not copy shared components or introduce a cross-stack ABI.

For ESP-IDF, public BSP headers belong in include/ and private code in src/.
Each example's bsp.cmake, managed dependencies and Target source/dependency
scope must agree. Preserve native component registration and required system
components. Regenerate dependency locks through the declared native tools only
when those inputs actually change. Hold LVGL locks, keep callbacks non-blocking,
and stop every task/timer/callback before freeing its screen or hardware state.

## Hardware variants and evidence

Select only sourced Target variants. Preserve the selected variant through the
profile, native configuration, lock and build cases. A default is not panel
detection or proof of PCB revision. Changing assemblies requires the corresponding
sourced native inputs, explicit review and relocking. One variant's evidence
cannot certify another. Linux overlay compilation does not establish OS/kernel
or physical behavior; Arduino board-manager profiles do not prove revision
compatibility.

## Firmware images and device operations

After an authorized successful build, use `firmware plan --project <path>
--output <new-plan.json>`, `firmware merge --plan <plan.json> --output <new-dir>`,
then `firmware verify <dir>`. Keep the matching native toolchain active. Planning
requires current successful build receipts and native layout metadata; offsets
must not be guessed. Current merging supports unsigned, unencrypted ESP-IDF
factory BIN. Other formats/platforms report not_supported. Source `package`
creates a source archive, not a firmware release.

No build, merge, package or verification command flashes or publishes. Device
writes require explicit authorization and an identified board/port. Full-chip
erase requires a specific request. Diagnose failures before retrying. Report
metadata, host tests, native compilation, file transfer and physical observations
separately; preserve original logs, revision, timestamps and source digests.

## External platform operations

SenseCraft authentication, managed devices, telemetry, Hub applications, uploads
and cloud deployment use the official external @seeed-studio/sensecraft-cli and
its bundled Skill. Install it only for an authorized task needing it; verify the
selected executable version/help. It is not required for native development.
Read the installed official Skill for setup and platform operations. CodeCraft
inference clients use their own Device-authenticated APIs; platform credentials
are not interchangeable. Installing or exporting a Skill grants no API permission.

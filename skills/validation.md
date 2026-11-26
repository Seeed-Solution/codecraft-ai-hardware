# Validation workflow

This document describes reference repository validation. Generated applications
use their own AGENTS.md, `verify-project` and scoped `build-project`; ordinary
application development does not require this repository-wide gate.

Choose checks using the [root validation policy](../AGENTS.md#required-validation-and-delivery).
Run `maintenance generate --check`, focused checks and `validate --static`. Unchanged firmware
and native build inputs/behavior require no native compilation or toolchain
setup. Native changes require affected Target/example/variant builds, plus native
project builds when export contents or build behavior changes. The no-option
`validate` command runs the complete software gate without Git diff filtering;
use it for an explicit full-build request or native changes affecting every
Target. The CLI is `codecraft`. Report scope-based omissions as skipped
and missing required prerequisites as not_run.

GitHub CI keeps the static/host gate on every run and uses the public
`codecraft maintenance ci plan` command to select affected native Targets. Its
fixed aggregate job rejects failed or unexpectedly skipped selected jobs and
reports intentional omissions explicitly. Read [CI selection](../docs/development/ci.md)
for committed-revision baselines and conservative fallback rules; local diff
review and the no-option full gate retain the scope rules above.

Device parsing, reference ownership, generation and export invariants follow
[Device documents and shared references](../docs/development/device-documents.md).
Checks reject malformed tables, duplicate or missing references, self-certifying
facts and unowned metadata. They preserve authored bytes and physical procedures;
source graphs and exported locks both hash complete files. Retained evidence lives
under tests/devices/<device>/ with source identity and log hashes intact.

Tests reject source contradictions, missing/cyclic dependencies, omitted cross-layer resources, empty/incorrect metadata, wrong adapter identity/case coverage and stale evidence. Source checks cover canonical metadata formatting, Python syntax, retained licensing/attribution, local links, README language pairs, forbidden engineering-document translations and private-key material. Native peripheral interfaces and cloud Skill contracts are tested at their respective owners. Physical writes are excluded. Missing tools remain not_run; unsupported simulations remain not_supported.

The static gate verifies complete Skill release inventories and CLI contracts,
helper synchronization and the 16-case offline HTTP/SSE/media protocol suite.
The seven onboarding/authentication cases and six realtime cases verify isolated
credentials, native host encoding, paced WebSocket audio and same-key recovery.
Each client
runs independently against a loopback fixture. Direct-client conformance covers
all eight operations, local/archived media, approved defaults and overrides,
input file bounds, SSE completion/finish status, image integrity/dimensions,
nonzero failures and retained partial output.
No real credential or paid API call is used. Tests reject extra installed files,
ignore unpinned bytecode and preserve pinned binary assets through exports/locks.

Shared contract tests live in tests/tools/cli/skill_maintenance.rs. They
check separate semantic/policy/API/package versions, restrictive schemas,
explicit default permission, unknown prerequisites/resources and source drift.
Per-client cases live under tests/skills/<skill-id>/. Domain tests prohibit
hardware Catalog imports from Skill checks, verify application dispatch uses its
explicit path, and exercise native-only plus selected-Skill exports for every
Target/variant. All eight workflow contexts work in isolated exports; their
source graph includes the selected Platform and portable guidance. Native-only
runs reject cloud imports. Synthetic policy tests are not a second live service
or measured billing evidence.

Standalone release tests live in tests/tools/cli/skill_maintenance.rs. They verify identical candidate bytes, explicit version changes, real isolated candidate/client conformance, source/candidate drift rejection, transactional rollback and retained recovery backups. Public-entry tests check JSON, typed errors and exit codes, source-locked exports and explicit relocking. Reference validation rejects application roots; both the application build command and direct adapter builds check the current source lock and actual toolchain. Clean export checks preserve the requested example across every selected variant, and retained evidence must cover all native cases for that selection, including overlays. README/Skill/CI commands use codecraft; repository-only release maintenance is not required by exported applications.

Test ownership and focused commands are specified in [the host testing guide](../docs/development/testing.md). The common runner recursively discovers owner-grouped modules under tests/, runs both plain functions and unittest cases, and exposes `codecraft maintenance test --list` / `test --path <owner>`. Each protocol method and native case has an owner-qualified identity and runs directly through this runner.

ESP-IDF composition tests validate each example's selected BSP sources, local components, managed manifests and Target/export closure. Native builds also compare actual BSP source and requirement lists with the declared composition. Settle native dependency locks before final source fingerprinting; preserve explicit system requirements and board configuration when narrowing components. See [native composition rules](../docs/development/native-compositions.md). Component counts alone are not measurements of firmware size, build time or runtime resources.

Variant checks cover Device/Target identity, selected source closure, project/lock provenance, adapter case coverage and actual CMake/compile definitions. Physical support is aggregated per variant; unknown revision compatibility cannot promote support, and one assembly cannot certify another. Full Target builds include all declared variants. See [hardware variants](../docs/development/hardware-variants.md).

Firmware tests cover successful-build receipts, stale/failed build rejection,
layout bounds, chip/configuration/security checks, immutable merge inputs,
boot-header transformations, image/padding hashes and publication failure.
Clean ESP-IDF export checks also plan, merge and verify a factory BIN through
the same pipeline. A completed manifest marks a release; firmware verification
checks offline integrity and layout, not authenticity or physical operation.

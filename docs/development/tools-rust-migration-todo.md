# Repository tools migration TODO

Started 2026-11-26. Scope: replace every maintained Python implementation under
`tools/` with Rust, validate its behavior, then remove the old implementation.
Do not retain two production implementations or move obsolete orchestration to
another directory merely to satisfy the file count.

## Ownership and parallel work

Three implementation workstreams run concurrently; the primary agent reviews
interfaces, source ownership, compatibility, tests and final integration.

- Evidence and generation: source-bound evidence, offline validation, generated
  indexes and Device document checks.
- Checks and CI: static validation, host-test execution, Git change planning and
  aggregate job results.
- Skill maintenance: semantic contracts, package candidate/check/review/apply,
  reCamera reference production and release provenance.
- Primary integration: command dispatch, shared dependency closure, migration
  regressions, removal of duplicate catalog/project/runtime implementations,
  documentation and complete diff review.

## Acceptance checklist

- [x] Inventory Python tools and assign three independent implementation owners.
- [x] Implement Rust maintenance commands without invoking Python tool modules.
- [x] Reuse one creator catalog, graph, project and native implementation.
- [x] Preserve generation determinism and authored Device bytes.
- [x] Bind new evidence to current sources, selection, toolchain and CLI identity;
  keep historical records and logs unchanged and never promote stale evidence.
- [x] Preserve candidate digest review, drift rejection, offline isolated client
  conformance and immutable original package provenance.
- [x] Port meaningful regression coverage before deleting tests of obsolete APIs.
- [x] Retain independent Python client/native-host tests under tests/ where their
  subjects still use Python; a Rust test runner may invoke their external runtime.
- [x] Remove tools/**/*.py and all live references/imports of removed tools.
- [x] Update resources, workflow commands, docs and generated views together.
- [x] Pass Rust formatting, Clippy, Rust integration tests, static/host validation,
  generation drift and cross-platform compile checks.
- [x] Review native impact; run native builds only if actual commands, inputs or
  dependencies change. Report physical validation separately.

## Compatibility and limits

Keep the public `codecraft` entry. Repository maintenance is not copied into
creator exports. SDK Python tools, Python applications and standalone cloud
clients are outside the tools implementation migration. Their dependencies must
remain explicit; do not claim the complete development ecosystem is Python-free.

The common Python client helper currently stored under tools/ must acquire one
real standalone-package owner without altering released package bytes merely to
move ownership. Historical schemas remain attributable records, not a second
executable project implementation. Unknown or incompatible evidence stays explicit.

CLI binaries rebuild only when implementation/dependencies/compiler/target inputs
change. Device, application and documentation edits do not rebuild the tool.
No device writes, cloud execution or publication are part of this task.

Platform metadata changes to schema 3 when removing its Python adapter/runtime
closure. Re-export applications carrying platform schema 2 and review/copy their
application changes; do not reinterpret old locks. Package candidates similarly
require re-preparation under candidate schema 3. Hardware inputs and declared
SDK versions are unchanged.

## Final validation and review

Validated on 2026-11-26 with Rust 1.89.0 on macOS arm64 and the isolated Python
3.14.6 / pytest 8.3.5 host-client test environment:

- 82 Rust tests passed: transaction/timeout (3), authoring (14), host checks (11),
  CI (18), evidence/generation (12), migration/native metadata (12), Skill
  maintenance (8) and SDK AST reference generation (4). No failed/ignored cases.
- 137 static/host checks passed, including the black-box cloud candidate release
  test with its complete registered offline client conformance.
- Rustfmt, Clippy with warnings denied and diff whitespace checks passed.
- Linux x86_64 musl and Windows x86_64 MSVC all-target compile checks passed.
  Windows runtime and its PowerShell installer were not executed locally.
- Generation reports no drift. The generated CI matrix changes only JSON key
  ordering, not its Target values or row order.
- Official reCamera SDK source at its pinned commit produced 64 modules,
  1,857 symbols and 68 pages identical to the retained outputs; no SDK execution.
- tools/ contains zero Python source files; 24 old implementation files were
  removed. Obsolete platform adapters and tests importing the deleted backend
  were replaced, not moved into another production directory.
- No native input, SDK version, board setting or native compiler command changed.
  Native firmware builds and the complete native gate were skipped. Host fixtures
  and export tests cover all three stacks and all registered examples.
- No device was flashed or physically verified; no live cloud service was called.
  Package bytes, release inventories, import provenance and historical Device
  evidence/logs were verified unchanged. No commit, push or publication in this task.

Architectural review fixed revision-mixing in physical evidence, planned-skip CI
exit handling, ambiguous operation source declarations, missing main/BSP compiled
components, stale cached fixture text, strict command arguments and subprocess
bounds. POSIX full-client conformance belongs in the host gate; portable Rust
binary tests do not silently skip it or require its platform-specific fixtures.

The test runner now follows pytest fixture semantics instead of retaining the
former custom Python runner. Platform schema 3 and release candidate schema 3
are explicit compatibility boundaries; historical inputs are never silently
reinterpreted as current application locks or approved candidates.

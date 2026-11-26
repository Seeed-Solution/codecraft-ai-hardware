# Architecture alignment TODO

Baseline: Feishu V0.2 revision 1566 and local implementation commit f537718.
Started 2026-11-25. The [local architecture plan](../architecture/hardware-architecture-plan.md)
is the maintained adaptation; Feishu is the attributed historical design source.
This ledger tracks the current alignment batch, while [V0.2 history](v0.2-todo.md)
retains historical results. No past result or evidence is rewritten.

## First batch: documentation alignment

- [x] A01: Convert the source design to maintained English Markdown, preserving
  source/version attribution and all major topic areas; remove login parameters.
- [x] A02: Replace the old whiteboard with editable Mermaid, showing project workflows,
  actual ownership, external services and a separate unimplemented roadmap.
- [x] A03: Correct the implementation overview's directory tree and command routing.
- [x] A04: Separate creator schema 4 from audit schema 3 and explicitly describe
  the unimplemented project evidence promotion in the Device guide.
- [x] A05: Link the local plan and this ledger from both documentation README views.
- [x] A06: Pass generation drift, link/command/language checks and validate --static;
  inspect the final diff. No native input changes: native/full gates are skipped.

## Next implementation: evidence continuity

- [x] A07: Define schema 4 evidence ownership and immutable receipt/CLI identity
  binding, including source digest, Target, variant, toolchain and expected cases.
- [x] A08: Implement project evidence registration/read/promotion and Context status.
  Reject stale source/CLI/toolchain identity, wrong variants, incomplete cases,
  tampered logs and failed/unrun observations; preserve historical records.
- [x] A09: Add positive/negative host tests for all three stacks and export paths.
  Missing evidence must remain unknown; build evidence must never imply physical
  verification. Native builds are required only if native behavior changes.

## Constraint review

- [ ] A10: Review fixed Device sections against actual parser consumers. Permit
  omission only where absence has an explicit meaning, distinguishing not
  applicable from unknown. Update the shared parser, guides and tests
  together; do not weaken sourced-fact or acceptance requirements by prose alone.
- [ ] A11: Resolve asset ownership: keep one authoritative declaration, reference
  actual media guidance and avoid mandatory empty Assets sections/directories.
  Verify sources/rights before adding or redistributing product imagery.

## Independent product roadmap

- [ ] A12: Select and implement one real user-project simulator scenario; document
  production-code reuse, unsupported behavior and separate simulation evidence.
- [ ] A13: Add a real MQTT Skill when a service contract is available; validate
  offline with synthetic credentials. Local/edge capability work requires an
  attributable model and actual executable binding.
- [ ] A14: Complete revision-specific physical acceptance and authorized external
  platform E2E separately. Assign actual owners and record observed results.

Open tasks are not acceptance claims or placeholders to implement merely
for diagram completeness. No firmware, device write or cloud service call is
needed for the first documentation batch. Feishu synchronization is a separate
future publication action; maintain the local Markdown first.

## First-batch verification

On 2026-11-25, `codecraft validate --static` passed all 309 static/host
checks using the existing Rust binary and Python 3.14.6 audit environment.
`codecraft maintenance generate --check` reported no drift. All 100 local
Markdown link destinations across the six changed documents resolved; code
fences, README navigation parity and exclusion of login query parameters were
checked. The Mermaid source was inspected; a separate rendered-diagram test
was not run. `git diff --check` passed.

No board/native input, compiler command or toolchain declaration changed. Native
builds and the complete gate were skipped for documentation-only scope. No
physical verification, cloud API execution or Feishu write was performed.
At the time of that first batch, A07-A14 remained open.

## Presentation alignment, 2026-11-26

The architecture diagram now emphasizes hardware knowledge, stack support,
Skills, project development and validation. Tool language and installation
details remain in the tooling guide, rather than defining the logical layers.
This is a documentation presentation change; implementation ownership and the
pending evidence work are unchanged.

Validation for this presentation update: all 309 static/host checks passed;
generation reported no drift, local links resolved and diff whitespace checks
passed. No native builds, complete gate, device operations or diagram rendering
were run for this documentation-only change.

## Tooling migration update, 2026-11-26

A07-A09 are implemented by the shared maintenance evidence module and its
regressions; integrated validation is tracked in the
[tools migration TODO](tools-rust-migration-todo.md). Earlier dated reports above
describe their original state. A10-A14 remain independent follow-up work. There
is one Device parser and one project/catalog implementation after migration.

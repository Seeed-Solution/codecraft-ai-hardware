# reCamera SDK ownership and optimization TODOs

Baseline: upstream `35bb7a1e28ed376c84816881535fcde043bea822`.
Implementation branch: `feat/v0.5/recamera-sdk-workflow`.
This ledger separates the current standalone SDK workflow from future native
Device/Target support. It does not establish new hardware compatibility.

## Accepted ownership

| Content | Maintained owner | Installed/exported form |
| --- | --- | --- |
| SDK implementation and official builder | Official reCamera SDK repository | Explicitly pinned upstream builder files; no local replacement runtime |
| Agent task selection and SDK-specific workflow | `skills/codecraft-development-recamera-pysdk/` | One self-contained Skill with task-specific references and execution helpers |
| API reference producer and editorial source notes | `tools/skill_packages/recamera/` | Generated API pages and a separate source/hash lock in the Skill |
| Package fingerprints and candidate review | `tools/skill_packages/recamera/` | Complete reviewed package bytes; release records stay outside their own package |
| Host acceptance | `tests/skills/codecraft-development-recamera-pysdk/` and `tests/tools/skill_packages/recamera/` | Offline fixtures and isolated package tests |
| Future board facts/native implementation | Device entry and its Target, once sourced and implemented | Source-bound context/export; no placeholder registration |

The installed Skill must not import repository tooling, a hardware Catalog or a
sibling Skill. API references and the small official builder remain bundled for
offline use. Moving maintenance tools does not change SDK/runtime versions,
manifest semantics, native firmware, dependency admission or device permissions.
Original import hashes in `docs/recamera-skill-import.json` remain immutable.

## First implementation batch

- [x] **RC-01 — Create the contribution branch.** Start from the fetched upstream
  main using the existing `feat/<version>/<goal>` rule; preserve the starting tree.
- [x] **RC-02 — Make the Agent entry task-oriented.** Keep scope, routing and
  essential boundaries in SKILL.md; let the existing focused references own
  detailed contracts. Preserve source-only, offline-package and authorized-device
  modes without making every task load the entire SDK guide.
- [x] **RC-03 — Separate reference maintenance from installed execution.** Move
  the generator and editorial notes into the Tool owner, expose maintenance through
  the public CLI, and regenerate changed views from that producer. Keep SDK pins
  and original upstream reference material attributable.
- [x] **RC-04 — Add development-package candidate review.** The existing cloud
  release registry requires cloud contracts and cannot honestly cover this SDK
  workflow. Add explicit development-Skill selection without inventing a cloud
  contract. Require complete inventories, version review, isolated tests, drift
  rejection and an exact reviewed digest before apply.
- [x] **RC-05 — Verify installed behavior and maintenance boundaries.** Cover
  deterministic packaging without app execution, missing/wrong inputs, payload
  tampering, builder/reference drift and candidate/source drift. Run the full
  static/host gate; native inputs are unchanged, so native builds are out of scope.
- [x] **RC-06 — Reconcile documentation and provenance.** Update bilingual
  navigation, maintenance commands and the import explanation together. Preserve
  all original import bytes and distinguish current results from historical ones.

## Later integration, with explicit prerequisites

- [ ] **RC-07 — Establish the Device/Target slice.** Obtain authoritative reCamera
  Pro revision/runtime facts and define actual acceptance cases. Add a Device
  entry, schema-3 Target and runnable native example together. Unknown board
  mappings remain unknown; the existing SDK package is not BSP verification.
- [ ] **RC-08 — Select a real software-stack adapter.** Evaluate the actual SDK
  app build/runtime against the Linux adapter before choosing reuse or a new
  stack. Native operations belong to that Platform; shared repository tools must
  not branch on a board/chip name to implement them. Complete dependency closure,
  locked project export and adapter tests are required before registration.
- [ ] **RC-09 — Consolidate execution tools with the SDK maintainer.** Review
  which app validation/conversion/delivery helpers should be released upstream.
  Consume an actual versioned upstream artifact before removing local execution
  helpers. Do not turn an unavailable upstream release into an install-time
  network requirement or keep two competing SDK implementations.
- [ ] **RC-10 — Verify the actual device workflow.** With an identified board,
  runtime and authorized access, exercise the final package's installation,
  lifecycle and requested behavior. Record physical observations separately from
  offline checks; no current optimization item grants device-write authority.

RC-07 through RC-10 are future integration work, not prerequisites for installing
the current offline SDK Skill. They must not be marked complete from scaffolding,
host tests or copied vendor descriptions.

## Branch convention findings

The repository already requires `feat/<version>/<goal>` from an agreed base in
[CONTRIBUTING](../../.github/CONTRIBUTING.md#pull-requests). The earlier prefix-only
renames left some historical branches without the version component. A complete
creation procedure remains separate governance work. This SDK change follows
the existing rule and does not rename other branches or change merge policy.

- [ ] **GIT-01 — Complete branch creation guidance.** Keep CONTRIBUTING as the
  owner, route agents there before creating a branch, define version/base
  selection and same-task reuse, and document local/fork tracking. Add a small
  PR branch-name check only after the accepted naming/exception rules are clear.
  Reconcile historical names separately without rewriting unrelated history.

## Validation record

Local validation on 2026-11-20 used macOS / Python 3.14.6 and the existing
isolated host-test environment (Pillow 13.0.0 and websockets 15.0.1).

| Check | Actual result |
| --- | --- |
| Installed SDK Skill conformance | passed: all 5 cases, including non-executing deterministic packaging and tamper rejection |
| Reference production | passed: all 3 host cases; the pinned official SDK check covers 64 modules, 1,857 symbols and 68 pages |
| SDK candidate maintenance | passed: all 9 cases, including source/candidate drift, immutable import identity, incomplete checks and rollback |
| Candidate/check/review/apply | passed: package 1.1.0 records all 105 installed files; application commands, builder/API revisions and original import remain unchanged |
| Final-source candidate check | passed: 5 isolated cases, no further package changes after apply |
| Full static/host gate | passed: `codecraft validate --static`, 286 passed, no failed/skipped/unrun rows |
| Generation / Skill structure / whitespace | passed: `generate --check`, skill-creator `quick_validate.py` and `git diff --check` |
| Native builds and standalone complete gate | skipped: no firmware or native build inputs/behavior changed; no board toolchain was installed or activated |
| Host Kit execution / model conversion / simulator / physical device / live API | not_run: not part of this maintenance validation; no device revision, runtime or behavior was promoted |

The initial rollback test did not inject its failure because macOS temporary
paths have `/var` and `/private/var` spellings. The fixture now compares resolved
paths; the rollback case and full gate passed after that test-only correction.

The final candidate review digest is
`248ef33a4ba52d565897a1f709b814f0f3e17a1e00438cee7ed0d51b8903ea9b`.
This records local package review, not installation, publication or native
reCamera support. RC-07 through RC-10 and GIT-01 remain open.

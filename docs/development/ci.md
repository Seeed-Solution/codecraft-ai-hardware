# CI selection by native impact

GitHub Actions always runs `codecraft validate --static`. The public
CI planner then selects native Target jobs from the actual committed changes.
Each selected Target retains its existing native command, all examples/variants,
and `--projects` checks. An unselected Target is reported as `skipped`, never as
a successful new firmware build. Physical verification is separate.

Selection lives in `tools/cli/src/maintenance_ci.rs`, with integration tests in
`tests/tools/cli/maintenance_ci.rs`. It reads committed manifests and dependency
closures as data; historical code is never executed. The generated complete
Target matrix is inventory; CI consumes the filtered plan.

## Selection rules

| Change | Native CI selection |
| --- | --- |
| Engineering documentation, README/Agent rules, host tests and retained observations | None; static/host validation still runs |
| Independent Skill/package maintenance | None, unless a Target explicitly consumes the changed native input |
| Board source, examples, assets, native configuration or Target manifest | Its Target |
| Declared shared component/resource | Consumers from the union of the old and new dependency graphs |
| Platform adapter/runtime or undeclared source in a native platform directory | Affected consumers, conservatively expanded to that platform when ownership is unclear |
| Shared native pipeline, export/build orchestration or public native CLI behavior | All dependent Targets |
| Workflow/matrix-generator changes or unclassified infrastructure | All Targets, with an explicit native or conservative-selection reason |
| New Target | The new Target; deleted Targets are recorded but cannot be built |
| Manual full validation or unavailable push baseline | All current Targets |

The planner follows transitive resource dependencies and directory additions,
including repository-internal configuration symlinks. Missing inputs, dependency
cycles, malformed metadata and external/broken links fail planning. Git renames
are expanded into deleted and added paths so neither side loses ownership.
Historical trees are read as data; their Python schema/adapter code is not run.
The CI snapshot inventory accepts historical Platform schema 2 and current schema 3
so a migration can be compared across its boundary. This is not a creator
compatibility reader: current Catalog and application operations require schema 3.

Context hashes include guidance, evidence and pipeline code, so a changed context
hash is not itself a native-build trigger. The native projection excludes reviewed
host metadata/validation modules and documentation. Other declared assets remain
native inputs, including a Markdown asset inside an application's source tree.

Changes to reviewed maintenance-only modules do not change native compiler
commands. Shared creator/native Rust code, dependency or compiler configuration
changes conservatively select all affected Targets. Platform-owned Rust changes
select that stack; unknown infrastructure remains conservative. This is an
ownership policy, not a proof of semantic equivalence.

## Events and public commands

For pull requests, compare the event's base commit with the checked-out test-merge
tree. Both the base and PR head must be ancestors of that tree. This selects the
changes in the actual integration result. For `main` pushes, compare the event's
`before` and `after` commits. Checkout uses full history. A missing/zero push
baseline selects all Targets; an explicitly mistyped local revision fails.

Newer PR runs cancel earlier runs for the same PR because each plan covers the
whole PR delta. Push/manual runs use separate concurrency groups per run: cancelling
an intermediate push could discard a Target change absent from the next delta.

`workflow_dispatch` retains full validation. A periodic full-build schedule is
follow-up work; this workflow does not introduce a schedule.

Inspect committed revisions locally without installing native toolchains:

```sh
codecraft maintenance ci plan --base upstream/main --head HEAD --report /tmp/codecraft-ci-plan.json
codecraft maintenance ci plan --full
```

These commands inspect committed snapshots, not unstaged or staged working-tree
changes. Continue to inspect the actual local diff and select contributor checks
using the [root policy](../../AGENTS.md#required-validation-and-delivery).
No-option `codecraft validate` still runs the full software gate;
it has no automatic Git filtering.

In Actions, the planner reads the event file, writes matrices/counts together to
`GITHUB_OUTPUT`, and appends Target decisions and changed-path reasons to the job
summary. It also writes a local JSON report. Event and job-result data reaches the
planner and aggregate gate through files and environment variables.

```sh
codecraft maintenance ci plan --event "$GITHUB_EVENT_PATH" --event-name "$GITHUB_EVENT_NAME" --github-output "$GITHUB_OUTPUT" --summary "$GITHUB_STEP_SUMMARY" --report "$RUNNER_TEMP/codecraft-ci-plan.json"
codecraft maintenance ci check --needs-env CODECRAFT_CI_NEEDS --summary "$GITHUB_STEP_SUMMARY"
```

Both commands use the public schema-2 JSON envelope; the inner plan has schema 1.
Planning is repository-only. These maintenance modules are not included in native
application exports; their public CI command reports `not_supported` there.

## Aggregate result and validation

The fixed `CI result` job uses `always()` and depends on the static/planning job
and every native platform job. It requires static/planning success, every selected
matrix to succeed, and empty matrices to be skipped. Failure, cancellation,
unexpected skipping, missing platform results, stale tested revisions and
inconsistent matrix/count outputs fail this
gate. Planned omissions remain explicit `skipped` rows in its report.

Administrators can require **CI result** in branch protection. Adding this job
does not configure GitHub branch protection. Avoid path-filtering the entire
required workflow: GitHub documents that such skipped workflows can leave required
checks pending in [its status-check guidance](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/skip-workflow-runs).

Validate changes with focused selection/event/gate tests, public-CLI export tests,
workflow syntax checks and `validate --static`. CI orchestration alone does not
change native compiler commands or require local firmware builds. Workflow edits
conservatively exercise the complete native matrix when run in Actions.

Future refinements can narrow selected Targets to examples/variants after their
dependency boundaries are proven. Reuse of previous native artifacts requires
separate validation of actual native inputs and toolchain identity; this planner
does not relabel historical evidence or implement artifact caching.

## Compiled CLI cache

The `cli` job calls `.github/workflows/cli.yml`. It pins Rust 1.89.0 and builds
Linux x86_64 musl, macOS arm64 and Windows x86_64 executables. Cache identity
contains compiler/target, Rust implementation and test sources, Cargo inputs and
the binary workflow. It excludes Device data, application sources, documentation
and Git commit IDs. Cache hits skip Cargo compilation and run the existing test
executables against the current checkout. Artifacts are workflow downloads; no
release is published automatically. Native jobs use the Linux static executable,
including inside the ESP-IDF container.

Rust CLI source/dependency changes conservatively select all native Targets.
Platform Rust changes select that stack. Documentation changes remain host-only.
The final result job requires the CLI job as well as every selected native job.
It downloads the compiled tool for aggregation. If the CLI job or artifact is
unavailable, the fixed result job fails explicitly instead of needing another
implementation of aggregation.

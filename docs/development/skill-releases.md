# Standalone Skill release workflow

Use this workflow for changes to the bundled cloud Skill packages. It prepares
reviewable local candidates, checks isolated clients and updates reviewed package
fingerprints in tools/skill_packages/packages/<skill-id>.json. It does not install Skills globally, publish artifacts or call a
real cloud service. Original import provenance in tools/skill_packages/import.json and
Skill release source fields is preserved.
The standalone reCamera development Skill uses explicit `--skill
codecraft-development-recamera-pysdk` selection with its own package inventory
and offline acceptance, without a cloud contract. Follow
[reCamera SDK maintenance](recamera-sdk-maintenance.md) for that path; the
unqualified commands below continue to select the cloud bundle.
The separately reviewed current upstream snapshot is recorded in
tools/skill_packages/upstream.json; copying new source must not rewrite the original import.

## Maintained inputs and versions

Edit common client behavior in skills/codecraft-capabilities-llm/scripts/device_api.py. Other package files
remain maintained under their own skills/codecraft-capabilities-* directories. The candidate
builder copies the common helper into every distribution; it does not rewrite
working package copies or their release records while preparing a candidate.

If a distribution helper differs from both its previously pinned bytes and the
maintained helper, preparation fails and preserves the local edit. Reconcile
those sources explicitly before preparing a candidate; packaging must not silently
discard a conflicting edit to a distribution copy.

A candidate covers all registered repository-skill packages together. Its
manifest records the exact maintained source graph, current package inventories,
previous/proposed release records, file additions/removals/changes and independent
versions. Package changes containing only Markdown files are classified as
documentation; code, assets, Agent configuration or other changes are runtime
changes. This classification does not infer API compatibility.

Any package-byte or API-version change requires an explicit higher numeric
major.minor.patch Skill version. Policy versions cannot decrease. Contract
versions must match the reviewed Skill contract. A changed semantic contract or
binding must be implemented and reviewed at its owning source; the packager
does not invent operation translations or update the contract itself.

Version overrides are a JSON object keyed by Skill ID. For example, after a
documentation-only ASR edit from package 2.0.1:

```json
{
  "codecraft-capabilities-asr": {
    "skill_version": "2.0.2"
  }
}
```

Optional fields are api_version, contract_version and policy_version. These must
match the separately reviewed values in package contract.json; omitted versions
remain unchanged. Release record schema is 1; candidate manifest schema is 3 and Skill contract container schema is 2. Candidates bind the compiled maintenance CLI and validation sources; older candidates must be re-prepared and reviewed. A shared-helper edit needs a reviewed version change
for every affected package. Do not change versions or hashes merely to suppress
a failing contract test.

## Prepare and inspect

Save version overrides outside the repository and choose a new output directory:

```sh
codecraft maintenance skill-release prepare --versions /work/release-versions.json --output /work/skill-candidate
```

For a reproducibility check with unchanged package inputs, omit --versions.
Outputs contain complete skills/<name> packages, proposed package release records
and manifest.json. Identical source bytes and version overrides produce identical
candidate file bytes and review_sha256 values; no timestamp or absolute source
path participates in the candidate identity. Normal Python caches and .DS_Store
are excluded; package symlinks and private environment files are rejected.
Validation and application copy only the pinned files and verify their copied
bytes against the reviewed hashes. Ignored caches never enter the staged or
applied package, and a file changing during copying causes failure.

Review the actual candidate files, each before/after fingerprint, the proposed
versions, policy and CLI/API behavior. The manifest reports changes relative to
the previously pinned package, including local edits not yet accepted by the
normal repository gate. A clean candidate does not by itself approve a release.

## Validate and apply

```sh
codecraft maintenance skill-release check --candidate /work/skill-candidate
codecraft maintenance skill-release apply --candidate /work/skill-candidate --review-sha256 <reviewed-64-character-digest>
```

check stages the candidate in a temporary repository with the exact declared
validation inputs. It checks complete pinned package inventories and CLI flags,
shared-helper equality, 16 offline protocol cases, seven onboarding/authentication
cases, six realtime cases and the thirteen current direct-client conformance
functions. Activate the host test environment from [testing.md](testing.md) first;
the optional client dependency pins are required by these suites.
Each client runs alone against a loopback
fixture with synthetic credentials. The explicit release-checks.json registry
must cover registered Skills and their operations. Added Skills require
their own concrete tests; the workflow does not fabricate a substitute service.

apply requires the exact reviewed digest and reruns those checks. A modified
candidate, missing/extra files, changed maintained sources or outdated release
inventory fails before application. Validation receipts are not trusted as a
substitute for this rerun. A release lock prevents simultaneous apply operations.

Only changed distribution directories and release records are replaced.
Unrelated repository files and original provenance are preserved. On a write
failure, completed replacements are restored. If restoring a backup itself
fails, recovery files remain under the reported skills/.release-* directory;
inspect and recover them before retrying. Do not remove a live release lock or
discard retained originals merely to clear an error.

After applying, review the resulting diff and run:

```sh
codecraft maintenance generate
codecraft maintenance generate --check
codecraft validate --static
```

Select any native/export builds using the
[root validation policy](../../AGENTS.md#required-validation-and-delivery).
Cloud client or Skill-only changes with unchanged native build inputs/behavior
do not require firmware compilation or the complete gate. No command here
flashes, publishes or establishes live service acceptance.
Copy an approved complete Skill directory for independent installation; runtime
imports remain inside it. An installed Skill does not require the repository CLI
or a sibling Skill. Existing exported applications keep their locked versions
until an explicitly reviewed application update or a new export.

The full workflow is repository maintenance. Plain exported applications carry
portable workflow guidance and native build inputs, using the separately installed Rust CLI, but do not carry the
release maintainer or its test suite; skill-release reports not_supported there.

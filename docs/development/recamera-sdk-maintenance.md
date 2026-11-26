# reCamera SDK Skill maintenance

This is repository maintenance for the standalone
`codecraft-development-recamera-pysdk` package. Application execution remains in
the installed Skill; hardware support remains a separate Device/Target concern.
See the [ownership and TODO ledger](recamera-sdk-todo.md).

## Sources and independent identities

- `tools/cli/src/maintenance_reference.rs`, `tools/skill_packages/recamera/api-notes.json` and
  `api-http-notes.json` own reference generation and imported editorial notes.
- The package's `scripts/api-reference-lock.json` identifies the API source
  revision, source inventory and generated pages. Original upstream descriptions
  retain their language and attribution; locally authored maintenance text is
  English. Do not hand-edit generated pages or their hashes.
- `scripts/sdk-contract-lock.json` separately identifies the bundled official
  builder and watched SDK contracts. Reference regeneration does not update it.
- `tools/skill_packages/recamera/release.json` pins every installed package file
  and its independently reviewed package version. It is outside the package.
- `docs/recamera-skill-import.json` is immutable original import evidence. It is
  not the current package inventory and must never be refreshed to accept edits.

The current execution helpers, official builder, manifest semantics and target
runtime profile are retained. The Skill contains no Device registration, cloud
contract or dependency on repository-maintenance code. The four cloud Skills
continue using their existing release registry and contract checks.

## Reference changes

Use the public CLI from the repository, with a local SDK checkout or exported
source tree matching the chosen reference baseline:

```sh
codecraft maintenance skill-reference --skill codecraft-development-recamera-pysdk --sdk-root /work/recamera-sdk --check
```

The check parses source without importing the SDK or executing application code.
It compares modules, public symbols, signatures, HTTP routes, source hashes and
generated page bytes. A mismatch reports `failed`; it does not refresh a pin.

After reviewing the source changes and the imported editorial notes, regenerate
with an explicit full commit. Writes require a clean Git checkout at that commit:

```sh
codecraft maintenance skill-reference --skill codecraft-development-recamera-pysdk --sdk-root /work/recamera-sdk --write --revision <full-commit>
```

The generator fails on new undocumented modules/routes and incomplete sources.
It removes only obsolete generated pages named by the previous lock. Inspect
the resulting diff, then run the read-only check again. Application-side builder
drift checking remains available through the installed `scripts/sdk_contract.py`.

## Candidate, check, review and apply

The existing [release workflow](skill-releases.md) has explicit development-Skill
selection. Omitting `--skill` retains the four-cloud-package behavior. For this
SDK package, use a version file outside the repository:

```json
{
  "codecraft-development-recamera-pysdk": {
    "skill_version": "1.1.0"
  }
}
```

Select a higher version than the current release record whenever any package
bytes change. Package versions cannot decrease. The version file accepts only
`skill_version`: SDK/API revisions remain separately reviewed source locks, not
cloud API, semantic-contract or service-policy versions.

```sh
codecraft maintenance skill-release prepare --skill codecraft-development-recamera-pysdk --versions /work/recamera-versions.json --output /work/recamera-candidate
codecraft maintenance skill-release check --skill codecraft-development-recamera-pysdk --candidate /work/recamera-candidate
codecraft maintenance skill-release apply --skill codecraft-development-recamera-pysdk --candidate /work/recamera-candidate --review-sha256 <reviewed-digest>
```

Candidate preparation is read-only for the repository. Omit `--versions` only
for an unchanged-package reproducibility check. The candidate records complete
package files, previous/proposed records, the exact maintenance/test source
closure and a deterministic review digest. Original import identity is preserved.

Check copies the candidate into a temporary source closure and actually runs the
Skill's host conformance suite. Those tests separately copy the installed Skill,
with no repository or SDK imports available to its application commands. Empty,
failed or skipped check results cannot pass. No live device/API access occurs.

Review the file-level changes, version and test output before applying that exact
digest. Apply reruns isolated checks and rejects changed candidate/source bytes,
private files and symlinks. The shared release lock prevents concurrent applies;
failed replacements restore the prior record/package or retain recovery files
with a reported path if restoration fails. This updates local reviewed bytes;
it does not install a Skill, push Git history or publish an artifact.

## Required verification

```sh
codecraft maintenance test --path tests/skills/codecraft-development-recamera-pysdk
codecraft maintenance test --path tests/tools/skill_packages/recamera
codecraft maintenance generate --check
codecraft validate --static
```

Use the [host environment](testing.md) for the full static gate. The focused SDK
suites need Python and Git, with synthetic local source/package fixtures only.
The full official reference check additionally needs the pinned SDK source;
absence of that checkout is `not_run`, not a matching-source claim.

Native firmware, model conversion, host Kit execution, installation and physical
behavior are separate checks. Select them only when the actual change or user
request needs them; package maintenance alone does not authorize device writes.

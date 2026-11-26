# Repository tooling

The public entry is `codecraft`. Read the root repository contract before edits.
All repository tooling is implemented in tools/cli/; do not add Python backends,
forwarding modules or duplicate project/catalog implementations. Native commands
remain owned by platforms/<stack>/native.rs. Runtime data is read from disk.

## Creator and maintenance ownership

Creator catalog, dependency graph, export, lock, build and firmware orchestration
share the same implementation with maintenance validation. Maintenance modules
own checks, evidence, generation, CI and package transactions. Platform metadata
and offline toolchain checks remain platform-owned. Source-bound evidence must
include compiled CLI identity; historical records and logs remain unchanged.

Application exports carry native sources and portable guidance from
tools/application/resource.json, not maintenance sources or CLI binaries. CLI
identity is locked separately. Read docs/development/rust-cli.md for lifecycle
and Python-runtime boundaries. Code changes require focused Rust tests under
tests/tools/cli/; independent client/native-host tests remain under their owners.

## Package maintenance

skill_packages/ retains release records, immutable imports/upstream snapshots,
reference-generation notes and the executable conformance registry. Maintenance
logic lives in maintenance_skills.rs, maintenance_contracts.rs and
maintenance_reference.rs. It is not an installable Skill.

The common Python client helper is maintained in the existing standalone LLM
package at skills/codecraft-capabilities-llm/scripts/device_api.py. Reviewed
candidates distribute those bytes to other independently installable packages.
No package may import another package at runtime. Follow
[release maintenance](../docs/development/skill-releases.md); do not refresh
hashes to accept unreviewed changes or rewrite original import provenance.

reCamera reference generation parses SDK sources without executing them. Follow
[SDK maintenance](../docs/development/recamera-sdk-maintenance.md). Candidate
apply requires an exact reviewed digest, reruns offline isolated conformance,
rejects drift and restores completed replacements on failure. Preserve recovery
backups when rollback cannot complete. Packaging does not authorize publication.

## Checks and dependencies

Use `codecraft validate --static` and the explicitly registered Cargo tests.
The Rust host-test dispatcher may invoke Python/pytest for independent Python
client and C-fixture test suites under tests/. Those are test dependencies, not
an alternative tools implementation. No real credentials, API calls or device
writes are authorized by a validation command. See
[test ownership](../docs/development/testing.md) and the root check-selection rules.

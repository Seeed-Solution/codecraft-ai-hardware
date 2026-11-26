# Skill ownership and maintenance

This directory owns independently installable hardware workflow and cloud
capability Skills. Read the root contract before changes. Native build
commands stay in Platforms; board implementations and physical facts stay in
Devices. SenseCraft platform authentication, telemetry, Hub and upload commands
remain in the external official CLI and Skill.

Keep each Skill directly under skills/ with its directory name matching the
SKILL.md name and Agent invocation metadata. Development workflows use
codecraft-development-*; cloud capabilities use codecraft-capabilities-*. These prefixes
classify Skills without adding category directories to installation paths.
Creator orchestration belongs in tools/cli/, repository audits in tools/cli/,
and package maintenance in tools/skill_packages/. None is an installable Skill or
a client dependency.

Each cloud client owns contract.json, SKILL.md, API references and package-local
runtime code. Stable semantics, service policy and API versions remain distinct.
Policy constraints may only narrow the semantic schema; defaults require explicit
policy_default permission. Archival is never an implicit default. Unknown
resource bounds and missing execution prerequisites must remain explicit.

Cloud release records in tools/skill_packages/packages/<skill-id>.json own distribution versions,
complete file hashes and original import provenance. Keep these records outside
packages to avoid self-hashing. Never refresh hashes to accept unreviewed edits.
Follow [release maintenance](../docs/development/skill-releases.md) and apply only
the reviewed candidate digest. Packaging is not installation or publication.

The standalone reCamera development Skill has its separate inventory at
tools/skill_packages/recamera/release.json and immutable import record at
docs/recamera-skill-import.json. Follow [SDK maintenance](../docs/development/recamera-sdk-maintenance.md)
for explicit candidate selection and API reference production. It has no cloud
contract and must not enter native discovery until a real Device/Target exists.

Runtime imports must stay inside the copied package. Shared repository contract
checks must not import the hardware Catalog or call a service. Native-only
operations must not load or export cloud records, contracts or clients. An explicit
Skill selection adds only that package and its actual verification dependencies.
See [contract ownership](../docs/architecture/skill-contracts.md).

Skill tests belong under tests/skills/<skill-id>/. Repository schema/export
checks belong under tests/tools/cli/; protocol/release cases belong under
tests/tools/skill_packages/. Run actual isolated clients against loopback fixtures with synthetic
credentials. Preserve input/output, default, privacy/TLS, failure and no-retry
coverage; --help checks alone do not establish integration.

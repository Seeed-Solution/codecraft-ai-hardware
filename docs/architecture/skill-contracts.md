# Standalone Skill contracts and release ownership

Each inference Skill owns its typed contract and independently runnable client.
Hardware peripherals remain Device facts and Target native interfaces. The cloud
server and SenseCraft platform management remain external.

Development workflows use codecraft-development-* package names. Cloud capability packages
use codecraft-capabilities-* names, including matching directory names,
SKILL.md identities, Agent invocation prompts and contract IDs. Both categories
remain flat under skills/ for installation. Repository tools live under tools/.
The service API, authentication environment names and operation schemas do not
depend on the distribution name. Original upstream paths in provenance retain
their source names; they are not runtime aliases for maintained packages.

## Single sources and independent versions

| Source | Responsibility |
| --- | --- |
| skills/<id>/contract.json | Semantic operations/input/output/configuration, errors, risk, resource dimensions, client bindings, service policy and API identity |
| skills/<id>/SKILL.md and references/api.md | Task authorization, usage and attributable service explanations |
| skills/<id>/scripts/ | File/content checks, authentication, HTTP, streaming, media and optional archival |
| tools/skill_packages/packages/<id>.json | Distribution version, complete package inventory, reviewed contract/API/policy versions and immutable original provenance |
| tools/cli/src/maintenance_contracts.rs | Repository validation, explicit export-selection checks and package graph composition |
| docs/skill-inventory.json | Generated ID/operation/version navigation for callers selecting a package |

Contract containers use schema 2; release records use schema 1. Semantic contracts
are 3.0.0, service policies 2.0.0 and public API contracts v1. Distribution versions
describe package bytes independently; read the selected package's release record
for its current version. Complete inventories live outside the package, so they can
pin contract.json and every other file without a self-hashing loop. Runtime
imports stay within one copied Skill directory; repository helpers are never a
client prerequisite. Original import.json and record source fields are immutable.
tools/skill_packages/upstream.json records the separately reviewed source snapshot used for
the current copy, including uncommitted source bytes; it is part of selected
package exports and release candidates.

## Semantics and service policy

Semantic operations retain no automatic retries, explicit archival, complete
streams, text-only LLM roles, single-recording ASR and one-image generation.
Service policy owns current bounds, billing declarations, restrictive schema
fragments and permitted defaults. Service limit keys cannot override semantic
limits. Only semantic configuration fields with policy_default: true may receive
defaults; currently LLM max_tokens and image size qualify. Caller overrides take
precedence. Archive must remain explicit.

Constraints may narrow existing fields, enums, ranges, lengths, required fields
or additional-property policy. They cannot add fields, change types, replace
an existing pattern or weaken a bound. Source references must name pinned package
files. Unknown billing retains null unit/basis/amount; declared amounts are
accounting units, not currency or observed charges. Null resource estimates are
unknown, never proof that a constrained host can run the client.

## Invocation and enforcement

Read the selected SKILL.md, API reference and contract; run its documented entry
directly only within the authorized task. The client owns concrete file validation,
effective defaults, authentication, TLS/loopback handling, protocol checks and
nonzero failure exits. Guidance requires explicit privacy/network decisions and
credential-reference availability; it does not turn metadata into authorization.
No inference request is made by discovery, context, export or package validation.

Repository selections contain exactly skill, configuration, environment and
limits. Configuration is checked against the policy-constrained schema. Environment
contains boolean network/tls/loopback/privacy_approved decisions and lists of
environment_refs/secret_refs names. It never carries credential values. Missing
prerequisites or unknown constrained resources block that selection. These are
declarations, not live service probes. Hardware integration remains unknown.

Each operation declares its own pinned entrypoint and explicit subcommand array.
Public API bindings contain only the method/path and fixed service routing;
model, provider and voice selection are absent. ASR includes file transcription,
realtime streaming, status and explicit same-key recovery. The recovery operation
can reserve quota if the original request never arrived; it is not a read-only probe.
Native integration guidance and the C authentication helper do not establish
board firmware support. Optional Python requirements apply to workstation helpers.

All eight operations are tested through actual isolated clients against loopback
fixtures, including default/override behavior, local/archived output, file bounds,
SSE DONE and finish status, realtime pacing/recovery, retained partial output and
nonzero failures. A schema/CLI mapping is
maintenance metadata, not another production invocation engine. Content checks
remain in the client; offline results are not measured billing or live acceptance.

## Graphs, locks and releases

Native-only graphs contain no cloud contract checker, release record or inference
client. Explicit selection adds that package's complete pinned files, release
identity, provenance and neutral verification dependencies. Hardware Catalog
construction is not a dependency of package checks or release maintenance.

CLI contract 3 uses explicit --skills selection files for context/new. Project
profiles and locks use schema 3. The JSON result envelope remains schema 2;
native adapter protocol versions remain independent. Existing application copies
keep their own locked source until a reviewed update; reference changes never
rewrite private projects. No fallback readers or forwarding commands are provided.

Use the candidate/check/review/apply workflow for every package-byte change,
including contract changes. Version overrides explicitly identify all reviewed
version changes; semantic/API/policy values must match the package contract.
Application names the exact digest, reruns isolated conformance, rejects source
drift and preserves transactional rollback. A package release does not install,
publish, contact a service or certify physical hardware.

# Local CLI boundaries and implementation acceptance

Status: upstream 2.0.1 synchronized and static/host acceptance passed, 2026-11-18 (Asia/Shanghai).
This document records the accepted scope, current command ownership and acceptance
items. The governing [architecture requirements](codecraft-hardware-distilled.md)
and [repository contract](../../AGENTS.md) use Devices, Platforms and Skills as
the three content owners. No firmware compilation is requested for this work.

## Boundary

Keep `codecraft` for local hardware development and repository
maintenance. SenseCraft platform authentication, managed devices, telemetry, Hub,
uploads and cloud deployment belong to the official external sensecraft-cli and
its bundled Skill. Direct inference belongs to the four standalone CodeCraft
Skills. There is no local pass-through wrapper for the external platform CLI.

The reviewed external executable is published version 0.5.0. Its `firmware advise`
provides offline upload guidance, while local firmware plan/merge/verify owns
native layouts and integrity. External app creation creates a Hub object; local
new exports a native application. External image help describes Agent SVG
creation, not the CodeCraft image inference endpoint. The observed PATH-selected
0.4.5 binary differs; verify the selected executable/help before relying on a
command. No live API, authentication lookup or upload was performed in this review.
See [external setup and routing](sensecraft-cli.md).

## Current local commands

| Command | Responsibility |
| --- | --- |
| catalog | Hardware-only metadata; result schema 4, no cloud initialization |
| discover | Match native Targets/interfaces and preserve unknowns |
| context | Selected task guidance plus its complete source graph; optional explicit --skills selections |
| new | Native export, brief, optional explicit Skills and source lock |
| verify-project | Check the requested application path, without a reference catalog prerequisite |
| lock-project | Record reviewed current source/environment while retaining immutable origin |
| build-project | Enforce the native toolchain, lock and build receipt |
| package | Produce a verified local source archive; no upload |
| firmware plan / merge / verify | Native image layout, successful build receipts, merging and offline integrity |
| generate | Derive Device/interface/Skill/CI views from their actual owners |
| validate | Report selected repository, host and native checks distinctly |
| test | Run the single central host-test inventory |
| record-evidence | Register attributed, source-bound physical/build observations |
| check-cloud-skills | Verify complete released package inventories and isolated entry contracts |
| skill-release prepare / check / apply | Reproducible candidates, explicit version review, exact digest and transactional application |

CLI contract 4 retains the removal of the generic service catalog/resolver/preparation commands
and the redundant public CI-matrix command. The CI matrix producer, generation
and drift checks remain. JSON envelope schema 2 and native adapter protocols
remain unchanged. Creator project profiles/locks use schema 4. Explicit --skills files
contain Skill IDs, configuration, declared environment references/decisions and
limits; there are no forwarding aliases or mixed-schema readers. Existing private
applications retain their own locked code until a deliberate reviewed update.

## Skill ownership and dependency closure

```text
skills/
  AGENTS.md
  codecraft-development-*/                  # Hardware development workflows
  codecraft-capabilities-*/    # ASR, TTS, LLM and image generation
    SKILL.md
    contract.json              # Semantic contract and separate service policy/API versions
    references/api.md
    agents/openai.yaml
    scripts/                   # Package-local runtime imports only
tools/
  AGENTS.md
  cli/                         # Standalone creator CLI, source graphs, exports and locks
  skill_packages/
    packages/<skill-id>.json    # Distribution version, complete hashes and provenance
    import.json                # Immutable original import provenance
    upstream.json              # Reviewed upstream snapshot
    release.py
    check_candidate.py
    release-checks.json
tests/
  skills/<skill-id>/            # Direct-client and workflow conformance
  tools/cli/                   # Shared contracts, project workflows and maintenance
  tools/skill_packages/         # Protocol/install/release checks and loopback fixtures
docs/
  skill-inventory.json          # Generated ID/operation/version navigation
```

Inventories live outside their packages and pin contract.json along with every
other file, including explicit binary resources. Semantic, policy, API and
package versions remain separate. Source changes use the candidate workflow;
original attribution is preserved. This structure implements no generic
multi-provider registry or empty model registry. A second service, MQTT or
local/edge execution still needs a real client, requirements and conformance.

Native-only source graphs contain no cloud contract code, release records or
clients. Explicit selection adds only the selected packages and their verification
and provenance dependencies. Repository Skill-contract checks never import the hardware
Catalog. Application-only commands load the requested application directly.
Platform and portable workflow guidance are included in the same graph/lock;
context identifies its selected guidance separately from the complete file closure.
See [contract ownership](../architecture/skill-contracts.md) and
[portable application workflow](application-workflow.md).

## Retained checks and final owners

| Condition | Enforced owner and acceptance |
| --- | --- |
| Typed operation input, exclusive LLM input and configuration | Skill contract checks and actual standalone argument/file validation |
| Defaults and overrides | Contract permission, restrictive policy checks and real default/override wire assertions; archive stays explicit |
| Network/TLS, privacy and reference availability | SKILL task guidance and explicit project-selection checks; client enforces actual TLS/loopback/authentication |
| Unknown host resource bounds | Contract estimates and selection validation; unknown constrained resources block selection |
| Audio/text/image limits | Actual standalone file/content checks exercised at positive and negative boundaries |
| Output schemas, SSE completion, local/archived media | Direct client loopback cases and output-schema checks; failures retain nonzero exits |
| No charged retries and retained partial/local output | Package runtime and protocol/conformance cases |
| Full package identity and independent versions | External release records, inventory checks and reviewed candidates |
| Source closure, binary assets and immutable origin | Export/lock checks, exact inventories and drift rejection |
| Independent operation | Isolated package execution, cloud checks without hardware imports and native exports without cloud imports |

## Implementation TODOs

- [x] **CLI-01 — Governing ownership.** Root/local rules, distilled specification,
  architecture and workflow guidance agree on the three content owners.
- [x] **CLI-02 — Contracts and records.** Four package-owned contracts cover eight
  operations; outside inventories avoid self-hashing; selection/profile/lock
  formats are explicit and versioned.
- [x] **CLI-03 — Direct-client parity.** Retain every condition in the ownership
  table above, including actual client defaults and failures.
- [x] **CLI-04 — Reviewed release.** Prepare, inspect, check and apply the exact
  candidate digest. Package versions are 2.0.1; semantics 3.0.0, policy 2.0.0 and
  API contract v1 retain their independent meaning. No installation/publication is implied.
- [x] **CLI-05 — Dispatch and graphs.** Native-only operations avoid cloud code;
  explicit applications avoid reference initialization; portable context closes
  OPT-20; public CLI exposes only the current commands above.
- [x] **CLI-06 — Remove superseded owners.** Contract/release/test/CI/documentation
  consumers use their current owners, with no forwarding modules or dual registries.
- [x] **CLI-07 — Final acceptance.** Generation drift, documentation, focused
  contracts/releases and the full static/host gate pass. Isolated exports cover
  all Targets/variants with and without explicit Skills, with exact locks and
  all eight workflow contexts. Confirm native source/configuration/toolchain and
  build-command bytes are unchanged.

## Verification scope

The current copy contains 40 byte-identical upstream files across four Skill
packages. Repository-owned contract.json files describe eight public operations,
fixed service routing, per-operation entrypoints and explicit subcommands.
The reviewed release candidate
`65e2e76be1da072c63f9fd9caefffce44a9f8b47892a397a03ff0e9a34c3131e`
passed 20 candidate checks and the same checks during apply: four package checks,
16 HTTP/SSE/media cases, seven onboarding/authentication cases, six realtime cases
and 13 direct-client conformance functions. Package distributions are 2.0.1 and the public API contract is v1; policy
versions are 2.0.0 and semantic contracts are 3.0.0. The current
source snapshot is recorded separately in tools/skill_packages/upstream.json.
Native firmware/configuration/build inputs, historical evidence and original
import provenance are unchanged. No global installation or publication was performed.

`codecraft validate --static` passed all 269 reported checks for the
synchronized 2.0.1 snapshot. The host environment uses Python 3.14.6, Pillow 13.0.0 and
websockets 15.0.1. Generation drift and final diff checks also passed. Isolated export
checks cover all 10 Targets / 11 Target-variant combinations with native-only and
four-Skill selections, exact locks and all eight workflow contexts.

The scope includes host tooling, contracts, documentation and tests. Native
firmware compilation and the complete build gate are skipped by the user's
instruction and unchanged firmware/native build inputs. Host fixtures and Linux
overlay checks in the static suite remain distinct. On-device and live cloud API
acceptance are not run, and historical evidence is not promoted by these changes.

The repository tooling migration consolidates maintenance in tools/cli/. The
shared client helper is owned by the standalone LLM package; reviewed releases
distribute its bytes without cross-package runtime imports. Historical execution
records above retain the environment and results that actually ran.

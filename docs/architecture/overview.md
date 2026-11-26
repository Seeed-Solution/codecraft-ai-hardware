# Hardware repository architecture

The repository has three content owners. Devices own hardware knowledge and native Targets;
Platforms supply shared software-stack infrastructure; Skills own development
workflows, cloud operation contracts and standalone clients.
Repository tooling in tools/ supports these owners without becoming an
installable Skill. Development workflow packages use codecraft-development-* names; cloud
capability packages use codecraft-capabilities-* names, all directly under skills/.
The governing requirements are [codecraft-hardware-distilled.md](../development/codecraft-hardware-distilled.md).

| Owner | Authoritative inputs | Derived outputs |
| --- | --- | --- |
| Device / Target | Per-device knowledge, authored specifications, assets, target.json, native BSP/configuration/examples | Device index, native interface bindings, scoped source closure and support state |
| Platform | adapter, platform.json, shared build helpers and reusable components | Native build cases, stack validation and CI matrix |
| Skill | workflows, contract.json, service policy and standalone clients | Installable development workflows and cloud capability packages |
| Repository tools | Creator orchestration in tools/cli/; repository checks in tools/cli/; helper sources, provenance and release records in tools/skill_packages/ | Task context, project lock, build receipt, generated indexes and reviewed release candidate |

Each Device registers its native Target at targets/target.json. That manifest
selects one software stack; peripheral interfaces belong to the Target. Cloud contracts belong to their consuming Skills and do not represent GPIO, display or individual device peripherals.
Cloud servers, accounts, pairing, telemetry and publishing infrastructure are external.

## Device knowledge entry

Each Device has one authored English AGENTS.md containing knowledge, sourced
specification tables and acceptance procedures. The deterministic reader follows
[Device documents and shared references][device-documents]. Shared documentation
has one maintained owner and is referenced through links. devices/AGENTS.md is the
derived directory index; Device documents themselves are never regenerated.

Device knowledge includes hardware behavior, reference implementation notes,
protocols, sources and media. Supporting material belongs in the device's
reference/ directory when needed. Concrete Target implementations live under
devices/<device>/targets/, including BSPs, examples and board configuration.
Platform owns shared stack/runtime adapters, build helpers and reusable components.
Target manifests bind native interfaces to implementations; Target READMEs document software
integration and build procedures. A Target can depend on Device reference source
through the same declared graph; it does not duplicate that source.

Assets are declared inline; an empty list means none are registered. Only actual
asset files need assets/. Dated observations, structured evidence and raw logs
live in tests/devices/<device>/ and are linked from the Device entry. Acceptance
procedures stay in the authored Device document. Board-specific host tests and fixtures live
in tests/devices/<device>/targets/; observations, records and logs are
excluded from host discovery. All other tests, fixtures and mocks also live under
tests/, grouped by the source owner according to [test ownership](../development/testing.md).

```text
devices/
  AGENTS.md                  # Device directory index
  <device>/
    AGENTS.md                # Knowledge, sourced specifications and acceptance
    reference/               # Supporting reference material, when needed
    assets/                  # Actual registered media, when present
    targets/
      target.json            # Device × software stack binding
      bsp/                   # Board drivers, where the stack uses a BSP
      examples/              # Standalone native examples
                             # Linux retains its native overlays/ and demos/
platforms/<stack>/
  platform.json              # Stack/runtime adaptation contract
  native.rs                  # Rust creator native operations
  components/                # Reusable native components, where needed
  cmake/                     # Shared native build helpers, where needed
skills/
  codecraft-development-*/               # Installable development workflow Skills
  codecraft-capabilities-*/ # Independently installable cloud capability Skills
tools/
  cli/                       # Rust creator CLI and orchestration
  skill_packages/            # Client maintenance, provenance and release tooling
tests/
  AGENTS.md                  # Test directory index and evidence rules
  devices/<device>/
    targets/                 # Board-specific host tests and fixtures
    observations.md          # Attributed observations, when available
    records/                 # Structured evidence
    logs/                    # Original sanitized logs
  platforms/<stack>/         # Adapter tests and shared test support
    components/<component>/  # Shared component tests and fixtures
  skills/<skill-id>/          # Workflow and standalone-client tests
  tools/cli/                 # Standalone Rust creator CLI
  tools/skill_packages/       # Installation, protocol and release tests
```

Target IDs remain <stack>/<device>; the stack is metadata, not another directory
inside targets/. Native software stacks retain their own ABIs. Each native file has
one owner: Targets reference shared Platform resources through their declared
dependency closure. Platform directories do not contain board copies or forwarding
paths. The same layout is preserved in generated projects and their locks.

Creator source graphs, profiles and locks use schema 4 and hash complete files. Historical evidence retains its original schema without a second runtime. Exports copy the
authored Device entry unchanged; derived indexes and evidence summaries live
outside it. Editing hardware guidance or acceptance changes source identity.

## Dependency and invocation boundaries

The public command is `codecraft`. Its implementation lives in
`tools/cli`; platform-owned Rust adapters implement native build behavior. Repository maintenance uses the same executable via `codecraft maintenance`. Repository
and exported application commands share one result contract. The application
workflow is `new`, `verify-project`, `lock-project`, `build-project`, then
`package` when a source archive is needed. Build and package never write a device.

Standalone Skill contract checks import neutral registry/graph helpers and never
the hardware Catalog. Native-only graphs exclude cloud code and records. Explicit
Skill selection validates configuration and declared prerequisites, then exports
that package. Authorized execution calls the standalone entry directly. See
[Skill contracts and release ownership](skill-contracts.md).

Context, export, locks and evidence use the same transitive source graph. Missing
inputs fail. Creator exports contain native sources and portable guidance. The separately
installed CLI binds the compiled adapters and orchestration by binary identity;
an explicit Skill selection adds its complete pinned
package and release identity. The immutable import origin and the reviewed application snapshot
are recorded separately. Source changes invalidate current support evidence. Toolchain observations apply
to the recorded environment, not to whichever SDK is installed on the reviewer's
machine. Application builds separately compare the active environment with the
reviewed lock.

## Private projects and V0.2 alignment

User applications may be generated under ignored projects/<name> or explicit external paths. project.yaml owns a validated brief; creation and relocking render its marked application AGENTS region without replacing custom rules. Reference graphs reject private-project inputs, while application locks still cover their complete source inventory. Ideation context adds the ideate and discover Skills. See [project workspaces](../development/project-workspaces.md).

The [local architecture plan](hardware-architecture-plan.md) adapts the Feishu design,
and the [alignment TODO](../development/architecture-alignment-todo.md) tracks current gaps.
The [V0.2 ledger](../development/v0.2-todo.md) tracks the design target separately from current implementation. SenseCraft platform operations use the official external sensecraft-cli and bundled Skill, with [setup/routing guidance](../development/sensecraft-cli.md) in this repository. Authentication, platform HTTP calls and command implementation stay upstream. Existing CodeCraft inference clients remain in their Skill packages; the verified platform command set does not replace them. MQTT/local/edge Providers and a registered simulator are pending. The existing simulation decision is a current limitation, not completion of V0.2 simulator acceptance.

## Native implementation

ESP-IDF BSPs separate public include/ contracts from private src/ implementation.
Each example declares its native composition and managed dependencies. The
adapter checks actual compiled sources and requirements after a successful build.
Board wiring and resource ownership stay in BSPs; shared protocols stay in
components. See [native compositions](../development/native-compositions.md).

Arduino Targets declare exact CLI, core, FQBN and library versions. The adapter
checks installed inputs, fingerprints the selected dependencies and passes
explicit library paths to the compiler. Sketches remain independent native
applications. Linux Targets own their overlays and device-interface demos;
source validation and overlay compilation are reported separately.

Parsed Device Markdown uses schema 5 and Target manifests use schema 3, each with an explicit variants object.
A selected variant follows context, export, native configuration, locks and
evidence. A build default does not identify a physical assembly. See
[hardware variants](../development/hardware-variants.md).

## Validation and maintained content

Host tests live under tests/, grouped by behavior owner, and are collected
recursively through one runner. Device physical acceptance is separate. Use `codecraft maintenance test --list`,
`codecraft maintenance test --path <owner>` and `codecraft validate --static`; select native builds using the
[root validation policy](../../AGENTS.md#required-validation-and-delivery).
No-option `validate` runs the complete gate and does not filter by changed files.
Unchanged firmware/native inputs do not require compilation; inspect every
selected check's result.
The repository derives support from source-bound evidence, not README assertions.
Context derives support from current graph and CLI-bound evidence; historical
record schema 2 remains archival and cannot promote schema 4 sources. See [evidence compatibility](../development/rust-cli.md#evidence-compatibility). A native
build and a clean locked export are required for build-verified status; physical
support requires actual revision-specific observations.
For a current passing build record, the record and each generated project need a
successful structured toolchain probe; every passing native case must identify
the same probe or its command version. Platform adapters own offline checks of
that identity against Target requirements via `validate_evidence_toolchain`.
No toolchain installation, live probing or device access is needed to review
recorded evidence. Failed/unrun observations remain recordable; stale records and
original logs remain unchanged and cannot promote current support. ESP-IDF local
`-dirty` edits are permitted and tracked by the existing application lock.

The `codecraft maintenance check-cloud-skills` command loads its own package validation without
importing the hardware Catalog. Hardware and application implementation imports
are deferred until their command or operation needs them.

README files are bilingual. Engineering guidance is English and describes the
current contract. Generated views are updated from their owning inputs. Source
provenance and real observations remain attributable; software that has no
current consumer, duplicate command entries and transition-only records are not
maintained architecture. See [testing](../development/testing.md),
[Skill releases](../development/skill-releases.md) and [open TODOs](../development/optimization-plan.md).

[device-documents]: ../development/device-documents.md

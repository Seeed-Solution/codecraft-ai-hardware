# Repository Contract for AI Agents

This file is the mandatory entry point for using or changing this repository.
Read Part I for every task. Part II also applies when changing repository
code, documentation, metadata, skills, tests, or workflows. Building an existing
example or creating a private project under projects/ or outside the repository
does not by itself make the task a repository contribution.

**MUST** and **MUST NOT** are requirements. An unmet requirement must be reported;
it must not be silently omitted or counted as satisfied. These rules operate
within the user's instructions and the environment's permissions. Local
`AGENTS.md` files and Skills add task-specific rules; they do not grant authority
to take actions outside the user's request.

Repository-wide scope, source, and evidence requirements apply at every layer;
local guidance MUST NOT weaken them. If instructions conflict, identify the
conflicting files and resolve the applicable rule before the dependent action.
MUST NOT silently select the easier rule. If the conflict cannot be resolved
from the user's instructions and authoritative sources, ask for the necessary
decision while continuing independent work.

## Part I: Current repository and usage rules

### Repository scope and current structure

This repository contains Seeed board support packages (BSPs), standalone demos,
verified hardware facts, development workflows and standalone Agent clients for
CodeCraft cloud APIs. Cloud clients live in their Skill packages. Server runtime,
device provisioning/pairing, account management and telemetry infrastructure
remain external and MUST NOT be introduced here.

The implementation separates Device facts, Platform software stacks and Skill-owned workflows/contracts:

| Path | Responsibility |
| --- | --- |
| `devices/AGENTS.md` | Generated device directory index linking each device knowledge entry |
| `devices/<board>/` | Device knowledge, references, assets and native implementation; AGENTS is the entry and Device contract owner |
| `devices/<board>/targets/` | Native BSP, examples and board configuration directly under one Target root; target.json declares its software stack |
| `platforms/esp-idf/components/` | Components shared by ESP-IDF boards |
| `platforms/<stack>/` | Shared software-stack/runtime adapters, build helpers and reusable components; no board directories |
| `skills/` | Flat installable Skills: codecraft-development-* development workflows and codecraft-capabilities-* cloud API clients |
| `tools/skill_packages/` | Skill package maintenance source, package release records, immutable import provenance and release tooling |
| `tools/cli/` | Standalone Rust creator CLI and shared orchestration |
| `tools/cli/src/maintenance_*.rs` | Repository checks, evidence, CI and package maintenance |
| `tests/` | All repository-maintained tests, fixtures and mocks, grouped by Device, Platform, Skill or Tool owner; see tests/AGENTS.md |
| `tests/devices/<board>/` | Retained observations, source-bound evidence records and sanitized original logs |
| `tests/devices/<board>/targets/` | Board-specific host tests, native fixtures and test doubles |
| `projects/<name>/` | Ignored private application exports, briefs and local locks; never reference dependencies |
| `docs/`, `.github/` | Reference documentation, contribution guidance, and CI |

Choose the platform by software stack, not chip name. An ESP32 using Arduino
belongs to the Arduino workflow. MUST NOT substitute another platform's build
or deployment procedure.

Device AGENTS.md owns authored hardware knowledge, specifications and acceptance
procedures. Follow [Device documents and shared references][device-documents] for
its format, source validation and ownership. Native code and board configuration
live directly in targets/. Apply platforms/<stack>/AGENTS.md to that stack's
Targets. Verification comes from source-bound evidence, never from a directory
listing or README claim.

### Task selection and required AI skills

1. Start with `git status --short --branch`; preserve existing changes.
2. Determine whether the task is inspection, project development/build/device
   use, or a repository contribution. For mixed tasks, apply contribution
   rules to the repository changes. Use `codecraft discover` to find candidates and `codecraft context --target <platform>/<device>` to collect their reference closure. Read-only questions do not require
   installing toolchains or running builds.
3. For development, identify the intended output location, board, software
   stack, and relevant hardware revision before relying on those choices.
   Prefer ignored projects/<name> workspaces for user applications; explicit
   external directories remain supported. Keep private code out of reference
   source directories unless the user requests an example or contribution.
   See docs/development/project-workspaces.md.
4. Load the matching routing-table entries and local rules before editing or
   executing a platform operation. MUST NOT load every README by default.
5. Select validation from the actual diff using
   [Required validation and delivery](#required-validation-and-delivery).
   Unchanged firmware and native build inputs do not require compilation or
   native toolchain setup.

Before AI-assisted development, all five skills MUST be installed and available:
`codecraft-development-develop`, `codecraft-development-setup`, `codecraft-development-build`,
`codecraft-development-device-test`, and `codecraft-development-debug`. Maintained sources are in
`skills/`. If any is missing, choose an installation method appropriate to the
environment, install it, and verify availability without waiting for the user
to request installation. No client or installation directory is mandatory.

The discovery and validation workflows also have `codecraft-development-discover` and `codecraft-development-validate` Skills. Use only the Skills matching the task. Availability does not require loading
all five, and installation does not authorize flashing, Git writes, or
publishing. A missing toolchain must be handled through `codecraft-development-setup`;
a failed operation must be diagnosed through `codecraft-development-debug` before retrying.

### Task-specific context routing

| Task | Read before the relevant operation |
| --- | --- |
| New application ideation | `skills/codecraft-development-ideate/SKILL.md`, `docs/development/project-workspaces.md`, then selected Device/Target context |
| Code change | `docs/development/ai-guide.md`, relevant headers and neighboring implementation |
| Environment bootstrap or missing toolchain | `skills/codecraft-development-setup/SKILL.md`, Target manifest and device knowledge entry |
| ESP-IDF board work | `devices/<board>/AGENTS.md`, `platforms/esp-idf/AGENTS.md`, `devices/<board>/targets/target.json`, matching entry in `docs/vendor-sources.md` |
| Linux board work | `devices/<board>/AGENTS.md`, `platforms/linux/AGENTS.md`, `devices/<board>/targets/target.json`, matching entry in `docs/vendor-sources.md` |
| Arduino board work | `devices/<board>/AGENTS.md`, `platforms/arduino/AGENTS.md`, `devices/<board>/targets/target.json`, matching entry in `docs/vendor-sources.md` |
| New board | Part II, `docs/development/ai-guide.md` onboarding section, platform rules, `docs/vendor-sources.md` |
| Build, tests, dependencies, partitions | `skills/codecraft-development-build/SKILL.md`, device knowledge entry, Target manifest and platform configuration (`sdkconfig.defaults`, `target.json`, as applicable) |
| Firmware image planning, merging or verification | `docs/development/firmware-images.md`, `skills/codecraft-development-build/SKILL.md`, Target firmware profile and Platform implementation |
| Flashing or on-device testing | `skills/codecraft-development-device-test/SKILL.md`, `tests/AGENTS.md`, board deployment instructions |
| Diagnosis | `skills/codecraft-development-debug/SKILL.md`, failing command/output and affected board context |
| Documentation or Agent rules | Part II, `docs/README.md`, affected rules and their referenced files |
| Skills or validation changes | `skills/README.md`, `skills/validation.md`, `docs/development/testing.md`, affected Skills and validation implementation |
| SenseCraft platform operation or external CLI setup | `docs/development/sensecraft-cli.md`, the official installed `sensecraft-cli/SKILL.md`, and its task-specific reference |
| CodeCraft cloud API operation | The matching `skills/codecraft-capabilities-{llm,asr,tts,image-generation}/SKILL.md` and its API reference; use only existing task authorization |
| Repository tooling or package maintenance | `tools/AGENTS.md`, affected tool modules and their tests; keep the Rust `codecraft` binary as the public entry |
| Commit or PR | `docs/development/ai-guide.md`, `.github/CONTRIBUTING.md`, `.github/PULL_REQUEST_TEMPLATE.md` |
| Upload, online flashing, or sharing firmware | The CodeCraft Hub section below and the linked CLI documentation |

### Hardware facts and uncertainty

- Follow this priority: **vendor specifications and measured results → board
  pins header → BSP implementation → hardware guide → README/demo code**.
  Consult `docs/vendor-sources.md` and the underlying source; a link alone
  does not verify the claim.
- Every hardware fact introduced or changed in code MUST have a nearby source
  comment. References must identify the source file/revision or measurement
  sufficiently to check the claim. MUST NOT invent pins, addresses, polarities,
  voltage limits, memory sizes, timing requirements, or device identities.
- Do not extrapolate a fact from a similar board, chip family, or different
  hardware revision. Missing information remains `unknown`; absence of
  evidence is not proof of compatibility or incompatibility.
- If required facts are missing or sources conflict, stop the operations that
  depend on them, record the gap/conflict, and seek the missing evidence or user
  decision. Continue independent work. MUST NOT choose a convenient value to
  make a build or test pass.
- Verify the declared toolchain, dependencies, and relevant board revision
  before making a compatibility claim. A build on another version is evidence
  only for the version actually used.

### Operation boundaries

- MUST NOT overwrite, revert, clean, or include unrelated user changes. Read
  an already-modified file before editing it and preserve changes outside the
  requested scope.
- Device writes require explicit authorization for the current session and
  confirmation of the target board and port/drive. Existing authorization
  remains valid within its scope; do not ask again for the same authorized
  action. A generic serial port or removable drive is not proof of identity.
- Flash readback or backup is not a prerequisite for flashing. Flashing does
  not guarantee preservation of user data and does not authorize full-chip
  erase; erase requires an explicit user request.
- Commit, push, PR, merge, and publication actions must each stay within the
  granted scope. Authorization for one does not imply the next. A build or
  repository edit does not authorize a device write or publication.
- MUST NOT expose or commit credentials, device secrets, private keys, personal
  data, or unsanitized logs. Generated firmware and temporary build output must
  remain outside versioned source.
- Report what actually ran. Metadata checks, compilation, file transfer,
  simulation, and physical behavior are distinct results; none proves the
  next. MUST NOT present a stale binary as the output of a failed new build.

### External SenseCraft tooling

For SenseCraft platform operations, use the official external
[`@seeed-studio/sensecraft-cli`](https://github.com/Seeed-Solution/sensecraft-cli)
and its bundled Agent Skill. Follow [setup and routing](docs/development/sensecraft-cli.md).
The repository owns installation/use guidance, not this tool's authentication,
HTTP implementation or command wrappers. Install it only for an explicitly
requested setup or a task that needs it. Native builds do not require it.
Repository operations still use `codecraft`; this separately maintained
tool uses its own CLI. Check its actual version/help before relying on commands.

### CodeCraft Hub

[CodeCraft Hub](https://codecraft.seeed.cc/hub) is a
separate firmware-sharing platform. This repository does not implement it.

Only when the user requests upload, online flashing, or sharing, use
[`@seeed-studio/sensecraft-cli`](https://github.com/Seeed-Solution/sensecraft-cli)
and its official nested create-app Skill to prepare the required package and
perform the requested platform operation. Follow the
[external tool guide](docs/development/sensecraft-cli.md) to install the published
npm package and bundled Skill; do not clone or build the CLI repository for
installation. An ordinary build, local flash, or board
change is not an upload request. Hardware facts remain owned by this repository;
platform success must not change the hardware verification status.


### Discovery and reference indexes

- [Device directory](devices/AGENTS.md)
- [Native interfaces and current evidence](docs/inventory.md)
- [Skill contracts and workflows](skills/AGENTS.md)
- [Documentation and authoritative guides](docs/README.md)

## Part II: Developer contribution constraints

### Change scope and implementation ownership

Before editing, identify the affected files, boards/platforms, source of truth,
and required validation. A contribution MUST solve the requested problem at
the owning layer and preserve unrelated behavior.

- Native board-specific reusable logic belongs in devices/<board>/targets/bsp. Device
  hardware principles, stack-independent implementation references and assets
  belong in devices/<board>/. A Target consuming reference source MUST declare
  it in its dependency graph instead of copying it. ESP-IDF components
  shared by two or more boards belong in `platforms/esp-idf/components/`. Pages, state
  machines, animations, and application tasks belong in examples/applications.
- A Target's logical ID is <stack>/<device>; its only physical source root is
  devices/<device>/targets/. Keep board-specific native configuration
  there and shared build helpers in platforms/<stack>/. MUST NOT duplicate a
  board under platforms/, copy shared components into a Device, or add forwarding
  directories. Declare the software stack in target.json, without an additional
  platform subdirectory inside targets/. Preserve each stack's native conventions.
- Keep standalone examples independently buildable. Keep testable logic
  independent of platform runtimes where possible; behavior changes require
  relevant host tests when that behavior can be tested on the host.
- MUST NOT introduce cloud runtime or account/platform implementation into
  hardware support. MUST NOT force a shared code ABI across software stacks.
- For a new board or changed hardware fact, establish the vendor/source record
  first, then update the owning header/BSP/profile and its documentation.
  Record cited source notes, implementation references and conflicts in the authored Device AGENTS.md section; retain supporting reference material in reference/ when needed.
- New devices MUST provide an authored AGENTS.md following [Device documents and shared references][device-documents], a Target manifest and native examples. Native interfaces cite implementation paths. Cloud consumption contracts belong to their standalone Skills; GPIO, display and audio remain native interfaces. Regenerate derived indexes and add relevant positive/negative checks when schema or adapter behavior changes.
- Toolchain, dependency, partition, and persistent-data changes MUST state
  compatibility and recovery implications. A directory move must not silently
  change a toolchain, hardware value, or firmware behavior.

### Documentation and rule changes

- Maintained `README.md` files MUST have a Simplified Chinese
  `README.zh_CN.md` counterpart with reciprocal links and equivalent commands,
  requirements and exceptions. All other engineering documents MUST use one
  English `.md` file: AGENTS, CLAUDE, SKILL, API references, architecture,
  contribution rules, plans, TODOs and reports MUST NOT have Chinese copies.
  Imported original evidence and third-party upstream documentation retain
  their original language and attribution. Structured localized labels may
  support README views; physical facts still have one canonical typed value.
  Generated engineering views and exported application rules use English;
  generated application README files retain both languages.
- State current behavior and proposed architecture separately. An architecture
  proposal does not establish implementation. Only document a generated
  index or automatic check as implemented when its producer/check exists.
- Maintain a fact at its owning source and reference or generate other views.
  MUST NOT hand-edit files or regions explicitly marked as generated; change
  their source and rerun the generator. Existing manually maintained indexes
  remain manual until a generator replaces them.
- Every rule MUST identify its applicable task and required action. If a
  prerequisite is missing, specify the blocked step and report it; do not add
  blanket approval requests or block independent work. Examples and translated
  rules MUST NOT contradict the governing requirement.
- Rule changes MUST review affected Skills, local rules, linked guidance, and
  validation behavior for contradictions. MUST NOT remove a requirement merely
  to make a failing check appear successful. Report documentation/implementation
  gaps explicitly rather than claiming new enforcement exists.
- A structural change MUST update references, commands, tests, CI and affected
  documentation together. Remove superseded implementations and entry points;
  MUST NOT retain forwarding modules, mixed schema readers or transition-only tests.

### Enforced ownership and source closure

- Device specifications MUST maintain one typed value per fact with independent source assertions. Follow [Device documents and shared references][device-documents] for the Markdown format and parsed schema; do not maintain translated or generated copies of hardware facts.
- Device Markdown is an authored source. Generation MUST leave it unchanged; exports and source graphs hash its complete bytes. Facts require independent cited notes or native sources. Keep procedures in that document and observations in the central evidence area; follow [Device documents and shared references][device-documents].
- Target metadata MUST declare native interfaces, common inputs, per-example inputs and dependencies. The Device-to-Target index MUST be derived from Target.device; duplicate backlinks are forbidden. All Target records MUST use schema v3 with an explicit variants object. Platform and adapter protocol versions remain independent.
- Native build/configuration logic MUST live in platforms/<stack>. Shared repository tools MUST NOT branch on stack IDs to implement native operations; the Rust registration module may select a platform-owned adapter.
- Context, project export and evidence MUST use the same transitive dependency graph. Never filter out missing dependencies to make export pass. Creator graphs MUST include every native application input and bind compiled CLI identity separately in the lock. Maintenance candidates bind their compiled CLI identity and complete validation sources.
- Application briefs MUST be owned by project.yaml. Render the marked application AGENTS region on creation/relocking, preserve custom rules outside it, and leave missing intent/BOM values unknown. Private projects MUST NOT enter reference discovery, test collection or source dependencies.
- A project lock MUST preserve initial import provenance separately from explicitly reviewed application updates. Builds MUST check the actual toolchain against the current lock.
- Evidence MUST name complete expected cases and the source digest, and live in tests/devices/<board>/. Preserve original log bytes, timestamps and observed results; storage changes MUST NOT refresh historical source digests. Changed source/adapter/contracts invalidate prior verification. Physical promotion additionally requires matching revision and named acceptance observations.
- Cloud operation contracts MUST live in the consuming Skill package's contract.json. Device peripherals and native interfaces MUST NOT become cloud contracts. This repository MUST NOT implement the cloud service.
- Skill contract validation in tools/cli/src/maintenance_contracts.rs MUST NOT import or construct the hardware Catalog. Native-only context, export and project checks MUST NOT load cloud packages or contract code. Explicit Skill selections add only their complete package and verification graph. Stable semantics, service policy and API versions remain separately reviewed. Policy may only narrow schemas; defaults require explicit permission. Follow docs/architecture/skill-contracts.md.
- Cloud Skill packages MUST run independently after copying one directory. Runtime imports MUST remain inside the package. Package release records in tools/skill_packages/packages MUST pin every package file, including contract.json, and preserve original provenance. Keep inventories outside packages to avoid self-hashing. Validation MUST use offline fixtures and synthetic credentials, never real services.
- Bundle changes MUST use candidate/check/review/apply in docs/development/skill-releases.md. Apply MUST name the reviewed digest, rerun isolated conformance and reject source drift. Package, API, semantic contract and policy versions MUST remain separately reviewed; original import provenance MUST NOT be rewritten. Packaging MUST NOT imply installation or publication.
- ESP-IDF examples MUST keep native BSP composition, Target example scope and managed dependency locks aligned. Public headers stay in include/ and private implementation in src/. Validate actual build-source selection and portable exports; changing source layout MUST NOT silently change hardware configuration or dependency versions. See `docs/development/native-compositions.md`.
- Hardware variants MUST come from sourced Device facts and explicit Target/native bindings. Carry the selected variant through context, export, locks, native build cases and physical evidence. A declared build default MUST NOT imply panel detection or revision compatibility; one variant's observations MUST NOT promote another. Unknown revision mappings remain unresolved. See `docs/development/hardware-variants.md`.
- All repository-maintained tests, fixtures and mocks MUST live under tests/, grouped by source owner. MUST NOT place test directories inside devices/, platforms/, skills/ or tools/. Use the recursive public runner for Python suites and the explicit Cargo integration targets for Rust suites; each test MUST run once in its owning suite. Physical evidence and logs are not executable test suites. Follow `docs/development/testing.md` for the authoritative ownership map, filtering and execution rules.
- Document and invoke creator operations through the standalone Rust `codecraft` CLI in tools/cli. Native implementations belong in platforms/<stack>/native.rs. Creator exports carry portable guidance, native application inputs and a separately locked CLI identity; they MUST NOT copy the CLI implementation. CLI contract 4 and creator profile/lock/graph schema 4 use explicit --skills selections. Historical evidence retains its original schema and is archived without promotion; it is not a second executable project implementation. Use `codecraft maintenance` for repository audits. Follow docs/development/rust-cli.md. Build the CLI only after Rust/dependency/compiler/target changes; data and application edits do not rebuild it.

### Required validation and delivery

During contribution work, inspect the final diff and select checks by actual
impact. Run the smallest relevant checks first. A commit, push, merge or
documentation change does not by itself require firmware compilation.

| Change scope | Required validation |
| --- | --- |
| Documentation, README, Agent rules or Skill instructions only | Check paths, commands, language policy and generation drift; run `validate --static`. No native builds. |
| Host tooling, cloud clients, discovery, project briefs, locks or CI with unchanged native build inputs and behavior | Run affected host/contract/export tests and `validate --static`, covering each affected platform. No native builds. |
| Firmware or native build inputs/behavior | Run focused host checks, `validate --static` and native validation for the affected Targets, examples and variants. Include `--projects` when native export contents or build behavior changes. |
| Explicit full-build request or native changes affecting every Target | Run the complete gate and inspect every substep. |

Native build impact includes firmware sources/headers, compiled assets, source
selection, SDK/board configuration, partitions/linker scripts, toolchain or
dependency versions, and adapter/export changes that alter those inputs or the
native build command. Judge the changed behavior, not just the directory or file
extension. Host-only orchestration, provenance, cache and documentation changes
do not become firmware changes merely because they share a source graph.

When firmware and native build inputs/behavior are unchanged, Agents MUST NOT
run native builds, set up native toolchains, or use the no-option `validate`
command unless explicitly requested by the user.

Offline evidence checks and toolchain-version parsing changes may use focused
host tests when declared toolchain versions, native inputs and emitted build
commands remain unchanged. Cover accepted/rejected identities and the affected
build preflight paths; do not infer a fresh firmware build from these tests.
Changing a declared toolchain, native input or emitted command still requires
native validation. Native commands in local
Platform/example guidance are conditional on this scope selection. For merges,
compare the final native inputs with validated parent inputs; rebuild only newly affected
combinations. A new commit or changed source digest alone is not a rebuild
trigger. Keep evidence freshness rules: do not relabel stale evidence as current
or claim a new build passed when compilation was skipped.

This is a check-selection policy for Agents and contributors. The CLI does not
automatically inspect the Git diff; no-option `validate` still runs all Targets
and clean project builds. CI keeps the static/host gate and selects native Targets
with `codecraft maintenance ci plan`; its fixed result job checks every selected
platform. See [CI selection](docs/development/ci.md) for event baselines, dependency
ownership, conservative fallbacks and explicit skips. This does not replace
reviewing an uncommitted local diff under the scope rules above.
Select any required native checks by the actual platform:

| Command | What it establishes |
| --- | --- |
| `codecraft validate --static` | Repository checks and host tests |
| `codecraft validate --target esp-idf/<board>` | ESP-IDF example builds with an activated, board-appropriate IDF |
| `codecraft validate --target linux/<board>` | Linux source checks; inspect whether overlay compilation ran |
| `codecraft validate --target arduino/<board>` | Arduino sketches compiled with the declared core/libraries |
| `codecraft validate` | Explicit complete-gate run under the scope rules above; includes native and clean project builds |

Documentation/rule changes additionally require checking referenced paths,
commands, README language equivalence, English-only engineering documents and applicability. Shared tools, discovery,
or CI changes require coverage of every affected platform; native compilation
is required only for actual native build impact as defined above. Pin, display, bus,
and power changes require board-revision-specific physical evidence before
being described as ready to merge, as required by `.github/CONTRIBUTING.md`.

If a required check cannot run, report its command, missing prerequisite, and
unverified scope. Do not silently weaken it or claim the complete gate passed.
An exit code of zero does not turn skipped substeps into successful checks.
Use these result meanings in reports; the CLI emits a versioned JSON envelope, typed errors and these distinct statuses:

- `passed`: the specified check actually ran and met its acceptance criteria.
- `failed`: it ran and failed.
- `skipped`: it was intentionally omitted; state why and what remains unverified.
- `not_run`: it did not run, including missing toolchain/device prerequisites.
- `not_supported`: the operation is explicitly unsupported; missing evidence or
  a missing local tool is not proof of non-support.

Final delivery MUST separately report:

1. Changes and scope, including compatibility implications where applicable.
2. Static/host check results and the complete-gate outcome, or `skipped` with
   the scope reason when the complete gate was not selected.
3. Build/check results per affected board and actual toolchain. If none are
   affected, say so; do not imply firmware was built.
4. On-device verification and evidence, or an explicit statement it was not
   performed. Record board revision, tested behavior, environment, and result.
5. Failed, skipped, unsupported, and unperformed checks, and remaining limits.

Review the final diff against the starting worktree. A passing check does not
authorize a commit, publication, or a stronger hardware-support claim. When
commits/PRs are requested, follow `.github/CONTRIBUTING.md` and the PR template;
use English imperative Conventional Commit subjects and include only the
authorized changes.

[device-documents]: docs/development/device-documents.md

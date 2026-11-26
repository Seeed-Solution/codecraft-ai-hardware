<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# Agent Skills

Skills own hardware development workflows and standalone cloud clients.
Device facts/native implementations belong in devices/; shared stacks belong in
platforms/. Read [Skill rules](AGENTS.md) and [contract ownership](../docs/architecture/skill-contracts.md).

| Category | Package names | Purpose |
| --- | --- | --- |
| Development workflows | `codecraft-development-*` | Discover, develop, configure, build, debug and validate hardware applications |
| Cloud capabilities | `codecraft-capabilities-*` | ASR, TTS, text generation and image generation through the CodeCraft Device API |

Each Skill is a direct child directory with its own SKILL.md. Copy one complete
Skill directory to install it. Repository tooling lives in
[tools/](../tools/AGENTS.md), outside the installable Skill collection.

## Hardware development

Use codecraft-development-develop as the general application entry. Follow the
[shared agent setup guide](../docs/development/agent-client-setup.md) for native
Skill discovery, a project-rule pointer or explicit document reading. Development
Skills need the selected repository or exported application for hardware and
workflow references; their installation directory is not that workspace. Other
independent cloud and reCamera packages retain their own package-local guidance.

The required workflows are codecraft-development-develop, codecraft-development-setup, codecraft-development-build,
codecraft-development-device-test and codecraft-development-debug. Use codecraft-development-ideate for application
briefs, plus codecraft-development-discover and codecraft-development-validate as needed. Shared Device
creator orchestration lives in tools/cli/; repository audits live in tools/cli/. Native commands remain in each
Platform. The [portable workflow](../docs/development/application-workflow.md)
and selected Platform/workflow guidance are exported and locked with applications.

## reCamera Pro SDK applications

Use [codecraft-development-recamera-pysdk](codecraft-development-recamera-pysdk/SKILL.md)
for reCamera Pro / RV1126B Python SDK apps, offline validation and App Center
packaging. Copy the whole Skill directory for independent use. It pins the
official SDK builder and API references; it does not register a native Target
or establish physical board support. Device upload, installation and testing
remain separate authorized operations. See its [provenance](codecraft-development-recamera-pysdk/references/provenance.md).
Reference generation and reviewed package inventories live in repository tools,
outside the installed Skill. See [SDK maintenance](../docs/development/recamera-sdk-maintenance.md)
and the [optimization TODOs](../docs/development/recamera-sdk-todo.md).

## External SenseCraft CLI

Use the official Skill bundled with @seeed-studio/sensecraft-cli for platform
authentication, managed devices, telemetry, Hub, files, model libraries and cloud
deployment. Follow [setup and routing](../docs/development/sensecraft-cli.md).
Install it outside this repository only when needed. Native development does
not require it, and installation never authorizes a platform operation.

## Cloud capabilities

| Skill | Operation |
| --- | --- |
| [codecraft-capabilities-llm](codecraft-capabilities-llm/SKILL.md) | Text generation and SSE |
| [codecraft-capabilities-asr](codecraft-capabilities-asr/SKILL.md) | WAV transcription, realtime ASR/STT, status and recovery |
| [codecraft-capabilities-tts](codecraft-capabilities-tts/SKILL.md) | WAV synthesis and optional archival |
| [codecraft-capabilities-image-generation](codecraft-capabilities-image-generation/SKILL.md) | Configurable image dimensions and optional archival |

Each package contains contract.json, SKILL.md, public API/onboarding/native
integration references, a portable C authentication header, Agent metadata and
package-local scripts. Copy the complete directory for independent use. MCU/Linux
integration uses the device networking stack; Python and SenseCraft CLI are
workstation tools, not firmware dependencies. Optional helpers use Python 3.10+;
realtime streaming needs websockets==15.0.1, and image decoding needs Python
3.11+ with Pillow==13.0.0. Follow each package's environment setup. No hardware
catalog, sibling package or repository CLI is required. Read its contract and API reference for exact limits,
defaults, billing declarations and unknowns; the [generated inventory](../docs/skill-inventory.json)
provides IDs, operations and separate versions.

Each package's onboarding reference explains API-origin lookup, EUI/DeviceKey
retrieval, private import and access checking; see the
[LLM onboarding reference](codecraft-capabilities-llm/references/onboarding.md) for the common
flow. Reuse existing configuration. The documented CLI release does not export a
CodeCraft Device API origin; its platform gateway alone does not establish that
service mapping. Missing-origin work must preserve existing credentials and
must not block independent local import or configured CLI provisioning.

Before an authorized API task, confirm network/TLS, privacy and credential-reference
availability. Set CODECRAFT_API_ORIGIN and CODECRAFT_DEVICE_FILE according to the
package reference. Keep real credentials, inputs and outputs outside the package.
Service routing is fixed: do not send model/provider/voice overrides. LLM
completion requires a normal finish status; realtime recovery uses the original
idempotency key and never automatically replays audio. Media is local by default; archive only when explicitly required. A failed archive
retains the local output and does not authorize another generation request.

Run the selected Skill entry directly, for example from its installed directory:

```sh
python3 scripts/transcribe.py --file /work/recording.wav
```

The client validates actual files, authentication, protocol and outputs. A nonzero
exit remains failure even if partial text or a valid local file exists. No
automatic charged retries are permitted. Host execution does not prove device
integration or live service acceptance.

## Free allowance

Each device receives the following free allowance every month, shared by the AI features below. Each row assumes the entire unused monthly device allowance is spent on that feature; **the rows are alternatives and must not be added together**.

| Feature | Per device per month | Per user per month, total across 5 devices |
| --- | --- | --- |
| File ASR / STT (up to 60 seconds per call) | **62 calls** | **310 calls** |
| Realtime ASR (up to 60 seconds per session) | **62 sessions** | **310 sessions** |
| TTS (up to 500 input characters per call) | **62 calls** | **310 calls** |
| Image generation | **6 images** | **30 images** |
| LLM: 100 input + 50 output tokens | **About 41 calls** | **About 205 calls** |
| LLM: 100 input + 100 output tokens | **About 31 calls** | **About 155 calls** |
| LLM: 200 input + 100 output tokens | **About 20 calls** | **About 100 calls** |

The five-device column adds the complete calls available on five separate devices; it is not a pooled account balance. LLM counts are estimates for the stated token usage. Actual usage includes the full request context and generated output; longer conversations or replies reduce the number of calls. Using one feature reduces the allowance available for the others. The service manages monthly renewal; checking device access again does not replenish the allowance.

## Optional project composition

`new --skills <selection.json>` and `context --skills <selection.json>` consume
an explicit list such as:

```json
[
  {
    "skill": "codecraft-capabilities-asr",
    "configuration": {},
    "environment": {
      "network": true,
      "tls": true,
      "privacy_approved": true,
      "environment_refs": ["CODECRAFT_API_ORIGIN"],
      "secret_refs": ["CODECRAFT_DEVICE_FILE"]
    },
    "limits": {}
  }
]
```

These are confirmed conditions/reference names, not credentials or API permission.
Missing conditions and constrained unknown resources fail selection. A native-only
project includes no cloud contracts/clients; explicit selection adds the exact
package, external release record and provenance to the source graph and lock.

## Maintenance

Release records in tools/skill_packages/packages/ pin every distributed file,
including binary assets and contract.json. tools/skill_packages/import.json
preserves original provenance; tools/skill_packages/upstream.json records the
separately reviewed upstream source snapshot. Upstream paths retain their
original names even when the maintained package has a different name.
Maintain shared runtime helpers under tools/skill_packages/ and use the
[candidate/check/review/apply workflow](../docs/development/skill-releases.md).
Package, API, semantic and policy versions are reviewed independently. Apply names
the exact digest, reruns isolated conformance and rejects source drift. Packaging
never installs or publishes a Skill.

Activate the [host test environment](../docs/development/testing.md) before the
following repository-root checks; dependency installation makes no API calls.

```sh
codecraft maintenance check-cloud-skills
codecraft maintenance test --path tests/tools/skill_packages/test_protocol.py
codecraft validate --static
```

check-cloud-skills verifies repository packages; --skills-root checks installed
copies. Missing, extra and symlink files fail. Caches are excluded; CLI checks use
fresh pinned bytes without inherited PYTHONPATH. Per-Skill tests live under
tests/skills/<skill-id>/; shared contracts live under tests/tools/cli/ and
installation/protocol/releases under tests/tools/skill_packages/. Tests use loopback fixtures and
synthetic credentials only. Real API acceptance remains [open](../docs/development/optimization-plan.md).

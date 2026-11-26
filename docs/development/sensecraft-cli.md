# External SenseCraft CLI

[sensecraft-cli](https://github.com/Seeed-Solution/sensecraft-cli) is maintained by Seeed-Solution. Use its published [npm package](https://www.npmjs.com/package/@seeed-studio/sensecraft-cli) and official bundled Agent Skill for SenseCraft platform tasks. This repository supplies setup and task-routing guidance; command implementation, authentication and platform HTTP calls belong to that external project.

The verified package version is **0.5.0**. Node.js 16+ is required. Installation downloads the platform binary; native firmware development does not require this optional Agent-host tool.

## Install and select the executable

Invoke the selected release directly without replacing another global installation:

```sh
npx --yes @seeed-studio/sensecraft-cli@0.5.0 --version
npx --yes @seeed-studio/sensecraft-cli@0.5.0 --help
```

For a persistent global installation, the upstream-supported command is:

```sh
npm install -g @seeed-studio/sensecraft-cli@0.5.0
sensecraft-cli --version
```

A scoped installation also works with `npm install --prefix <external-tools-directory> --save-exact @seeed-studio/sensecraft-cli@0.5.0`; invoke its `node_modules/.bin/sensecraft-cli` directly. Keep that directory outside the reference repository. Verify the actual executable version, since an existing global command can resolve to a different release. Review the new release's help and Skill before changing the selected version. See the [official install guide](https://github.com/Seeed-Solution/sensecraft-cli/blob/main/INSTALL.md) for supported platforms and configuration locations.

## Load the official Skill

After npm installation, the entry is `@seeed-studio/sensecraft-cli/skills/sensecraft-cli/SKILL.md` under the selected installation's `node_modules`. For a global installation, `npm root -g` locates that parent. Link or copy the complete `skills/sensecraft-cli` directory into the Agent host's supported user Skill directory, keeping its `references/` and nested `create-app/` intact. Use the Skill shipped with the selected executable version. Do not maintain another copy of this Skill or its implementation in the hardware repository.

Read that entry before a SenseCraft task, then its product-specific reference. A request to turn a local project into a Hub app uses the official nested `create-app/SKILL.md`. Repository commands remain `codecraft`; the separately maintained external tool is invoked through its own CLI. External installation and Skill registration do not authorize login, cloud writes or hardware writes.

## Task routing

| Task | Official command group |
| --- | --- |
| Devices, groups and telemetry | `device`, `group`, `data` |
| CodeCraft Hub applications | `app`; official create-app Skill for local-project handoff |
| Platform model library | `model` |
| Files and firmware upload | `file`; `firmware` supplies upload advice |
| Cloud-device deployment | `cloud` |
| reCamera platform/deploy-page assistance | `recamera` |
| Account setup for an authorized platform task | `auth` |

Use the selected version's `--help` and the official Skill for arguments, authentication and result handling. Keep credentials in the external tool's supported configuration location, outside source, project profiles and locks. Native board builds, source facts, toolchain selection and physical acceptance remain governed by the hardware repository. Platform upload advice is not proof of hardware compatibility, a successful build or a physical flash.

The verified command list does not contain replacements for the CodeCraft LLM, ASR, TTS or image-generation APIs. In particular, `image --help` describes a prompt for the Agent to write SVG, not the existing inference service. The four current CodeCraft client Skills and their package-owned contracts therefore remain separate. Introducing this platform tool does not register a synthetic service implementation or claim those APIs have migrated.

## CodeCraft Device API onboarding

Each standalone client maintains its complete onboarding flow inside its own
package; start with the selected Skill, for example
[codecraft-capabilities-llm onboarding](../../skills/codecraft-capabilities-llm/references/onboarding.md).
That flow covers the three runtime inputs: API origin, EUI and DeviceKey.
Existing configurations do not require another account login or device creation.
The CLI supplies development-device credentials; local import is independent of
CodeCraft API availability.

The documented 0.5.0 profile summary and devkit outputs do not export a CodeCraft
Device API origin. A platform gateway used for account-authenticated /openapi
calls does not establish the Device-authenticated /deviceapi route. Until the
external CLI publishes a service-specific endpoint contract, obtain the matching
origin from the service's published environment configuration. Do not add a
repository CLI proxy, parse private CLI configuration or guess a domain.
Automatic discovery remains [OPT-22](optimization-plan.md), not an implemented
Skill feature.

## Offline verification

On 2026-11-18, the published 0.5.0 executable passed 12 invocations: `--version`, top-level `--help`, and help for `auth`, `app`, `model`, `file`, `cloud`, `device`, `data`, `group`, `recamera` and `image`. The official packaged Skill was installed into the Agent's user discovery directory. Verification used an empty temporary `SENSECRAFT_CONFIG_FILE`, with `SENSECRAFT_CLI_NO_UPDATE_NOTIFIER=1` and `SENSECRAFT_CLI_NO_ACTIVITY_NOTIFIER=1`. No login, business API, upload, publication or device operation was executed.

Ordinary repository validation must remain offline and must not install npm dependencies, inspect real credentials or call SenseCraft APIs. The external CLI owns its own tests; this repository does not duplicate its command implementation or introduce a forwarding CLI. The reviewed integration scope and remaining V0.2 work are tracked in the [TODO ledger](v0.2-todo.md).

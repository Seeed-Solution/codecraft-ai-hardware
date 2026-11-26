<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# CodeCraft AI Hardware

**Build and customize Seeed hardware projects with your AI coding assistant.**

CodeCraft AI Hardware brings together device knowledge, native board support packages (BSPs), examples and Agent Skills for AI-assisted embedded development with **ESP-IDF, Arduino and Linux**. Start from an existing example, create your own project, and give your coding assistant the board context and development workflow it needs.

[Choose an example](#choose-a-starting-example) · [Quick start](#quick-start) · [Devices](devices/AGENTS.md) · [Agent Skills](skills/README.md) · [Documentation](docs/README.md) · [简体中文](README.zh_CN.md)

## What you can do

- **Start with board context.** Find device references, native interfaces, toolchain requirements and known limitations before changing firmware.
- **Make an example your own.** Export a private application with its BSP, dependencies and source lock; develop with the board's native software stack.
- **Work with an AI coding assistant.** Use Skills for setup, development, builds, debugging and authorized device testing.
- **Add optional cloud features.** Standalone Skills extend your project with capabilities such as text generation, speech recognition, speech synthesis and image generation. Local native development does not require a cloud account.

## Choose a starting example

These are existing source examples to build on. Follow each Device entry for hardware revision requirements and the Target guide for setup.

| Starting point | Device and software stack | Example source | Setup |
| --- | --- | --- | --- |
| Explore an e-paper display | [reTerminal E1001](devices/reterminal-e1001/AGENTS.md) · ESP-IDF | [epd_hello](devices/reterminal-e1001/targets/examples/epd_hello) | [Target guide](devices/reterminal-e1001/targets/README.md) |
| Explore a display and buttons | [Wio Terminal](devices/wio-terminal/AGENTS.md) · Arduino | [display_buttons](devices/wio-terminal/targets/examples/display_buttons) | [Target guide](devices/wio-terminal/targets/README.md) |
| Start with a button and LED | [Grove Beginner Kit](devices/grove-beginner-kit/AGENTS.md) · Arduino | [button_led](devices/grove-beginner-kit/targets/examples/button_led) | [Target guide](devices/grove-beginner-kit/targets/README.md) |

Find more boards and examples in the [device directory](devices/AGENTS.md). The generated [native interface and evidence matrix](docs/inventory.md) distinguishes implementation from build and physical verification. Current-source on-device acceptance remains unperformed; historical observations do not certify a new source version.

## Quick start

Using Codex, Claude Code, Cursor, ZCode, WorkBuddy, Muse or another coding agent?
Start with the [shared agent setup guide](docs/development/agent-client-setup.md).
It provides one reusable request for new and existing applications, with optional
native Skill or project-rule entry. Client experience testing is left to users.

### 1. Get the repository and select your board

Have Git and the standalone CodeCraft CLI available. Install the latest stable Release binary after cloning with `sh tools/cli/install.sh` (Windows: `tools/cli/install.ps1`). Use `--from-source` (`-FromSource` on Windows) only for local CLI development. Downloaded binaries require neither Python nor Rust. See [installation and updates](docs/development/rust-cli.md).

```sh
git clone https://github.com/Seeed-Solution/codecraft-ai-hardware.git
cd codecraft-ai-hardware
codecraft discover --interface display
```

The following walkthrough uses **reTerminal E1001 / ESP-IDF**. For another board, use its Target guide and declared software stack.

```sh
codecraft context --target esp-idf/reterminal-e1001
```

Read the returned context and the [E1001 setup guide](devices/reterminal-e1001/targets/README.md#toolchain). Install and activate the exact toolchain declared in [target.json](devices/reterminal-e1001/targets/target.json) before building; when working with an agent, use [codecraft-development-setup](skills/codecraft-development-setup/SKILL.md) for missing tools.

### 2. Create your own workspace

Run from the repository root:

```sh
codecraft new --target esp-idf/reterminal-e1001 --example epd_hello --output projects/my-e1001-project
codecraft verify-project projects/my-e1001-project
```

Choose an output directory that does not already exist. `projects/` is ignored by Git so your application stays separate from the reference examples; an explicit external directory also works. `project.yaml` records your selections, and `codecraft.lock` tracks imported sources and toolchain provenance. Use the installed standalone executable; project edits do not rebuild it.

### 3. Build the selected example

With the declared toolchain active, run:

```sh
codecraft lock-project projects/my-e1001-project --toolchain
codecraft build-project projects/my-e1001-project
```

The exported workspace records CLI identity and contains no CodeCraft Python runtime; from that directory, use `codecraft build-project .`. Builds do not flash the device. To test on hardware, follow the selected [Device acceptance procedure](devices/reterminal-e1001/AGENTS.md#physical-acceptance) and [codecraft-development-device-test](skills/codecraft-development-device-test/SKILL.md), with the board and port identified and device writes explicitly authorized.

For application changes, briefs, reviewed relocking and repeat builds, follow the [portable application workflow](docs/development/application-workflow.md). Use [codecraft-development-ideate](skills/codecraft-development-ideate/SKILL.md) and `new --brief <file.json>` to record purpose, users, scenarios and a bill of materials in your project.

## Develop with your AI coding assistant

Open this checkout in your coding assistant and have it read [AGENTS.md](AGENTS.md). Before AI-assisted development, install the complete directories for these five [hardware workflow Skills](skills/README.md) into your assistant's Skill discovery location: `codecraft-development-develop`, `codecraft-development-setup`, `codecraft-development-build`, `codecraft-development-device-test` and `codecraft-development-debug`. The repository contract directs the agent to install and verify any missing workflows.

For example, start a new application with this request:

```text
Read AGENTS.md and use the required CodeCraft Skills.
I have a reTerminal E1001. Help me create a private project at
projects/my-e1001-project from the epd_hello example.
Check the board context and declared toolchain, then build it.
Report the build result and any unknowns. Ask me to identify the
board and port and authorize device writes before flashing.
```

Use a fresh output path if you already followed the quick start. For an existing application, open its workspace and follow its generated `AGENTS.md`. Agent workflows cover setup, development, build, debugging and device testing; the selected Platform owns the native commands.

## Optional cloud features and firmware sharing

Standalone clients in [skills/](skills/README.md#cloud-capabilities) extend cloud capabilities with features such as LLM text generation, ASR/STT, TTS and image generation. Each client owns its `contract.json` and API reference and can be installed independently. Cloud service and account infrastructure remain external; read the selected Skill for credentials, limits and authorized API use.

See the [free allowance table](skills/README.md#free-allowance) for monthly per-device usage and monthly totals across five devices. The features share one monthly device allowance; the rows are alternative usage estimates.

To include clients in an application, CLI contract 4 uses `new --skills <selection.json>` with explicit selections; project profiles and locks use schema 4. See [project composition](skills/README.md#optional-project-composition) and the generated [Skill inventory](docs/skill-inventory.json).

[CodeCraft Hub](https://codecraft.seeed.cc/hub) is the separate firmware-sharing platform. For requested uploads, online flashing, SenseCraft device management or cloud deployment, use the external [sensecraft-cli](https://github.com/Seeed-Solution/sensecraft-cli) and its official bundled Skill. Follow [installation and task routing](docs/development/sensecraft-cli.md) for the verified version and prerequisites. Native development does not require this tool; installing a Skill or CLI does not authorize cloud operations.

## Documentation and contribution

| You want to… | Start here |
| --- | --- |
| Find device knowledge and native examples | [Device directory](devices/AGENTS.md) |
| Check implementation and evidence status | [Native interface matrix](docs/inventory.md) |
| Customize an exported project | [Application workflow](docs/development/application-workflow.md) |
| Explore hardware and cloud Skills | [Skills guide](skills/README.md) |
| Understand ownership and architecture | [Architecture](docs/architecture/overview.md) |
| Report a problem or contribute | [Issues](https://github.com/Seeed-Solution/codecraft-ai-hardware/issues) · [Contribution guide](.github/CONTRIBUTING.md) |
| Browse all guides and open work | [Documentation](docs/README.md) · [Implementation ledger](docs/development/v0.2-todo.md) |

### Repository layout

| Path | Contents |
| --- | --- |
| `devices/<device>/` | Device knowledge, references, assets and native BSPs/examples under `targets/` |
| `platforms/` | Software-stack adapters, build helpers and shared components |
| `skills/` | `codecraft-development-*` development workflows and `codecraft-capabilities-*` standalone cloud Skills |
| `tools/` | Local CLI and project orchestration in `codecraft/`; client maintenance and package release records in `skill_packages/` |
| `projects/` | Ignored private application workspaces |
| `tests/` | Tests, fixtures, mocks and retained device evidence, grouped by source owner |
| `docs/` | Engineering guidance, generated indexes and open work |

### Contributor checks

Follow the [repository contract](AGENTS.md), [maintenance entry](CONTRIBUTION.md) and [test ownership guide](docs/development/testing.md). After source metadata changes, run `codecraft maintenance generate`; then check generation drift and static/host behavior:

```sh
codecraft maintenance generate --check
codecraft validate --static
```

Documentation-only changes do not require native builds. Firmware or native build changes need scoped validation under the [validation policy](AGENTS.md#required-validation-and-delivery); no-option `codecraft validate` runs the complete software gate. Build results, simulation and physical observations are reported separately. Generated firmware and transient build output stay untracked; retained, sanitized evidence lives under `tests/devices/<device>/`.

For standalone client package changes, follow the [candidate/check/review/apply release workflow](docs/development/skill-releases.md). Packaging does not install or publish a Skill. Remaining acceptance and simulation work is tracked in the implementation ledger linked above.

## License

[Apache License 2.0](LICENSE). Retained third-party attributions: [NOTICE](NOTICE).

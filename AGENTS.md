<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# Repository Guidelines for AI Agents

This file is the only mandatory entry point for AI-assisted work in this
repository. Read task-specific documents from the routing table below; do not
load every README by default.

## What this repository is

A pure hardware capability reference for Seeed devices. It contains board
support packages, standalone demos, and verified hardware facts. It
deliberately contains **no cloud runtime**: no device pairing, no account
system, no remote protocol, no telemetry. Do not introduce any of these.

## Required AI skills

The following five skills are required for AI-assisted development in this
repository: `codecraft-develop`, `codecraft-setup`, `codecraft-build`,
`codecraft-device-test`, and `codecraft-debug`. Their maintained sources are
under `skills/`.

Before starting development, the AI must check that all five are installed and
available in its current environment. If any are missing, the AI must choose an
appropriate installation method for its tool and environment, perform the
installation, and verify availability itself. Do not wait for the user to
request installation. No particular script, installation directory, or AI
client is mandatory.

Having all five available does not mean loading or invoking all five for every
task: use only the matching skills, and retain the separate authorization
requirements for flashing, Git writes, and publishing.

## Project and safety baseline

- This is a multi-board, dual-platform repository. ESP32 firmware lives under
  `esp32/`; Linux device support lives under `linux/`. Never mix the two
  build systems.
- Hardware facts follow this priority: **vendor specifications and measured
  results → the board's pins header → BSP implementation → hardware guide →
  README/demo code**. If a task requires a hardware detail not defined by
  these sources, ask the user instead of guessing. Never invent a pin number.
- Every hardware fact in code must cite its source (vendor schematic, vendor
  BSP, or measured result) in a nearby comment. See
  [docs/vendor-sources.md](docs/vendor-sources.md).
- Preserve existing user changes. Start with `git status --short --branch`;
  never overwrite or clean unrelated files.
- Flashing new firmware does not require backing up the firmware already on
  the device; do not make a flash readback a prerequisite. This does not
  guarantee preservation of user data or authorize a full-chip erase.
- Reusable board logic belongs in the board's BSP directory or
  `esp32/components/`; pages, state machines, animations, and application
  tasks belong in the example or application.
- Never commit credentials, device secrets, private keys, personal data, or
  unsanitized logs.
- Every maintained Markdown document uses English at its default `.md` path
  and Simplified Chinese in a paired `.zh_CN.md` file. Keep both versions
  aligned and retain reciprocal language links. (Root `README` and `AGENTS`
  pairs are the current compliance baseline; translate other documents as
  they stabilize.)

## Task-specific context routing

| Task | Read before editing |
| --- | --- |
| Any code change | `docs/development/ai-guide.md`, relevant headers and neighboring implementation |
| Environment bootstrap or missing toolchain | `skills/codecraft-setup/SKILL.md` |
| ESP32 board work (pins, buses, display, audio, battery) | `esp32/AGENTS.md`, the board's `esp32/boards/<board>/README.md`, `docs/vendor-sources.md` |
| Linux board work | `linux/AGENTS.md`, the board's `linux/boards/<board>/README.md` |
| Adding a new board | `docs/development/ai-guide.md` (board onboarding section), `docs/vendor-sources.md` |
| Build, test, dependencies, partitions | `skills/codecraft-build/SKILL.md`, the board's sdkconfig defaults |
| Flashing or on-device testing | `skills/codecraft-device-test/SKILL.md` |
| Documentation | this file's bilingual rule, `docs/README.md` |
| Commit or PR | `docs/development/ai-guide.md` (delivery format section) |

## Required validation and delivery

Run the smallest relevant check while iterating, then run the complete gate
before delivery:

```bash
./tools/validate.sh --static           # repository checks + host tests
./tools/validate.sh --esp32 <board>    # ESP-IDF build for the affected board
./tools/validate.sh                    # complete gate
```

The ESP-IDF gate requires an activated ESP-IDF environment; each board
declares its required IDF version in its board README. Do not describe a
successful build as hardware validation. Final delivery must report these
fields separately:

1. Static checks result.
2. Build result per affected board.
3. On-device verification result, or an explicit statement that it was not
   performed.

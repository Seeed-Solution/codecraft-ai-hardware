<p align="right">
  <strong>English</strong> · <a href="README.zh_CN.md">简体中文</a>
</p>

# CodeCraft AI Hardware

An open hardware capability reference repository built for AI-agent-driven
development on Seeed devices.

This is a **pure hardware** repository: board support packages (BSPs),
standalone demos, verified hardware facts, and agent-facing development
guidance. There is no cloud service, no device pairing, and no account
system — firmware built from this repository runs entirely on the device.

## Who this is for

- **AI coding agents** (Codex, Muse Code, Claude Code, ...): the repository
  is structured so an agent can find authoritative hardware facts, follow a
  task routing table, and pass a validation gate without human hand-holding.
  Start at [AGENTS.md](AGENTS.md).
- **Human developers**: use the per-board examples as a starting point for
  your own firmware. Start at [docs/README.md](docs/README.md).

## Repository layout

| Path | Contents |
|---|---|
| `esp32/boards/<board>/` | Per-board sdkconfig, board-level BSP, and standalone examples for ESP32 targets |
| `esp32/components/` | Components shared across ESP32 boards |
| `linux/boards/<board>/` | Linux-based devices (device tree overlays, GPIO maps, Python/C demos) |
| `skills/` | Agent skills governing develop / setup / build / device-test / debug workflows |
| `docs/` | Hardware guides, vendor-source citations, development conventions |
| `tools/validate.sh` | The validation gate every delivery must pass |
| `tests/` | Host-side unit tests for board-independent logic |

## Supported boards

| Board | Platform | Status |
|---|---|---|
| SenseCAP Watcher | ESP32-S3 | Phase 0: facts gathered, BSP extraction pending |
| SenseCAP Indicator | ESP32-S3 | Phase 0: facts gathered, BSP extraction pending |
| reTerminal E1001 | ESP32-S3 | Phase 0: facts gathered, BSP extraction pending |
| reTerminal E1002 | ESP32-S3 | Phase 0: facts gathered, BSP extraction pending |
| reSpeaker Lite (XIAO ESP32-S3) | ESP32-S3 | BSP + 3 examples build-verified (IDF v6.0.2); on-device test pending |
| reTerminal (CM4) | Linux | Planned |

See [docs/inventory.md](docs/inventory.md) for the capability matrix and
extraction sources, and [docs/vendor-sources.md](docs/vendor-sources.md) for
the authoritative upstream reference for every board.

## Validation gate

```bash
./tools/validate.sh --static           # repository checks + host tests
./tools/validate.sh --esp32 <board>    # ESP-IDF build for one board
./tools/validate.sh --linux <board>    # Linux board checks
./tools/validate.sh                    # complete gate
```

## License and attribution

Apache License 2.0, see [LICENSE](LICENSE). Third-party attributions are in
[NOTICE](NOTICE).

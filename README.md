<p align="right">
  <strong>English</strong> · <a href="README.zh_CN.md">简体中文</a>
</p>

# CodeCraft AI Hardware

An open hardware capability reference repository built for AI-agent-driven
development on Seeed devices.

This is a **pure hardware** repository: board support packages (BSPs),
standalone demos, verified hardware facts, and agent-facing development
guidance. It contains no cloud runtime, device pairing, or account
system — firmware built from this repository runs entirely on the device.

## Who this is for

- **AI coding agents** (Codex, Muse Code, Claude Code, ...): the repository
  is structured so an agent can find authoritative hardware facts, follow a
  task routing table, and pass a validation gate without human hand-holding.
  Start at [AGENTS.md](AGENTS.md).
- **Human developers**: use the per-board examples as a starting point for
  your own firmware. Start at [docs/README.md](docs/README.md).

## CodeCraft Hub

[CodeCraft Hub](https://test-sensecraft.seeed.cc/wiki/en/sensecraft-cli/)
is a cloud platform for sharing firmware. It lives outside this
repository.

To upload firmware for online flashing or sharing, use
[`@seeed-studio/sensecraft-cli`](https://test-sensecraft.seeed.cc/wiki/en/sensecraft-cli/)
to generate and upload it. Install and usage are documented on that
page, including the agent skill that ships with the CLI. AI agents
follow the CodeCraft Hub section in [AGENTS.md](AGENTS.md) and do this
only when asked to upload, flash online, or share firmware.

## Repository layout

| Path | Contents |
|---|---|
| `esp32/boards/<board>/` | Per-board sdkconfig, board-level BSP, and standalone examples for ESP32 targets |
| `esp32/components/` | Components shared across ESP32 boards |
| `linux/boards/<board>/` | Linux-based devices (device tree overlays, GPIO maps, Python/C demos) |
| `arduino/boards/<board>/` | Arduino-framework boards (arduino-cli sketches); see `arduino/AGENTS.md` |
| `skills/` | Agent skills governing develop / setup / build / device-test / debug workflows |
| `docs/` | Hardware guides, vendor-source citations, development conventions |
| `tools/validate.sh` | The validation gate every delivery must pass |
| `tests/` | Host-side unit tests for board-independent logic |

## Supported boards

| Board | Platform | BSP scope | Examples |
|---|---|---|---|
| SenseCAP Watcher | `ESP32-S3` | camera, display, touch, audio, knob, battery, microSD | 8 |
| SenseCAP Indicator | `ESP32-S3` | RGB display, IO expander, button, touch | 3 |
| reTerminal E1001 | `ESP32-S3` | EPD, keys, LED, SHT4x, battery, RTC, charger, buzzer, PDM mic, microSD | 4 |
| reTerminal E1002 | `ESP32-S3` | E1001 BSP with the Spectra 6 color panel | 4 |
| reSpeaker Lite (XIAO ESP32-S3) | `ESP32-S3` | XMOS I2S audio, BOOT button, WS2812 LED | 3 |
| reTerminal Sticky | `ESP32-S3` | SSD1677 EPD, keys, SHT4x | 3 |
| XIAO ESP32-S3 1.47" Touch Display | `ESP32-S3` | JD9853A LCD, buttons, PDM mic | 3 |
| reTerminal (CM4) | `Linux` | peripherals overlay + demos: buttons, LEDs, buzzer, sensors, RTC | — |

All ESP32-S3 boards are build-verified on ESP-IDF v6.0.2; on-device
validation is pending for every board.

See [docs/inventory.md](docs/inventory.md) for the capability matrix and
extraction sources, [docs/on-device-validation.md](docs/on-device-validation.md)
for the field-test checklist, and
[docs/vendor-sources.md](docs/vendor-sources.md) for
the authoritative upstream reference for every board.

## Validation gate

```bash
./tools/validate.sh --static           # repository checks + host tests
./tools/validate.sh --esp32 <board>    # ESP-IDF build for one board
./tools/validate.sh --linux <board>    # Linux board checks
./tools/validate.sh                    # complete gate
```

## Contributing

See [`.github/CONTRIBUTING.md`](.github/CONTRIBUTING.md) for commit, pull
request, and issue conventions. AI agents start at [AGENTS.md](AGENTS.md).

## License and attribution

Apache License 2.0, see [LICENSE](LICENSE). Third-party attributions are in
[NOTICE](NOTICE).

<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# esp-idf/respeaker-lite

Device knowledge: [AGENTS.md](../AGENTS.md). Hardware facts are maintained there and in its references.

Build/check: `codecraft validate --target esp-idf/respeaker-lite` from the repository root.

See `target.json` for examples, native interface references and exact toolchain selection. Build results and device observations are separate.

BSP public headers live in `bsp/include/`; implementation and private headers live in `bsp/src/`. Each example has its own `bsp.cmake` and managed dependency manifest. Exported projects include that example’s selected interfaces and source closure.

## Target implementation

Device behavior, reference interfaces and example purposes are documented in the [Device implementation references](../AGENTS.md#implementation-references).

### Layout

- `bsp/include/bsp_pins.h` — authoritative pin map with per-fact citations.
- `bsp/` — `bsp_audio` (XMOS + AIC3204 over I2C, 16 kHz I2S slave),
  `bsp_led` (WS2812), `bsp_button` (BOOT with debounce task).
- `examples/` — `audio_meter`, `button_led`, `wifi_scan`. Each is a
  standalone IDF project sharing the board `sdkconfig.defaults` via symlink. Native code paths below are relative to the Target directory.

Each is a standalone IDF project sharing the board `sdkconfig.defaults`
via symlink. Native code paths below are relative to the Target directory.

## Toolchain

The executable version declaration in `target.json` requires ESP-IDF 6.0.2 for esp32s3. Activate that SDK before building.

In an exported application, use `codecraft build-project .`; reference-repository validation commands do not replace application lock checks.

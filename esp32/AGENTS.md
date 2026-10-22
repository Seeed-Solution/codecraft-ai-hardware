# ESP32 Platform Rules

Applies to everything under `esp32/`. The root `AGENTS.md` governs first;
this file adds platform-specific rules.

## Board directory contract

Each `esp32/boards/<board>/` directory contains:

- `sdkconfig.defaults` — the board's build baseline (target, flash mode and
  size, PSRAM mode and speed).
- `bsp/` — board support package: `include/bsp_pins.h` (the authoritative
  pin map for the board), display/touch/audio/power drivers.
- `examples/` — standalone examples; each builds on its own.
- `README.md` — verified hardware facts with citations, the required ESP-IDF
  version, and known conflicts between sources.

## ESP-IDF versions

Declared per board in its README. Do not assume one version across boards.
The current known requirement: reSpeaker Lite builds with ESP-IDF v6.0.1.

## Rules

- Never invent a pin. Facts come from the sources in
  `docs/vendor-sources.md`, in the priority order from the root AGENTS.md.
- `bsp_pins.h` is the board's source of truth once written; keep it in sync
  with the vendor source and record any measured deviation in the board
  README.
- Flash layout: keep the default partition table minimal (NVS, PHY data,
  one factory app) unless the product requires otherwise.
- LVGL is not thread-safe; hold the BSP LVGL lock outside the LVGL task.
- Shared silicon drivers (e.g. the SSCMA camera client) live in
  `esp32/components/` only when at least two boards use them.

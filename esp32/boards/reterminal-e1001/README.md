# reTerminal E1001 (7.5" mono e-ink)

Status: **Phase 0** — facts verified, BSP extraction pending.

## Verified facts

- SoC: ESP32-S3, 16 MB flash (DIO 80 MHz), octal PSRAM (80 MHz).
  Source: muse `esp32/devices/sdkconfig.reterminal-e1001`.
- Display: UC8179-driven monochrome e-ink panel.
  Source: same (`HOMEHUB_LED_BACKEND_RETERMINAL_UC8179`).
- Input: the green button, GPIO 3. Source: same.
- ESP-IDF version: TBD (verify against vendor repo).

## Vendor source

Seeed wiki and the reTerminal e-ink driver repositories — identify the
canonical UC8179 driver and waveform data before writing code
(see `docs/vendor-sources.md`).

## Extraction plan

1. `bsp/include/bsp_pins.h` — e-ink SPI bus, busy/reset/DC pins, button.
2. `bsp/` — UC8179 driver with full/partial refresh, button.
3. `examples/` — `epaper_hello`, `button_refresh`.

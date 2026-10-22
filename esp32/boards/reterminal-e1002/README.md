# reTerminal E1002 (7.3" Spectra 6 color e-ink)

Status: **Phase 0** — facts verified, BSP extraction pending.

## Verified facts

- SoC: ESP32-S3, 16 MB flash (DIO 80 MHz), octal PSRAM (80 MHz).
  Source: muse `esp32/devices/sdkconfig.reterminal-e1002`.
- Display: Spectra 6 color e-ink panel.
  Source: same (`HOMEHUB_LED_BACKEND_RETERMINAL_SPECTRA6`).
- Input: the green button, GPIO 3. Source: same.
- ESP-IDF version: TBD (verify against vendor repo).

## Vendor source

Seeed wiki and the reTerminal e-ink driver repositories — Spectra 6 requires
correct color waveform handling; verify against vendor example code
(see `docs/vendor-sources.md`).

## Extraction plan

1. `bsp/include/bsp_pins.h` — e-ink SPI bus, busy/reset/DC pins, button.
2. `bsp/` — Spectra 6 driver with the vendor waveform, button.
3. `examples/` — `epaper_palette`, `button_refresh`.

# SenseCAP Indicator (D1 / D1S / D1Pro / D1L)

Status: **Phase 0** — facts verified, BSP extraction pending.

## Verified facts

- SoC: ESP32-S3, 8 MB flash (QIO 80 MHz), 8 MB octal PSRAM.
  Source: muse `esp32/devices/sdkconfig.sensecap-indicator`.
- Display: 4 inch 480x480 ST7701S RGB panel; the ~450 KB framebuffer sits in
  octal PSRAM. Source: same.
- Input: top case button, active low, GPIO 38. Source: same.
- Status indication: no status LED on the board; UIs use display edge bars.
  Source: same.
- Sensors: an on-board RP2040 handles the sensor channel; out of scope for
  the initial BSP. Source: same.
- USB: CH340 USB-UART bridge. Source: same.
- ESP-IDF version: TBD (verify against vendor repo).

## Vendor source

Seeed wiki and vendor repositories — identify the canonical BSP before
writing drivers (see `docs/vendor-sources.md`). Verify the 480x480 RGB
timing against vendor examples.

## Extraction plan

1. `bsp/include/bsp_pins.h` — RGB bus pins, button GPIO 38.
2. `bsp/` — ST7701S RGB display init + LVGL port, button.
3. `examples/` — `display_color_bars`, `touch_demo` (if touch populated),
   `button_demo`.

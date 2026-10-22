# SenseCAP Indicator (D1 / D1S / D1Pro / D1L)

Status: **Phase 2** — BSP extracted (display + button + touch),
build-verified on ESP-IDF v6.0.2. Not yet flashed to hardware.

## Verified facts

- SoC: ESP32-S3, 8 MB flash (QIO 80 MHz), 8 MB octal PSRAM (80 MHz).
  Source: muse `esp32/devices/sdkconfig.sensecap-indicator` (field-tested).
  Note: the vendor SDK runs flash/PSRAM at 120 MHz with an IDF 5.x patch;
  80 MHz is the conservative option used here.
- Display: 4 inch 480x480 ST7701S RGB panel, 16-bit bus, PCLK 18 MHz,
  timing hbp=50 hfp=10 hpw=8 / vbp=20 vfp=10 vpw=8, pclk active-low = 0
  (GX screen variant). Register init goes over a bit-banged 9-bit 3-wire
  SPI: SCK=GPIO41, SDO=GPIO48, CS on TCA9535 expander IO4, reset on IO5.
  Source: Seeed-Solution/SenseCAP_Indicator_ESP32
  `components/bsp/src/boards/sensecap_indicator_board.c`,
  `lcd_panel_config.c`, `Kconfig.projbuild`.
- Backlight: GPIO 45, active high. Source: same vendor board file.
- Input: top case button, active low, GPIO 38. Source: same vendor board
  file + muse sdkconfig.
- IO expander: TCA9535 at 0x20 (fallback 0x39) on the board I2C bus
  (SDA=39, SCL=40). Pin map: IO4 LCD CS, IO5 LCD reset, IO7 touch reset,
  IO8 RP2040 reset, IO10 BMP3xx power. Source: same vendor board file.
- Touch: I2C on the shared board bus. Vendor probes FT6336U (0x48/0x38)
  then GT911 (0x5D/0x14); reset is a 5 ms low pulse on expander IO7, and
  no interrupt line is wired, so the driver polls. Implemented in
  `bsp/bsp_touch.c` with the esp_lcd_touch FT5x06/GT911 components.
- Sensors: an on-board RP2040 handles the sensor channel; out of scope for
  this BSP. Source: muse sdkconfig notes.
- USB: CH340 USB-UART bridge. Source: muse sdkconfig.

## Vendor source

Canonical SDK: [Seeed-Solution/SenseCAP_Indicator_ESP32](https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32)
(Apache 2.0, targets ESP-IDF 5.1.1). This BSP is a fresh ESP-IDF v6
implementation using the vendor's pin/timing/register facts.

## BSP

| Header | Capability |
|---|---|
| `bsp_display.h` | ST7701S RGB panel init + esp_lcd RGB handle + backlight |
| `bsp_io_expander.h` | TCA9535 IO expander (shadowed output/config regs) |
| `bsp_button.h` | GPIO 38 button, debounced task |
| `bsp_touch.h` | FT6336U/GT911 auto-probe touch (esp_lcd_touch) |

Pin facts with sources: `bsp/include/bsp_pins.h`.

## Layout

- `bsp/include/bsp_pins.h` — every pin and timing constant, cited.
- `bsp/bsp_io_expander.c` — TCA9535 driver (shadowed output/config regs).
- `bsp/bsp_display.c` — ST7701S init + esp_lcd RGB panel + backlight.
- `bsp/bsp_button.c` — GPIO 38 button, debounced task.
- `bsp/bsp_touch.c` — FT6336U/GT911 auto-probe touch (esp_lcd_touch).
- `sdkconfig.defaults` — shared memory/flash config (symlinked into each
  example).
- `examples/display_color_bars` — 8 vertical color bars + backlight on.
- `examples/button_backlight` — white screen, button toggles backlight.
- `examples/touch_read` — logs touch coordinates as you press the screen.

## Examples

- `display_color_bars` — 8 vertical color bars + backlight on.
- `button_backlight` — white screen, button toggles backlight.
- `touch_read` — logs touch coordinates as you press the screen.

## Build

```sh
idf.py -C examples/display_color_bars build
```

Then flash and confirm: 8 vertical bars (white → black) on screen.

## Pending extraction

- DX screen variant (RGB only, no ST7701S SPI init) — untested, GX assumed.
- RP2040 sensor channel.

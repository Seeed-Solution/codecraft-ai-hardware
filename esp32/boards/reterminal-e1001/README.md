# reTerminal E1001 (7.5" mono e-ink)

Status: **extracted, build-verified on ESP-IDF v6.0.2** — not yet flashed to hardware.

Battery-powered 800x480 monochrome e-paper terminal: ESP32-S3R8
(8 MB octal PSRAM), 32 MB flash, SHT4x temperature/humidity, PCF8563 RTC,
SY6974B charger for the 2000 mAh LiPo, buzzer, PDM mic, microSD, three top
keys and a green status LED.

## BSP

| Header | Capability |
|---|---|
| `bsp_epd.h` | UC8179 panel: 1 bpp framebuffer (MSB leftmost, 1 = white), full/fast refresh, panel deep-sleeps after each update |
| `bsp_button.h` | green / Left / Right keys (GPIO 3/4/5, active low, debounced) |
| `bsp_led.h` | green status LED (GPIO 6, active low) |
| `bsp_sht4x.h` | temperature/humidity, single-shot, CRC-checked |
| `bsp_battery.h` | battery millivolts (ADC1 CH0, 10k/10k divider, enable GPIO 21) |

Pin facts with sources: `bsp/include/bsp_pins.h`.
Pending drivers (pins documented, code TODO): PCF8563 RTC, SY6974B charger,
PDM mic, buzzer, microSD.

## Examples

- `epd_hello` — checkerboard + bars pattern, one full refresh; demonstrates
  the framebuffer format and the refresh cycle.
- `sht4x_battery` — 10 s sampling loop of sensor + battery, LED blink per
  sample, key press samples immediately.

Build (ESP-IDF v6.0.2):

```sh
cd examples/epd_hello && idf.py build flash monitor
```

## Notes for agents

- Flash: 32 MB W25Q256JV (vendor schematic + Zephyr dts). muse-gadget-sdk
  configures 16 MB; both build, 32 MB matches hardware.
- The 48 KB framebuffer belongs in octal PSRAM (`MALLOC_CAP_SPIRAM`); the
  examples fall back to internal RAM.
- The UC8179 has no partial-refresh waveform here: "fast" refresh is the
  muse/GxEPD2 GDEY075T7 sequence. Do not invent a partial refresh.
- Board heat warms the onboard SHT4x; apply an offset in the application
  (muse used a Kconfig offset in tenths of a degree).

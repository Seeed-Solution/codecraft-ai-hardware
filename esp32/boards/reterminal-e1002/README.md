# reTerminal E1002 (7.3" Spectra 6 color e-ink)

Status: **extracted, build-verified on ESP-IDF v6.0.2** — not yet flashed to hardware.

Battery-powered 800x480 six-color (black/white/yellow/red/blue/green)
e-paper terminal: ESP32-S3R8 (8 MB octal PSRAM), 32 MB flash, SHT4x
temperature/humidity, PCF8563 RTC, SY6974B charger for the 2000 mAh LiPo,
buzzer, PDM mic, microSD, three top keys and a green status LED. Same
mainboard as the E1001; only the panel differs.

## BSP

| Header | Capability |
|---|---|
| `bsp_epd.h` | Spectra 6 panel (GDEP073E01): 4 bpp framebuffer (2 px/byte, left pixel high nibble), `bsp_epd_ink_t` ink codes, `bsp_epd_set_pixel()` helper, full refreshes only (~30 s) |
| `bsp_button.h` | green / Left / Right keys (GPIO 3/4/5, active low, debounced) |
| `bsp_led.h` | green status LED (GPIO 6, active low) |
| `bsp_sht4x.h` | temperature/humidity, single-shot, CRC-checked |
| `bsp_battery.h` | battery millivolts (ADC1 CH0, 10k/10k divider, enable GPIO 21) |

Pin facts with sources: `bsp/include/bsp_pins.h`.
Pending drivers (pins documented, code TODO): PCF8563 RTC, SY6974B charger,
PDM mic, buzzer, microSD.

## Examples

- `epd_hello` — six vertical color bars, one full refresh.
- `sht4x_battery` — 10 s sampling loop of sensor + battery, LED blink per
  sample, key press samples immediately.

Build (ESP-IDF v6.0.2):

```sh
cd examples/epd_hello && idf.py build flash monitor
```

## Notes for agents

- The 192 KB framebuffer must go to octal PSRAM (`MALLOC_CAP_SPIRAM`); the
  examples fall back to internal RAM.
- Full refreshes only; the panel ignores fast/partial requests. A refresh
  takes ~30 s and the busy timeout is 60 s.
- Ink packing: 2 pixels per byte, left pixel in the high nibble; codes
  0 black, 1 white, 2 yellow, 3 red, 4 blue, 5 green (muse + Zephyr
  color-palette agree).
- Board heat warms the onboard SHT4x; apply an offset in the application.

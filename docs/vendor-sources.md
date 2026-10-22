# Vendor Sources

Before adding a feature to a board (its display, codec, power chip, camera,
sensors and so on) or porting a board, read the vendor's own code first.
Clone the repo rather than reading it on the web, and search it. Cite the
source of every hardware fact in a comment at the top of the file that uses
it. Do not guess pins.

This table's citation discipline is derived from the device-porting recipe in
Meta's muse-gadget-sdk (`esp32/devices/AGENTS.md`, Apache 2.0).

| Board | Vendor source | Where to look |
|---|---|---|
| SenseCAP Watcher | [Seeed-Studio/SenseCAP-Watcher-Firmware](https://github.com/Seeed-Studio/SenseCAP-Watcher-Firmware) | `components/sensecap-watcher/` is the vendor BSP. `include/sensecap-watcher.h` has the pins for the LCD, touch, knob, IO expander, audio, battery, SD card and the Himax camera chip (driven through `components/sscma_client/`). `examples/factory_firmware/` is the firmware it ships with. xiaozhi-esp32's [sensecap-watcher board](https://github.com/78/xiaozhi-esp32/tree/main/main/boards/sensecap-watcher) is a second reference. |
| SenseCAP Indicator | [Seeed-Solution/SenseCAP_Indicator_ESP32](https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32) (Apache 2.0, ESP-IDF 5.1.1) | `components/bsp/src/boards/sensecap_indicator_board.c` has the full pin map and TCA9535 expander map; `lcd_panel_config.c` has the ST7701S register init and bit-bang SPI channel; `Kconfig.projbuild` has PCLK/timing defaults and the GX/DX screen variants. |
| reTerminal E1001 / E1002 | Seeed wiki + reTerminal e-ink repositories (identify canonical driver repo) | UC8179 (E1001) and Spectra 6 (E1002) waveform and refresh handling; verify against vendor example code before writing a driver. |
| reSpeaker Lite | [respeaker/reSpeaker_Lite](https://github.com/respeaker/reSpeaker_Lite) | `doc/images/pinout.png`, the I2C examples, `xmos_firmwares/`. See also Seeed wiki pages for the XIAO ESP32-S3 combination. |
| reTerminal (CM4) | Seeed wiki / [Seeed-Studio/seeed-linux-dtoverlays](https://github.com/Seeed-Studio/seeed-linux-dtoverlays) | The dtoverlay sources for display, buttons, buzzer, RTC and backlight control. |
| XIAO ESP32-S3 (module-level) | [Seeed-Studio/esp-idf or wiki](https://wiki.seeedstudio.com/xiao_esp32s3_getting_started/) | Module pinout, PSRAM mode, antenna notes. |

## Reference firmware worth reading

- [78/xiaozhi-esp32](https://github.com/78/xiaozhi-esp32) — community AI
  firmware with per-board drivers for several Seeed targets; a useful second
  opinion when vendor docs and measured behavior disagree.
- Meta's muse-gadget-sdk `esp32/devices/` — sdkconfig overlays and board
  docs with field-tested facts (e.g. flash modes, PSRAM modes, button GPIOs)
  already verified on hardware. Apache 2.0.

## Conflict resolution

When sources disagree, apply the priority from the root `AGENTS.md`:
vendor specifications and measured results first, then the board's pins
header, then BSP implementation, then guides, then demo code. Record the
conflict and its resolution in the board README so the next agent does not
re-litigate it.

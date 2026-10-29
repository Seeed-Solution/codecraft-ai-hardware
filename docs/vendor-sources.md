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
| reTerminal E1001 / E1002 | [Zephyr board definitions](https://github.com/zephyrproject-rtos/zephyr/tree/main/boards/seeed/reterminal_e1001) (`reterminal_e1001` / `reterminal_e1002`, Apache 2.0, vendor-contributed) + [E1001 V1.2 schematic](https://files.seeedstudio.com/wiki/reterminal_e10xx/res/202004307_reTerminal_E1001_V1_2_SCH_251120.pdf) (CC BY-SA 4.0) | The dts + pinctrl dtsi are the cleanest facts: key/LED GPIOs and active levels, battery ADC divider + enable, I2C addresses (SHT4x 0x44, PCF8563 0x51, SY6974B 0x6B), buzzer PWM, 32 MB flash. The schematic confirms flash size and power topology. UC8179 (E1001) and Spectra 6 (E1002) refresh sequences came from muse `esp32/main/epd_uc8179.c` / `epd_spectra6.c`. PCF8563 RTC register handling follows `zephyr:drivers/rtc/rtc_pcf8563.c`; the SY6974B charger driver follows `zephyr:drivers/charger/charger_sy6974b.c` (register map, part-ID check, reset/watchdog init) with board defaults from the dts (1000 mA / 4208 mV). |
| reSpeaker Lite | [respeaker/reSpeaker_Lite](https://github.com/respeaker/reSpeaker_Lite) | `doc/images/pinout.png`, the I2C examples, `xmos_firmwares/`. See also Seeed wiki pages for the XIAO ESP32-S3 combination. |
| Wio Terminal | [Seeed-Studio/ArduinoCore-samd v1.8.6](https://github.com/Seeed-Studio/ArduinoCore-samd/tree/v1.8.6) + [Wio Terminal schematic v1.2](https://files.seeedstudio.com/wiki/Wio-Terminal/res/Wio-Terminal-SCH-v1.2.pdf) (CC BY-SA 4.0) | `variants/wio_terminal/variant.h` is the pin and peripheral map; `libraries/Seeed_Arduino_LCD/User_Setup.h` selects the ILI9341 LCD on SPI3. Schematic sheet 02 has the MCU, 4 MB QSPI flash and SPI2 microSD; sheet 04 has the five-way switch, buzzer and analog microphone; sheet 05 has the top buttons, Grove J4/J5, light sensor and IR emitter; sheet 06 has the RTL8720DN; sheet 07 has the LIS3DHTR and LCD connector. The IMU's `0x18` address comes from [Seeed_Arduino_LIS3DHTR](https://github.com/Seeed-Studio/Seeed_Arduino_LIS3DHTR) `LIS3DHTR_DEFAULT_ADDRESS`; the library defines `0x19` as an alternate address. J4 has no onboard I2C pull-ups (sheet 05). The [product page](https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/) supplies memory, display, wireless, sensor-range and 16 GB microSD limits. The [Wi-Fi firmware guide](https://wiki.seeedstudio.com/Wio-Terminal-Network-Overview/) covers the required RTL8720 eRPC image and `ambd_flash_tool`. Exact core/library versions are pinned in `arduino/boards/wio-terminal/`. |
| reTerminal (CM4) | [Seeed-Studio/seeed-linux-dtoverlays](https://github.com/Seeed-Studio/seeed-linux-dtoverlays), pinned at commit `1d53c3cf2c96` | `overlays/rpi/reTerminal-overlay.dts` (file header MIT) has every peripheral fact: MCP23008 @I2C1 0x38 pin map (buttons 0/2/1/3, LEDs 4-6 active low, buzzer 7 active high), expander IRQ GPIO6, power key GPIO13, LTR-303 @0x29, LIS331DLH @0x19, PCF8563 @I2C3 0x51, I2C1 300 kHz. Display/touch need Seeed's out-of-tree modules (`modules/ili9881d`); the repo-level COPYING is GPLv2, so link rather than vendor it. |
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

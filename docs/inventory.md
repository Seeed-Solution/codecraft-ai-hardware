# Phase 0 Inventory Matrix

The extraction baseline. Every fact below cites its source; `muse:` paths are
relative to the muse-gadget-sdk repository (Apache 2.0, Meta Platforms), and
`vendor:` entries point to the authoritative upstream listed in
[vendor-sources.md](vendor-sources.md).

## ESP32 boards

### SenseCAP Watcher

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3, 32 MB flash, octal PSRAM with XIP (`SPIRAM_XIP_FROM_PSRAM`) | `muse:esp32/devices/sdkconfig.muse-sensecap-watcher` | confirmed |
| Display + touch | LCD, touch, knob, IO expander pins live in the vendor BSP | `vendor:SenseCAP-Watcher-Firmware components/sensecap-watcher/include/sensecap-watcher.h` | pending |
| Audio | codec pins in vendor BSP | same as above | pending |
| Camera | Himax chip via SSCMA protocol | `muse:esp32/components/camera/` (`camera_sscma.c`, `sscma_proto.c`) | pending |
| Battery / SD | pins in vendor BSP | same as display row | pending |
| Power profile | USB-free sweep methodology + findings | `muse:esp32/docs/power/sensecap-watcher*.md`, `ppk2.md` | docs only, carry over |
| IDF version | per muse default (verify against vendor firmware) | `muse:esp32/README.md` | verify |

### SenseCAP Indicator (D1 / D1S / D1Pro / D1L)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3, 8 MB flash (QIO 80 MHz), 8 MB octal PSRAM | `muse:esp32/devices/sdkconfig.sensecap-indicator` | confirmed |
| Display | 4 inch 480x480 ST7701S RGB, 16-bit bus, PCLK 18 MHz; 450 KB framebuffer in octal PSRAM; register init over bit-banged 9-bit SPI (SCK=41, SDO=48, CS on TCA9535 IO4) | same + vendor `SenseCAP_Indicator_ESP32` | **extracted** (`bsp_display`) |
| Input | top button, active low, GPIO 38 | same + vendor board file | **extracted** (`bsp_button`) |
| Status indication | no status LED; display edge bars used instead | same | confirmed |
| Sensors | RP2040 co-processor handles sensors; unused in the muse profile | same | out of scope for v1 |
| USB | CH340 USB-UART bridge | same | confirmed |
| IDF version | vendor SDK targets ESP-IDF 5.1.1; this BSP builds clean on v6.0.2 | vendor sources + local build | confirmed |
| IO expander | TCA9535 @0x20 (fallback 0x39), I2C SDA=39/SCL=40; IO4 LCD CS, IO5 LCD RST, IO7 TP RST, IO8 RP2040 RST, IO10 BMP PWR | vendor board file | **extracted** (`bsp_io_expander`) |
| Touch | FT6336U @0x48/0x38 or GT911 @0x5D, reset on expander IO7 | vendor `indev_tp.c` | pending |

### reTerminal E1001 (7.5" mono e-ink)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3R8 (8 MB octal PSRAM), **32 MB flash** (W25Q256JV, DIO 80 MHz). muse configures 16 MB; the schematic + Zephyr dts confirm 32 MB | vendor schematic E1001 V1.2 + `zephyr:boards/seeed/reterminal_e1001` + `muse:esp32/devices/sdkconfig.reterminal-e1001` | confirmed |
| Display | UC8179 800x480 mono, SPI2 (SCLK=7, MOSI=9, CS=10, DC=11, RST=12, BUSY=13 low=busy), 10 MHz; 1 bpp MSB-leftmost 1=white; full + fast refresh, no partial waveform | `muse:esp32/main/epd_uc8179.c` + Zephyr dts | **extracted** (`bsp_epd`) |
| Input | three top keys: green/Refresh=GPIO 3, Left=4, Right=5; active low with internal pull-up | Zephyr dts + muse sdkconfig | **extracted** (`bsp_button`) |
| Status LED | green, GPIO 6, active low | Zephyr dts | **extracted** (`bsp_led`) |
| Sensors | SHT4x @0x44 on I2C0 (SDA=19, SCL=20) | `muse:esp32/main/reterminal_sht4x.c` + Zephyr dts | **extracted** (`bsp_sht4x`) |
| Battery | ADC1_CH0 (GPIO 1), 10k/10k divider (VBAT = 2x ADC), enable GPIO 21 active high, 10 ms settle | Zephyr dts | **extracted** (`bsp_battery`) |
| RTC | PCF8563 @0x51 on I2C0 (shares the bus with SHT4x via `bsp_i2c0_internal`) | Zephyr dts + `zephyr:drivers/rtc/rtc_pcf8563.c` | **extracted** (`bsp_rtc`) |
| Charger | SY6974B @0x6B on I2C1 (SDA=39, SCL=40); board defaults 1000 mA / 4208 mV from Zephyr dts; part-ID verified at init | `zephyr:drivers/charger/charger_sy6974b.c` + Zephyr dts | **extracted** (`bsp_charger`) |
| Audio / buzzer | PDM mic CLK=42 DATA=41 EN=38; buzzer LEDC GPIO 45 | Zephyr dts + schematic | **extracted** (`bsp_mic`, `bsp_buzzer`) — build-verified, mic levels need on-device validation |
| microSD | SDSPI on SPI2 shared with the panel: SCLK=7, MOSI=9, MISO=8, CS=14; TF_CARD_PWR rail via TPS22916 load switch, EN=16 active high; card-detect switch GPIO 15, low = inserted | vendor schematic E1001 V1.2 sheet 06 Peripherals (J5 ST-TF-003J, U13 TPS22916) | **extracted** (`bsp_sdcard`) — E1002 identical (shared mainboard) |
| IDF version | BSP + examples build clean on ESP-IDF v6.0.2 | local build | confirmed |

### reTerminal E1002 (7.3" Spectra 6 color e-ink)

Same mainboard as the E1001 (same pin map, 32 MB flash, SHT4x, RTC, charger,
battery sense, keys, LED); only the panel differs.

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | as E1001: ESP32-S3R8, 32 MB flash | as E1001 | confirmed |
| Display | Spectra 6 (GDEP073E01) 800x480 six-color; 4 bpp, 2 px/byte, left pixel in high nibble; ink codes 0 black, 1 white, 2 yellow, 3 red, 4 blue, 5 green; full refresh only (~30 s, busy timeout 60 s) | `muse:esp32/main/epd_spectra6.c` + Zephyr dts color palette | **extracted** (`bsp_epd`) |
| Input / LED / sensors / battery / RTC / charger / mic / buzzer | identical to E1001 (shared `bsp_pins.h` values) | as E1001 | **extracted** (same drivers copied) |
| IDF version | BSP + examples build clean on ESP-IDF v6.0.2 | local build | confirmed |

### Seeed reSpeaker Lite (XIAO ESP32-S3)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | XIAO ESP32-S3, 8 MB flash (DIO 80 MHz), 8 MB octal PSRAM | `muse:esp32/devices/sdkconfig.seeed-respeaker-lite` | confirmed |
| Audio in | XMOS-processed microphone over 16 kHz I2S; XMOS must run Seeed **16 kHz I2S firmware v1.0.9** (USB-audio and 48 kHz images incompatible) | `muse:esp32/devices/seeed-respeaker-lite.md` + vendor example | **extracted** (`bsp_audio`) |
| Input | XIAO BOOT button, GPIO 0, active low | sdkconfig + doc above | **extracted** (`bsp_button`) |
| Status | single WS2812 on GPIO 1 (XIAO D0), no power switch | `muse:esp32/main/led_status.c` + vendor D-pin map | **extracted** (`bsp_led`) |
| USB | small XIAO board USB exposes USB Serial/JTAG console (`303a:1001`) | doc above | confirmed |
| RF | attach XIAO external antenna; 2.4 GHz Wi-Fi only | doc above | confirmed |
| IDF version | ESP-IDF v6.0.1 per muse doc; examples build clean on v6.0.2 | doc above + local build | confirmed |

## Arduino boards

## Linux boards

| Board | Capability notes | Source | Status |
|---|---|---|---|
| reTerminal (CM4) | buttons/LEDs/buzzer/RTC/light sensor/accelerometer via mainline dtoverlay + sysfs/evdev/IIO; display via vendor overlay | seeed-linux-dtoverlays @1d53c3c (see vendor-sources.md) | Phase 1: overlay + demos written, hardware verification pending |
| reComputer series | Jetson-based; GPIO header maps | vendor wiki | planned |

## Cross-cutting dependencies to resolve during extraction

| Dependency | Needed by | Plan |
|---|---|---|
| SSCMA client (`camera_sscma.c`, `sscma_proto.c`) | SenseCAP Watcher camera | extract into `esp32/components/sscma_client/` with Meta copyright headers retained |
| LVGL version per board | all display boards | declare per board; do not assume one version |
| ESP-IDF version split | reSpeaker Lite requires v6.0.1; others TBD | declare per board in board README; CI matrix must cover both |

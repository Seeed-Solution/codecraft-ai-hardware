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


### reTerminal Sticky (3.97" mono e-ink)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3R8 (8 MB octal PSRAM); muse configures 16 MB flash (DIO 80 MHz); the 32 MB claim on the store page is unconfirmed | `muse:esp32/devices/sdkconfig.reterminal-sticky` | confirmed (16 MB), 32 MB unverified |
| Display | SSD1677 800x480 e-paper, used portrait 480x800; 1 bpp mono with full + fast refresh, plus a 4-gray image mode; SPI2 SCLK=13, MOSI=14, MISO=12, CS=15, DC=16, RST=17 (active low), BUSY=18 (high = busy), panel power EN=47, 20 MHz | `muse:esp32/main/epd_ssd1677.c` (MIT); protocol credited to Seeed `factory_fw` `ssd1677.c` (repo not publicly locatable, see vendor-sources.md) | **extracted** (`bsp_epd`) |
| Input | three keys: AI=GPIO 4, Up=5, Down=6, active low | `muse:esp32/components/muse/boards/board_reterminal_sticky.c` | **extracted** (`bsp_button`) |
| Sensors | SHT4x temperature/humidity @0x44 on I2C1 (SDA=1, SCL=0), shared with the BQ27220 fuel gauge @0x55 | `muse:esp32/main/reterminal_sht4x.c` + `board_reterminal_sticky.c` | **extracted** (`bsp_sht4x`) |
| Audio in | PDM mic on I2S0: CLK=19, DATA=20, power EN=38 (active high) | `muse:board_reterminal_sticky.c` | pins extracted; driver not yet ported |
| Touch | GT911 capacitive touch on I2C0 (SDA=3, SCL=2), 0x5D (fallback 0x14), power EN=42, RST=41, INT=21; Y axis flipped in software | `muse:board_reterminal_sticky.c` | pins extracted; driver not yet ported |
| microSD | SDSPI on the panel's SPI2 bus: CS=8, power EN=10, card-detect=11 (low = inserted), MISO=12 | `muse:board_reterminal_sticky.c` | pins extracted; driver not yet ported |
| Charger | charger enable on GPIO 39; muse drives it low at boot but the active level is unconfirmed against `factory_fw` | `muse:board_reterminal_sticky.c` | unverified until measured |
| IDF version | BSP + examples build clean on ESP-IDF v6.0.2 | local build | confirmed |

### XIAO ESP32-S3 1.47" Touch Display

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | XIAO ESP32-S3R8, 16 MB flash (DIO 80 MHz), 8 MB octal PSRAM | `muse:esp32/devices/sdkconfig.seeed-xiao-s3-147` | confirmed |
| Display | 1.47" JD9853A (ST7789-compatible) 172x320 LCD, SPI CS=3 (D2), DC=4 (D3), SCLK=7 (D8), MOSI=9 (D10), RST=13 (D17), BL=12 (D18); mirror X, gap 34; JD9853A init sequence from Seeed_GFX2 (MIT) | `muse:esp32/components/muse/boards/board_seeed_xiao_s3_147.c` + `Seeed-Studio/Seeed_GFX2` `Driver_JD9853A.cpp` | **extracted** (`bsp_display`) |
| Touch | AXS5106L capacitive touch @0x63 on I2C (SDA=5/D4, SCL=6/D5), INT=44 (D7) | `muse:board_seeed_xiao_s3_147.c` + Seeed_GFX2 | pins extracted; driver not yet ported |
| Input | USR1=GPIO 11 (D19), USR2=GPIO 42 (D15), active low | `muse:board_seeed_xiao_s3_147.c` | **extracted** (`bsp_button`) |
| Audio in | PDM mic on I2S0: CLK=1 (D0), DATA=2 (D1) | `muse:board_seeed_xiao_s3_147.c` + Seeed_GFX2 `0519_DashBoard_147_ESP32` | **extracted** (`bsp_mic`) — build-verified, levels need on-device validation |
| microSD | CS=43 (D6) on the LCD SPI bus | `muse:board_seeed_xiao_s3_147.c` | pins extracted; driver not yet ported |
| USB | USB-Serial/JTAG console; UART0 pins are taken by SD CS and touch INT, so no UART console fallback | `muse:esp32/devices/seeed-xiao-s3-147.md` | confirmed |
| IDF version | BSP + examples build clean on ESP-IDF v6.0.2 | local build | confirmed |

## Arduino boards

### Wio Terminal

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ATSAMD51P19A (Cortex-M4F), 512 KB program flash, 192 KB RAM, 120 MHz default; W25Q32JVZPIM 4 MB QSPI flash | Wio Terminal schematic v1.2 sheets 02; product page; core 1.8.6 `boards.txt` | confirmed |
| Display | 2.4-inch 320x240 ILI9341 on SPI3 (SCK=68, MOSI=67, MISO=66, CS=69, DC=70, RST=71, BL=72); `TFT_eSPI` bundled in the core | schematic sheet 07 + core `variant.h` + `Seeed_Arduino_LCD/User_Setup.h` | **extracted** (`display_buttons`) |
| Input | A/B/C keys GPIO 28/29/30; five-way up/left/right/down/press GPIO 31/32/33/34/35; active low with pull-ups | schematic sheets 04-05 + core `variant.h` | **extracted** (`display_buttons`) |
| IMU | LIS3DHTR, `Wire1`, SDA=78, SCL=77, address `0x18`, INT1=79 | schematic sheet 07 + core `variant.h` + Seeed_Arduino_LIS3DHTR `LIS3DHTR_DEFAULT_ADDRESS` | **extracted + measured** (`accelerometer`, 2026-10-28) |
| Light sensor | `WIO_LIGHT`, GPIO 27 / ADC channel 15; 400-1050 nm product range | schematic sheet 05 + core `variant.h` + product page | **extracted** (`light_sensor`, raw ADC) |
| Microphone | Analog electret + LMV358 on `WIO_MIC`, GPIO 39 / ADC channel 12 | schematic sheet 04 + core `variant.h` | **extracted** (`microphone_level`, analog envelope) |
| Buzzer | Passive B4013AM422-042 on GPIO 12 via Q4 | schematic sheet 04 + core `variant.h` | **extracted** (`buzzer_tones`) |
| microSD | SPI2: SCK=63, MOSI=62, MISO=61, CS=64, detect=65; product maximum 16 GB | schematic sheet 02 + core `variant.h` + product page | **extracted** (`sd_card_log`) |
| IR emitter | 940 nm LED D10, `WIO_IR` GPIO 14 active high through Q13; back of enclosure, left of microSD slot; transmit only | schematic sheet 05 + core `variant.h` + product page + Seeed IR emitter wiki | **extracted + emission observed** (`ir_nec_sender`) |
| Grove | J4 I2C `Wire` SDA=46/SCL=45, 3.3 V, no onboard pull-ups; J5 A0/D0 + A1/D1 | schematic sheet 05 + core `variant.h` | **extracted** (`grove_i2c_scan`, J4 only) |
| Wireless | RTL8720DN, 2.4/5 GHz 802.11 a/b/g/n + BLE 5.0; CHIP_PU=80, IRQ0=11; RTL eRPC firmware required | schematic sheet 06 + core `variant.h` + product page + Wi-Fi firmware guide | **extracted + Wi-Fi and BLE measured** (`wifi_scan` and `ble_scan` passed on firmware 2.1.3) |
| Arduino toolchain | `Seeeduino:samd@1.8.6`; exact libraries in `arduino/boards/wio-terminal/libraries.txt`; all 10 sketches compile | local arduino-cli build | build-verified; on-device results in `docs/on-device-validation.md` |

### Grove Beginner Kit for Arduino

Scope: **post-October-2025 kit revision only**. The supported sensor set is
DHT20 at I2C 0x38 and SPA06-003 at I2C 0x77/0x76; the 2020 DHT11/BMP280
revision is not part of this board directory.

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| MCU / memory | ATmega328P-MU, 16 MHz, 32,256-byte sketch maximum, 2 KB SRAM, 1 KB EEPROM, 5 V logic | Seeeduino Lotus wiki + Lotus v1.1 schematic + `arduino:avr@1.8.8` UNO board definition | confirmed |
| Board identity | Vendor core defines `Seeeduino:avr:Lotus`; repository builds the function-equivalent `arduino:avr:uno` target. The legacy 1.2.1 core bundles an i386-only avr-gcc and its indexed 1.3.0 archive is missing | Seeeduino AVR 1.2.1 archive + ArduinoCore-avr 1.8.8 + local build | build-verified only |
| I2C bus | Hardware I2C: A4/PC4/pin 18 SDA, A5/PC5/pin 19 SCL, 5 V logic | Lotus v1.1 schematic + core `standard` variant | **extracted** (`i2c_scan`) |
| Digital output | Grove LED on D4, active high, not PWM | kit wiki default-pin table + Lesson 1/4 | **extracted** (`led_blink`) |
| Digital input | Grove Button on D6, pressed HIGH; module has an external pull-down | kit wiki Lesson 2 | **extracted** (`button_led`) |
| Analog input | Rotary potentiometer on A0; 10-bit ADC | kit wiki Lesson 3 | **extracted** (`potentiometer`) |
| Tone output | Passive piezo buzzer on D5; driven with `tone()` | kit wiki Lesson 4 | **extracted** (`buzzer_tones`) |
| Analog light | Grove Light on A6; raw ADC, not lux | kit wiki Lesson 5 | **extracted** (`light_sensor`, raw only) |
| Analog sound | Grove Sound on A2; analog envelope, not dB SPL or audio | kit wiki Lesson 6 | **extracted** (`sound_sensor`, envelope only) |
| Display | 0.96-inch 128x64 SSD1306/SSD1315-compatible OLED on I2C 0x3C (0x3D alternate); 0x78 in the vendor table is the 8-bit write address | kit wiki + Grove OLED wiki | **extracted** (`oled_status`) |
| Temperature/humidity | DHT20 on I2C 0x38; black detector identifies this revision; confirm 0x38 with `i2c_scan` because the published kit schematic predates this sensor | kit wiki version notes + Grove DHT library 2.0.2 | **extracted** (`temperature_humidity_dht20`; confirm 0x38) |
| Pressure | SPA06-003 on I2C 0x77/0x76; library checks device ID 0x11 | SPA06-003 wiki + Seeed Arduino SPA06 1.0.1 | **extracted** (`air_pressure_spa06`) |
| Motion | LIS3DHTR on I2C 0x19; the library default 0x18 must be overridden | kit wiki Lesson 10 + LIS3DHTR 1.2.4 | **extracted** (`accelerometer`) |
| Arduino toolchain | `arduino:avr@1.8.8`; exact libraries in `arduino/boards/grove-beginner-kit/libraries.txt`; all 10 device demos plus `i2c_scan` compile | local arduino-cli build | build-verified; on-device serial/readout checks recorded in `docs/on-device-validation.md` |

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

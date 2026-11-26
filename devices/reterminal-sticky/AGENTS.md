# reTerminal Sticky

## Overview

Compact magnetic 3.97-inch 4-level grayscale e-paper device with touch, three buttons, environmental/motion sensors, RTC, microphone, buzzer, microSD, Wi-Fi/BLE and a 750 mAh battery.

Best for low-frequency, always-visible notes, weather, schedules and status dashboards. Pocket size makes it suitable for both handheld and stationary use. Stick it to magnetic surfaces, or the back of your phone. Mount it to any surface with the adhesive magnetic ring accessory.

## Profile

| Field | Value |
| --- | --- |
| ID | reterminal-sticky |
| Format | 5 |
| Revision | unknown |
| Maintenance | unassigned |
| Maturity | experimental |

## Usage

Start from the Intent routing table below. The registered Target provides the standalone display, button and SHT40 examples (`epd_hello`, `button_hello`, `sht4x`). The guides under `reference/` explain display refresh, RTC, battery, navigation, sleep, microphone, microSD and IMU implementation patterns; a guide alone does not establish a supported native interface or verified hardware behavior. Export private applications into a project workspace through the development workflow instead of editing the reference examples.

## Physical layout

Viewing from the front in portrait orientation.

- Dimensions (height × width × thickness): 106 × 65.5 × 7.3 mm
- Weight: 70 g
- Protection: IP40 (not waterproof)

Card-style body with a light silver-grey front bezel and glass touch cover. The rear shell is light silver-grey, with four N52 magnets in the four corners and a center metal ring beneath the shell (it works with the adhesive magnetic ring accessory). All four edges are white. Three buttons sit along the upper part of the right edge, from top to bottom: AI/OK/Power (grey), Up (white), Down (white). The microSD slot is on the lower part of the left edge. On the bottom, from left to right: reset hole, microphone, lanyard hole, charging LED and USB-C port.

## Specs

### Platform

- ESP-IDF (recommanded)

- Arduino

### Controller

- Main controller: ESP32-S3
- PSRAM capacity: 8 MB
- Physical QSPI Flash: 32 MB

### Connectivity

#### Wireless

- 2.4 GHz Wi-Fi

- BLE 5.0

#### Wired interface:

- USB-C for power, data, firmware download and debugging (CH343P USB-UART)

- Expandable storage: microSD via SPI, sharing SPI2 with the display

### Inputs

#### Sensors

- SHT40 temperature/humidity (I2C)
- LSM6DS3TR-C 6-axis IMU (I2C)
- PDM digital microphone
- Real-time clock: PCF8563 (I2C)

#### Controls

- Buttons: AI / OK / Power button, Up, Down; Reset via CHIP_PU
- Touch: GT911 capacitive touch panel (I2C)

### Outputs

#### Display

- 3.97-inch E-Ink, 4-level grayscale
- Panel controllers supported by the official demo: SSD1677 or SSD2677. The [pinned official panel implementation][vendor-panel] detects the controller; the mapping from hardware revisions to panel controllers remains unknown.
- Native resolution: 800 × 480 px (horizontal)
- Pixel density: 235 ppi

#### Audio output

- Onboard buzzer (PWM)

### Power

- Battery: 750 mAh single-cell lithium
- Fuel gauge: BQ27220 (I2C)
- Charging: USB-C; red/green charging indicator (red while charging, green when fully charged)

### Accessories

- Mounting accessory: Adhesive magnetic ring for non-metal surfaces, included in the packaging

### Pins and signals

Sticky exposes no user-accessible connector pins. This table is for development reference only.

Required at boot: drive GPIO45 (HOLD) and GPIO46 (LOCK) HIGH to keep the power rail and power controller on.

| Function          | Hardware                             | Bus                        | Signals                                                        | Constraints                                                                                                                                                                                                                     |
| ----------------- | ------------------------------------ | -------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Display           | E-paper, SSD1677 or SSD2677          | SPI2, 20 MHz               | SCK=13, MOSI=14, MISO=12, CS=15, DC=16, RST=17, BUSY=18, EN=47 | RST active low; BUSY high while busy; EN high = powered. SPI2 is shared with microSD, which needs its own CS.                                                                                                                   |
| Touch             | GT911                                | I2C0                       | SDA=3, SCL=2, INT=21, RST=41, EN=42                            | Address 0x5D (fallback 0x14). The digitizer counts Y from the bottom, so flip Y in software.                                                                                                                                    |
| microSD           | microSD card                         | SPI2 (shared with display) | CS=8, EN=10, DET=11                                            | EN high = powered; DET low = card inserted.                                                                                                                                                                                     |
| Sensors and clock | SHT40, LSM6DS3TR-C, PCF8563, BQ27220 | I2C1                       | SDA=1, SCL=0; IMU INT=7                                        | Addresses 0x44, 0x6A, 0x51, 0x55. All share one bus instance.                                                                                                                                                                   |
| Keys              | AI/OK/Power, Up, Down                | GPIO                       | 4, 5, 6                                                        | Active low. GPIO4 is the deep-sleep wake source.                                                                                                                                                                                |
| Microphone        | PDM digital microphone               | PDM                        | CLK=19, DATA=20, EN=38                                         | EN high = powered.                                                                                                                                                                                                              |
| Buzzer            | Onboard buzzer                       | PWM                        | GPIO48                                                         |                                                                                                                                                                                                                                 |
| Power control     | Power and charger                    | GPIO                       | HOLD=45, LOCK=46, charger EN=39, external-power detect=9       | HOLD and LOCK active high. Charger EN: the demo drives it low to enable charging; the active level is unconfirmed against the factory firmware, so treat it as unverified until measured. Detect high = external power present. |
| USB-UART          | CH343P                               | UART                       | TX=43, RX=44                                                   |                                                                                                                                                                                                                                 |
| Reset             | Reset button hole                    | CHIP_PU                    | not a GPIO                                                     |                                                                                                                                                                                                                                 |

## Specifications

### Controller

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Main controller (`mcu`) | string | ESP32-S3 | — | documented | [device-notes][device-notes] | Main controller: {value} |
| PSRAM capacity (`psram`) | integer | 8 | MB | documented | [device-notes][device-notes] | PSRAM capacity: {value} MB |
| Physical QSPI flash (`flash`) | integer | 32 | MB | documented | [device-notes][device-notes] | Physical QSPI Flash: {value} MB |

### Connectivity

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Wi-Fi (`wifi`) | string | 2.4 GHz Wi-Fi | — | documented | [device-notes][device-notes] | - {value} |
| Bluetooth (`ble`) | string | BLE 5.0 | — | documented | [device-notes][device-notes] | - {value} |

### Inputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Buttons (`keys`) | string | AI / OK / Power button, Up, Down | — | documented | [device-notes][device-notes] | Buttons: {value} |
| Touch (`touch`) | string | GT911 | — | documented | [device-notes][device-notes] | Touch: {value} capacitive touch panel |
| Microphone (`mic`) | string | PDM digital microphone | — | documented | [device-notes][device-notes] | - {value} |
| Temperature/humidity sensor (`temp_sensor`) | string | SHT40 | — | documented | [device-notes][device-notes] | - {value} temperature/humidity (I2C) |
| IMU (`imu`) | string | LSM6DS3TR-C | — | documented | [device-notes][device-notes] | - {value} 6-axis IMU (I2C) |
| Real-time clock (`rtc`) | string | PCF8563 | — | documented | [device-notes][device-notes] | Real-time clock: {value} (I2C) |

### Outputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Panel controllers supported by the official demo (`display`) | string | SSD1677 or SSD2677 | — | documented | [device-notes][device-notes] | Panel controllers supported by the official demo: {value}. |
| Display size (`display_size`) | number | 3.97 | inch | documented | [device-notes][device-notes] | - {value}-inch E-Ink, 4-level grayscale |
| Native panel width (`display_width`) | integer | 800 | px | documented | [device-notes][device-notes] | Native resolution: {value} × 480 px (horizontal) |
| Native panel height (`display_height`) | integer | 480 | px | documented | [device-notes][device-notes] | Native resolution: 800 × {value} px (horizontal) |
| Buzzer (`buzzer`) | string | Onboard buzzer (PWM) | — | documented | [device-notes][device-notes] | - {value} |

### Power

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Battery capacity (`battery`) | integer | 750 | mAh | documented | [device-notes][device-notes] | Battery: {value} mAh single-cell lithium |
| Fuel gauge (`gauge`) | string | BQ27220 | — | documented | [device-notes][device-notes] | Fuel gauge: {value} (I2C) |

### Accessories

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Mounting accessory (`mount`) | string | Adhesive magnetic ring for non-metal surfaces, included in the packaging | — | documented | [device-notes][device-notes] | Mounting accessory: {value} |

## Intent routing

Locate references and targets based on intent. Target is the BSP in [`targets/`](targets/README.md). Reference is a guide in `reference/` for implementation of functions.

| Intent                           | Covers                                                                                                          | Reference                                                                                                                                                                                                                                                                                                                                                    | Target                                                                                                             |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Refresh the e-paper display      | Grayscale full, monochrome full and monochrome partial refresh                                                  | [display-refresh.md](reference/display-refresh.md)                                                                                                                                                                                                                                                                                                           | `bsp_epd_update`, `bsp_epd_update_gray4`; example `epd_hello`.                      |
| Identify the panel controller    | Read chip ID with `0x70` (`0x07` = SSD2677, else SSD1677), then apply that chip's busy timeout and reset timing | Demo source: [`resolve_panel_model()`](https://github.com/Seeed-Projects/sticky-esp-idf-dashboard/blob/1a28b2a11da2df3819c3a6f34561793f562d53c8/components/seeed_epaper/epaper_panel.c), [`sticky_display_init()`](https://github.com/Seeed-Projects/sticky-esp-idf-dashboard/blob/1a28b2a11da2df3819c3a6f34561793f562d53c8/main/devices/sticky_display.cpp) | None                                                                                                               |
| Update only the status-bar clock | Minute event, change detection, redraw of the `HH:MM` area, partial refresh                                     | [display-rtc-time.md](reference/display-rtc-time.md)                                                                                                                                                                                                                                                                                                         | Display only; no RTC driver                                                                                        |
| Add an information page          | Renderer, page registration, navigation, `AppState` data flow                                                   | [add-information-page.md](reference/add-information-page.md)                                                                                                                                                                                                                                                                                                 | None                                                                                                               |
| Navigate with buttons or swipes  | Button presses and horizontal swipes mapped to page and refresh events                                          | [button-touch-navigation.md](reference/button-touch-navigation.md)                                                                                                                                                                                                                                                                                           | `bsp_button_init`, `bsp_button_pressed`; example `button_hello`. No touch driver |
| Sleep and wake                   | AI hold-to-sleep, shutdown order, active-low EXT1 wake, page-state restore                                      | [deep-sleep-wake.md](reference/deep-sleep-wake.md)                                                                                                                                                                                                                                                                                                           | None                                                                                                               |
| Show battery and charging state  | BQ27220 percentage, external-power presence, separate validity states                                           | [display-battery-percentage.md](reference/display-battery-percentage.md)                                                                                                                                                                                                                                                                                     | None                                                                                                               |
| React to orientation changes     | Stable IMU orientation events, refresh only when the IMU page is visible                                        | [imu-orientation-monitoring.md](reference/imu-orientation-monitoring.md)                                                                                                                                                                                                                                                                                     | None                                                                                                               |
| Capture audio                    | AI-triggered one-second PDM sample with scoped microphone power                                                 | [microphone-capture.md](reference/microphone-capture.md)                                                                                                                                                                                                                                                                                                     | None                                                                                                               |
| Read files from microSD          | Mount, read and unmount around the SPI2 bus shared with the display                                             | [microsd-text-page.md](reference/microsd-text-page.md)                                                                                                                                                                                                                                                                                                       | None                                                                                                               |
| Read temperature and humidity    | SHT40 reading on shared I2C1                                                                                    | None                                                                                                                                                                                                                                                                                                                                                         | `bsp_sht4x_read`; example `sht4x`                                                       |
| Create images of the device      | Official product shots and use scenes with descriptions; physical details image models often get wrong          | [image-generation.md](reference/image-generation.md)                                                                                                                                                                                                                                                                                                         | None                                                                                                               |

## Limitations

- Physical board revision and fresh on-device verification are not recorded.
- The official demo supports SSD1677 and SSD2677; hardware-revision mappings remain unknown. The registered Target BSP supports SSD1677 only. Identify the panel before relying on display behavior; see the panel-detection sources in Intent routing.
- Charger enable (GPIO39) active level is unconfirmed against the factory firmware; treat it as unverified until measured.
- The native configuration uses a 16 MB flash address range while the vendor specification states 32 MB physical flash; the installed capacity is not measured.

## Source conflicts

| Source | Resolution |
| --- | --- |
| [device-notes][device-notes] | The vendor specification states 32 MB physical QSPI flash while the Target's `sdkconfig.defaults` configures a 16 MB address range. Keep the configured 16 MB range until the flash chip is identified on hardware; a configured address range is not a measurement of installed capacity. |

## Physical acceptance

### Preconditions

Identify the actual unit, revision, required peripherals and authorized write scope. Run one selected native example at a time. Record source digest, toolchain, stimuli, actual observations and recovery; a plan or build is not a physical pass.

### identity

- Interfaces: none
- Source: [target][target]

Record board identity, observed revision, connected port/drive and approved write scope.

Expected: Identity and revision are unambiguous and match this Device.

### recovery

- Interfaces: none
- Source: [target][target]

After the functional cases, power-cycle/restart using the documented normal procedure.

Expected: Device returns to the documented working state; record failures and recovery steps.

### button-hello

- Interfaces: input
- Source: [target][target]

Run `button_hello` under the confirmed write scope and observe the selected hardware.

Expected: log AI/Up/Down presses.

### epd-hello

- Interfaces: display
- Source: [target][target]

Run `epd_hello` under the confirmed write scope and observe the selected hardware.

Expected: test pattern, full refresh, then an inverted fast refresh.

### sht4x

- Interfaces: environment
- Source: [target][target]

Run `sht4x` under the confirmed write scope and observe the selected hardware.

Expected: temperature/humidity log once a second.

## Sources

- [device-notes][device-notes]
- [Sticky product page][product]
- [Sticky store listing][store]
- [Sticky hardware overview][vendor]
- [Sticky playground registry][playground]
- [Sticky ESP-IDF dashboard demo][dashboard]
- [Official panel implementation at 1a28b2a][vendor-panel]
- [Target README][readme]
- [Target manifest][target]
- [Native pin definitions][bsp-pins]
- [Display API contract][epd-api]
- [Display refresh implementations][guide-display-refresh]
- [RTC status-bar time partial refresh][guide-display-rtc-time]
- [Information page structure][guide-add-information-page]
- [Button and touch navigation][guide-button-touch-navigation]
- [Deep sleep and wake][guide-deep-sleep-wake]
- [Battery and charging display][guide-display-battery-percentage]
- [IMU orientation monitoring][guide-imu-orientation-monitoring]
- [Microphone capture][guide-microphone-capture]
- [microSD text page][guide-microsd-text-page]
- [Device image generation][guide-image-generation]

[device-notes]: AGENTS.md#specs
[product]: https://www.seeedstudio.com/sticky/
[store]: https://www.seeedstudio.com/reTerminal-Sticky-p-6861.html?utm_source=github&utm_medium=repository&utm_campaign=codecraft-ai-hardware
[vendor]: https://www.seeedstudio.com/sticky/docs/en/device-guide/hardware-overview/
[playground]: https://github.com/Seeed-Projects/reterminal-sticky-playground-registry/tree/main
[dashboard]: https://github.com/Seeed-Projects/sticky-esp-idf-dashboard/tree/1a28b2a11da2df3819c3a6f34561793f562d53c8
[readme]: targets/README.md
[target]: targets/target.json
[bsp-pins]: targets/bsp/include/bsp_pins.h
[epd-api]: targets/bsp/include/bsp_epd.h
[guide-display-refresh]: reference/display-refresh.md
[guide-display-rtc-time]: reference/display-rtc-time.md
[guide-add-information-page]: reference/add-information-page.md
[guide-button-touch-navigation]: reference/button-touch-navigation.md
[guide-deep-sleep-wake]: reference/deep-sleep-wake.md
[guide-display-battery-percentage]: reference/display-battery-percentage.md
[guide-imu-orientation-monitoring]: reference/imu-orientation-monitoring.md
[guide-microphone-capture]: reference/microphone-capture.md
[guide-microsd-text-page]: reference/microsd-text-page.md
[guide-image-generation]: reference/image-generation.md

[vendor-panel]: https://github.com/Seeed-Projects/sticky-esp-idf-dashboard/blob/1a28b2a11da2df3819c3a6f34561793f562d53c8/components/seeed_epaper/epaper_panel.c

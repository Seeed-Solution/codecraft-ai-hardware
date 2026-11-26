# SenseCAP Indicator

## Overview

RGB display, touch and a case button through native examples; sensor co-processor interfaces are outside the registered Target scope.

## Profile

| Field | Value |
| --- | --- |
| ID | sensecap-indicator |
| Format | 5 |
| Revision | unknown |
| Maintenance | unassigned |
| Maturity | experimental |

## Usage

Use the listed standalone examples as the supported development entry points.

## Physical layout

unknown; revision-specific orientation and imagery have not been verified.

## Hardware notes

### Hardware facts

- SoC: ESP32-S3, 8 MB flash (QIO 80 MHz), 8 MB octal PSRAM (80 MHz).
  Source: reference `../../devices/sensecap-indicator/targets/sdkconfig.defaults` (field-tested).
  Note: the vendor SDK runs flash/PSRAM at 120 MHz with an IDF 5.x patch;
  80 MHz is the conservative option used here.
- Display (GX assembly): 4 inch 480x480 ST7701S RGB panel, 16-bit bus, PCLK 18 MHz,
  timing hbp=50 hfp=10 hpw=8 / vbp=20 vfp=10 vpw=8, pclk active-low = 0
  (GX screen variant). Register init goes over a bit-banged 9-bit 3-wire
  SPI: SCK=GPIO41, SDO=GPIO48, CS on TCA9535 expander IO4, reset on IO5.
  Source: Seeed-Solution/SenseCAP_Indicator_ESP32
  `components/bsp/src/boards/sensecap_indicator_board.c`,
  `lcd_panel_config.c`, `Kconfig.projbuild`.
- Backlight: GPIO 45, active high. Source: same vendor board file.
- Input: top case button, active low, GPIO 38. Source: same vendor board
  file + reference sdkconfig.
- IO expander: TCA9535 at 0x20 (fallback 0x39) on the board I2C bus
  (SDA=39, SCL=40). Pin map: IO4 LCD CS, IO5 LCD reset, IO7 touch reset,
  IO8 RP2040 reset, IO10 BMP3xx power. Source: same vendor board file.
- Touch: I2C on the shared board bus. Vendor probes FT6336U (0x48/0x38)
  then GT911 (0x5D/0x14); reset is a 5 ms low pulse on expander IO7, and
  no interrupt line is wired, so the driver polls. Implemented in
  `bsp/src/bsp_touch.c` with the esp_lcd_touch FT5x06/GT911 components.
- Sensors: an on-board RP2040 handles the sensor channel; out of scope for
  this BSP. Source: Target sdkconfig notes.
- USB: CH340 USB-UART bridge. Source: Target sdkconfig.

### Vendor source

Canonical SDK: [Seeed-Solution/SenseCAP_Indicator_ESP32][vendor]
(Apache 2.0, targets ESP-IDF 5.1.1). The native implementation uses these vendor pin/timing/register facts.

### Supported assemblies and scope

GX and DX have explicit native profiles. See [sourced display variants][agents]
for controller differences and selection. Physical revision mapping remains
unknown. The RP2040 sensor channel is outside the registered BSP interfaces.

### Source locations

Primary links identify vendor sources; inspect the stated files and revisions before changing hardware facts.

| Board | Vendor source | Where to look |
| --- | --- | --- |
| SenseCAP Indicator | [Seeed-Solution/SenseCAP_Indicator_ESP32][vendor] (Apache 2.0, ESP-IDF 5.1.1) | `components/bsp/src/boards/sensecap_indicator_board.c` has the full pin map and TCA9535 expander map; `lcd_panel_config.c` has the ST7701S register init and bit-bang SPI channel; `Kconfig.projbuild` has PCLK/timing defaults and the GX/DX screen variants. |

### Display assembly provenance

This source record describes panel assembly choices, not
an observed PCB revision or successful physical validation.

#### Pinned vendor evidence

Repository: Seeed-Solution/SenseCAP_Indicator_ESP32, commit
`77edb8d2b9a92fc67965c1b2d4a838f0d09a1800`, inspected 2026-11-18.
The repository's Apache-2.0 license applies to its source implementation.

| File | Source and SHA-256 |
| --- | --- |
| Kconfig.projbuild | [Pinned source][vendor-2]; `c98d6e5506d471630e3a8b8a154956c20a57b8ebcf793932c75bdf930ac214fe` |
| sensecap_indicator_board.c | [Pinned source][vendor-3]; `fe1b67a80a9d84baceb41dd62e8590986003922111f5e501feab061585a93d77` |
| lcd_panel_config.c | [Pinned source][vendor-4]; `e0db50a81b3e70c1adc10efffb0d4a4a6ad0ce0860304d77b4ffaf6e94d8a808` |
| LICENSE | [Pinned source][vendor-5]; `58d1e17ffe5109a7ae296caafcadfdbe6a7d176f0bc4ab01e12a689b0499d8bd` |

#### Assembly facts and native behavior

Common display interface: RGB. Both vendor choices use 480 by 480 pixels,
a 16-bit RGB data bus and the same declared 18 MHz clock and porch/pulse timing.
The existing pin/timing definitions remain unchanged.

GX panel supplier: GuanXian. GX register controller: ST7701S.
GX initialization: SPI+RGB. The vendor calls lcd_panel_st7701s_init after RGB
initialization. The existing native register sequence remains the GX path.

DX panel supplier: DaXian. DX initialization: RGB-only.
After RGB initialization, the vendor sets expander LCD CS and LCD RESET high,
then configures both as outputs. It does not send the GX register sequence.
The DX controller identity is not established by the cited source; the vendor's
null controller label is a software marker, not a physical controller name.

These choices are not mappings from product names or serial numbers to PCB
revisions. Neither a verified revision list nor a board-specific panel
identification procedure is available in this record. Do not infer the panel
from the D1/D1S/D1L/D1Pro product label or a successful compiler result.

#### GX application observations (2026-11-26)

Source: [FoloTV/tellytile touch report at commit 86a3715](https://github.com/Links17/tellytile/blob/86a37151fe410a9623259a7606bf620381f6d906/evidence/indicator-gx-touch-orientation.md)
and the project developer's report in the 2026-11-26 repair request. The tested
unit used GX and FT6336U; PCB revision was not supplied. These are external
application observations, not physical acceptance of this repository revision.

- The developer confirmed correct brightness/frame-rate hit locations after
  rotating both touch axes about pixel 479. The BSP applies this correction
  only to GX with the FT6336U path, filtering coordinates outside 0..479.
  GT911 and DX retain their mappings. Confirm four corners on other units.
- Holding RP2040 in reset stopped provisioning beeps in the application. The
  developer reports sensors and microSD unavailable in that mode; optional
  reSpeaker Lite audio is independent. This is a co-processor disable policy,
  not an independently verified buzzer mute or peripheral compatibility claim.
- The developer reported stable playback on the tested channel with line
  buffering, 16 MHz PCLK and 64-byte data-cache lines. Buffer depth was not
  supplied. These are application tuning observations, not new panel limits
  or proof of the root cause. Vendor-default 18 MHz remains the BSP default.
- The application report identifies an LEDC fade-service error during dimming;
  the BSP's optional PWM path uses separate set-duty/update-duty operations.
  The report also records an I2C startup timeout and a later unexplained
  software restart; this change does not claim to resolve those failures.

Use the Target README for configuration and resource ownership. Repeat physical
acceptance on the actual revision before claiming this BSP is hardware verified.

#### Acceptance boundary

Software selection must identify GX or DX explicitly in the project and lock,
reject unknown choices, and retain the actual native compile definition in its
build evidence. The historical default can only describe the original GX
build profile; it is not automatic physical detection.

Physical acceptance requires an identified board/revision, attributable panel
identity, authorized deployment and observed display/touch/recovery results.
Those observations remain not_run. Image licensing/revision matching and
runtime resource measurements remain separate acceptance items.

Native pin citations: [bsp_pins.h][bsp-pins]. Software configuration and examples belong to the [Target][readme].

## Implementation references

The [esp-idf/sensecap-indicator Target][readme] provides the native reference implementation. Header and example names below are relative to that Target. Keep hardware behavior and implementation guidance here; the Target owns executable code, software-stack bindings and build configuration.

### Reference interfaces
| Header | Native interface |
|---|---|
| `bsp_display.h` | ST7701S RGB panel init + esp_lcd RGB handle + backlight |
| `bsp_io_expander.h` | TCA9535 IO expander (shadowed output/config regs) |
| `bsp_button.h` | GPIO 38 button, debounced task |
| `bsp_touch.h` | FT6336U/GT911 auto-probe touch (esp_lcd_touch) |

Pin facts with sources: [bsp_pins.h][bsp-pins].

### Reference examples

- `display_color_bars` — 8 vertical color bars + backlight on.
- `button_backlight` — white screen, button toggles backlight.
- `touch_read` — logs touch coordinates as you press the screen.

## Specifications

### Controller

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Flash capacity (`flash`) | integer | 8 | MB | documented | [device-notes][device-notes] | {value} MB flash |
| Controller (`mcu`) | string | ESP32-S3 | — | documented | [device-notes][device-notes] | SoC: {value}, |
| PSRAM capacity (`psram`) | integer | 8 | MB | documented | [device-notes][device-notes] | {value} MB octal PSRAM |

### Connectivity

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| USB bridge (`usb`) | string | CH340 | — | documented | [device-notes][device-notes] | USB: {value} USB-UART bridge |

### Inputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Button (`button`) | string | top case button | — | documented | [device-notes][device-notes] | Input: {value}, |
| Touch controller (`touch`) | string | FT6336U | — | documented | [device-notes][device-notes] | Vendor probes {value} |

### Outputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Display (`display`) | string | RGB | — | documented | [device-notes][device-notes] | Common display interface: {value}. |
| Native panel height (`display_height`) | integer | 480 | px | documented | [device-notes][device-notes] | 480x{value} |
| Native panel width (`display_width`) | integer | 480 | px | documented | [device-notes][device-notes] | {value}x480 |

### Power

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Power (`power_details`) | string | — | — | unknown | — | No independently confirmed value is retained; consult the original reference for limitations. |

### Accessories

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Accessories (`accessories_details`) | string | — | — | unknown | — | No independently confirmed value is retained; consult the original reference for limitations. |

## Hardware variants

### dx: DX display assembly

Source: [device-notes][device-notes]

#### Outputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Compatible PCB revisions (`compatible_revisions`) | strings | — | — | unknown | — | No attributable mapping from this panel assembly to physical PCB revisions is recorded. |
| Display register controller (`display_controller`) | string | — | — | unknown | — | The cited vendor source does not identify the DX controller. |
| Initialization interface (`initialization`) | string | RGB-only | — | documented | [device-notes][device-notes] | DX initialization: {value}. |
| Panel supplier (`panel_supplier`) | string | DaXian | — | documented | [device-notes][device-notes] | DX panel supplier: {value}. |

### gx: GX display assembly

Source: [device-notes][device-notes]

#### Outputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Compatible PCB revisions (`compatible_revisions`) | strings | — | — | unknown | — | No attributable mapping from this panel assembly to physical PCB revisions is recorded. |
| Display register controller (`display_controller`) | string | ST7701S | — | documented | [device-notes][device-notes] | GX register controller: {value}. |
| Initialization interface (`initialization`) | string | SPI+RGB | — | documented | [device-notes][device-notes] | GX initialization: {value}. |
| Panel supplier (`panel_supplier`) | string | GuanXian | — | documented | [device-notes][device-notes] | GX panel supplier: {value}. |

## Limitations

- Physical board revision and fresh on-device verification are not recorded.
- Read the original hardware reference for unsupported drivers, prerequisites and known limitations.
- GX/DX build selection does not identify the actual panel or PCB revision; physical compatibility remains unknown.

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

### button-backlight

- Interfaces: display, gpio, input
- Source: [target][target]

Run `button_backlight` under the confirmed write scope and observe the selected hardware.

Expected: press toggles backlight

### display-color-bars

- Interfaces: display, gpio
- Source: [target][target]

Run `display_color_bars` under the confirmed write scope and observe the selected hardware.

Expected: ST7701S RGB bars render

### touch-read

- Interfaces: gpio, touch
- Source: [target][target]

Run `touch_read` under the confirmed write scope and observe the selected hardware.

Expected: FT6336U or GT911 probed; press all four display corners and the
center, checking coordinates stay in 0..479 and match the displayed location.
Record GX/DX selection, probed controller and actual PCB revision.

## Sources

- [device-notes][device-notes]
- [bsp-pins][bsp-pins]
- [vendor][vendor]

[device-notes]: AGENTS.md#hardware-notes
[target]: targets/target.json
[bsp-pins]: targets/bsp/include/bsp_pins.h
[vendor]: https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32
[agents]: AGENTS.md
[vendor-2]: https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32/blob/77edb8d2b9a92fc67965c1b2d4a838f0d09a1800/components/bsp/Kconfig.projbuild
[vendor-3]: https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32/blob/77edb8d2b9a92fc67965c1b2d4a838f0d09a1800/components/bsp/src/boards/sensecap_indicator_board.c
[vendor-4]: https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32/blob/77edb8d2b9a92fc67965c1b2d4a838f0d09a1800/components/bsp/src/boards/lcd_panel_config.c
[vendor-5]: https://github.com/Seeed-Solution/SenseCAP_Indicator_ESP32/blob/77edb8d2b9a92fc67965c1b2d4a838f0d09a1800/LICENSE
[readme]: targets/README.md

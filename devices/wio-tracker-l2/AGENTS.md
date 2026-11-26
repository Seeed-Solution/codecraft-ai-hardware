# Wio Tracker L2

## Overview

Wio Tracker L2 is a board for custom LoRa trackers with a screen, GNSS, microSD
storage and an external battery connector. Use it to build a portable off-grid
messaging or mapping device, or to prototype an ESP32-S3 Wi-Fi application.

## Profile

| Field | Value |
| --- | --- |
| ID | wio-tracker-l2 |
| Format | 5 |
| Revision | unknown |
| Maintenance | unassigned |
| Maturity | experimental |

## Usage

Use the native Wi-Fi scan when developing an ESP-IDF application. For local
operations on Meshtastic firmware, use the existing client transports and
region configuration.

## Physical layout

![Wio Tracker L2, front view with landscape display and exposed board edges][front]

Viewed from the front with the screen text upright, the L2 has a black landscape
display with a rectangular bezel. The dark board extends above and below the
screen, exposing rows of holes. An amber display flex is visible on the right.
The base L2 is supplied without an enclosure and accepts an external Li-ion
battery. [Official model comparison][models]

Bare-board length, width, thickness, mass, ingress rating and supported rotation
angles are unverified. The front photograph does not locate the individual
buttons, USB-C, Grove, battery connector or microSD slot along the board edges.
The folded-antenna dimensions and weight in the series specification are not
identified as bare-board measurements.

## Platform

The registered native Target is ESP-IDF 6.0.2 for ESP32-S3. Start with the
[Wi-Fi scan example][example] for custom firmware. On a unit already running
Meshtastic, the [companion guide][mesh] covers the USB, BLE and TCP transports.

## Hardware notes

- Controller and storage. The Wio-S3 module combines ESP32-S3 and SX1262. Seeed lists 16 MB flash, 8 MB PSRAM and a microSD card slot. [Seeed specification][vendor]
- Connectivity. The L2 supports 2.4 GHz Wi-Fi, Bluetooth LE and SX1262 LoRa. USB-C carries power and data. The Grove port exposes two data signals. [Seeed specification][vendor] · [Connection guide][getting-started]
- Inputs. The series specification lists L76K GNSS, a microphone, a touch panel on the IPS configuration, and wake, user/boot and reset buttons plus a power switch. [Seeed specification][vendor]
- Outputs. The QSPI display interface supports the IPS and e-paper options. The documented IPS panel is 3.2 inches with 320 × 240 pixels in landscape. The series diagram shows an I²S codec and speaker port. [Seeed specification][vendor]
- Power. Seeed lists USB-C input and solar input. The base L2 accepts an external Li-ion battery via a connector. Battery capacity and pack dimensions depend on the pack selected. [Model comparison][models]

## Pins and signals

Use the [grouped pin map][pins] for the external Grove signals, panel buses,
radio, GNSS, audio, SD, USB, LEDs and controls. It distinguishes ESP32 GPIOs
from expander bits and documents the reset-time straps on GPIO0 and GPIO46.

## Implementation references

| Interface | Implementation |
| --- | --- |
| Native Wi-Fi scan | [bsp_wifi_scan_count()][wifi] counts access points on an initialized station. The [wifi_scan example][example] owns station setup and teardown. |
| Meshtastic companion | [l2_meshtastic.py][client] checks the local node model and uses the installed firmware's configuration API. |

## Specifications

### Controller

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Processor (`mcu`) | string | ESP32-S3 | — | documented | [hardware notes][notes] | combines {value} and SX1262 |
| Flash (`flash`) | integer | 16 | MB | documented | [hardware notes][notes] | {value} MB flash |
| PSRAM (`psram`) | integer | 8 | MB | documented | [hardware notes][notes] | {value} MB PSRAM |
| Removable storage (`storage`) | string | microSD | — | documented | [hardware notes][notes] | a {value} card slot |

### Connectivity

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Wi-Fi (`wifi`) | string | 2.4 GHz Wi-Fi | — | documented | [hardware notes][notes] | supports {value} |
| Bluetooth (`ble`) | string | Bluetooth LE | — | documented | [hardware notes][notes] | {value} and SX1262 LoRa |
| LoRa (`lora`) | string | SX1262 | — | documented | [hardware notes][notes] | {value} LoRa |
| Wired (`usb`) | string | USB-C | — | documented | [hardware notes][notes] | {value} carries power and data |
| Expansion (`grove`) | string | Grove | — | documented | [hardware notes][notes] | The {value} port exposes two data signals |

### Inputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Position input (`gnss`) | string | L76K GNSS | — | documented | [hardware notes][notes] | lists {value}, a microphone |
| Microphone (`microphone`) | string | microphone | — | documented | [hardware notes][notes] | GNSS, a {value}, |
| Controls (`controls`) | string | wake, user/boot and reset buttons | — | documented | [hardware notes][notes] | {value} plus a power switch |
| Touch (`touch`) | string | touch panel | — | documented | [hardware notes][notes] | a {value} on the IPS configuration |

### Outputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Display connection (`display`) | string | QSPI | — | documented | [hardware notes][notes] | The {value} display interface |
| IPS landscape resolution (`resolution`) | string | 320 × 240 | px | documented | [hardware notes][notes] | {value} pixels in landscape |
| Audio (`audio`) | string | I²S codec and speaker port | — | documented | [hardware notes][notes] | an {value} |

### Power

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Power input (`power`) | string | USB-C | — | documented | [hardware notes][notes] | lists {value} input and solar input |
| Solar power (`solar`) | string | solar input | — | documented | [hardware notes][notes] | USB-C input and {value} |
| Battery (`battery`) | string | external Li-ion battery | — | documented | [hardware notes][notes] | accepts an {value} via a connector |

### Accessories

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Device-specific accessories (`accessories`) | string | — | — | unknown | — | The model comparison gives no dedicated accessory SKU for the base L2. |

## Intent routing

| Intent | Covers | Reference | Target |
| --- | --- | --- | --- |
| Scan Wi-Fi in custom firmware | Station setup, passive scan, USB console result | [Wi-Fi scan workflow][scan-guide] | [wifi_scan][example], [bsp_wifi_scan_count()][wifi] |
| Connect an installed Meshtastic node | Phone pairing in MUI/BaseUI and host access via BLE, USB or TCP | [Connection guide][mesh] | [connect()][client] |
| Join Wi-Fi or choose the LoRa region | Personal network credentials, region selection and readback | [Configuration guide][mesh] | [join_wifi(), select_region(), region()][client] |
| Wire a peripheral | Bus signals, expander controls and boot constraints | [Pins and signals][pins] | None |
| Illustrate the base L2 | Official product photo, orientation and visible features | [Product image guide][imagery] | None |

## Limitations

- Physical revision, fitted display and on-device results for the native example are unknown.
- The native Target exposes a Wi-Fi scan. Meshtastic client operations require the corresponding firmware and are documented separately.
- The vendor introduction and specification table give different RF endpoints. Use the installed SKU and firmware region profile for deployment.

## Physical acceptance

### Preconditions

Identify the L2, board revision, USB port and firmware. Obtain authorization
for device writes. Use a known 2.4 GHz AP for the native scan. Run the
Meshtastic cases on a unit with that firmware installed.

### wifi-scan

- Interfaces: wireless
- Source: [target][target]

Run the native scan within reach of the test AP and capture the USB console.

Expected: The scan completes and reports at least one AP with the known AP on.

### mesh-connect

- Interfaces: none
- Source: [guide][mesh]

Pair a phone with the identified unit, inspect its region, then join the
test AP and reconnect over the LAN using the device IP.

Expected: Both transports identify the same node and report the same region.

## Sources

- [Seeed L2 introduction — board specifications and signal assignments][vendor]
- [Version comparison — base L2 configuration and purchase availability][models]
- [Official L2 front photograph — bare-board display assembly][front]
- [Seeed getting-started guide — Meshtastic pairing and network configuration][getting-started]
- [Hardware notes — source assertions for the specifications][notes]
- [Pin reference — grouped wiring and boot constraints][pins]
- [Image reference — orientation and visible features][imagery]
- [Meshtastic guide — connection and region selection][mesh]
- [Target guide — toolchain and project export][build]
- [Native Wi-Fi header — scan contract][wifi]
- [Wi-Fi example — station lifecycle and result][example]
- [Meshtastic companion — host configuration functions][client]

[notes]: AGENTS.md#hardware-notes
[vendor]: https://wiki.seeedstudio.com/meshtastic_wio_tracker_l2_intro/
[models]: https://wiki.seeedstudio.com/meshtastic_wio_tracker_l2_intro/#version-comparison
[front]: https://files.seeedstudio.com/wiki/SenseCAP/Wio_Tracker_L2/3-100026809-Wio-Tracker-L2.jpg
[getting-started]: https://wiki.seeedstudio.com/get_started_with_meshtastic_wio_tracker_l2/
[mesh]: reference/meshtastic.md
[pins]: reference/pins-and-signals.md
[imagery]: reference/image-generation.md
[scan-guide]: reference/meshtastic.md#choose-the-firmware
[build]: targets/README.md
[wifi]: targets/bsp/include/bsp_wifi.h
[example]: targets/examples/wifi_scan/main/main.c
[client]: targets/clients/l2_meshtastic.py
[target]: targets/target.json

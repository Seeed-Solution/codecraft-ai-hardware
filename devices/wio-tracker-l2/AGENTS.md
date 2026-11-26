# Wio Tracker L2

## Overview

Portable Wio-S3 LoRa tracker for off-grid messaging. This entry covers the
L2 native Wi-Fi scan example and local connectivity workflows on L2 units
running Meshtastic.

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

## Implementation references

- [Native example and private project export][build]
- [Reusable station scan interface][wifi]
- [Wi-Fi, phone pairing, client protocols and region selection][mesh]

## Physical layout

The L2 specification lists USB Type-C, a MicroSD card slot, a Grove connector
and a QSPI display interface. Identify the fitted panel before choosing UI
instructions. The native example uses only the USB console and on-chip Wi-Fi.

## Hardware notes

The [Seeed specification][vendor] identifies the module as Wio-S3 (ESP32-S3
and SX1262), with 16 MB flash and 8 MB PSRAM. It lists L76K GNSS, a QSPI
display interface, USB Type-C power and a MicroSD card slot. The
[L2 connection guide][getting-started] specifies 2.4 GHz Wi-Fi and describes
Bluetooth LE pairing in the Meshtastic UI.

## Specifications

### Controller

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Processor (`mcu`) | string | ESP32-S3 | — | documented | [hardware notes][notes] | Wio-S3 ({value} and SX1262) |

### Connectivity

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Wi-Fi (`wifi`) | string | 2.4 GHz Wi-Fi | — | documented | [hardware notes][notes] | specifies {value} |

### Inputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Position input (`gnss`) | string | L76K GNSS | — | documented | [hardware notes][notes] | lists {value} |

### Outputs

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Display connection (`display`) | string | QSPI | — | documented | [hardware notes][notes] | a {value} display interface |

### Power

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Power input (`power`) | string | USB Type-C | — | documented | [hardware notes][notes] | {value} power |

### Accessories

| Fact | Type | Value | Unit | Status | Source | Assertion / reason |
| --- | --- | --- | --- | --- | --- | --- |
| Removable storage (`storage`) | string | MicroSD | — | documented | [hardware notes][notes] | a {value} card slot |

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

- [hardware notes][notes]
- [Seeed product specification][vendor]
- [Seeed L2 getting started][getting-started]
- [Meshtastic workflows][mesh]
- [native build][build]
- [native wireless interface][wifi]
- [Meshtastic companion module][client]

[notes]: AGENTS.md#hardware-notes
[vendor]: https://wiki.seeedstudio.com/meshtastic_wio_tracker_l2_intro/
[getting-started]: https://wiki.seeedstudio.com/get_started_with_meshtastic_wio_tracker_l2/
[mesh]: reference/meshtastic.md
[build]: targets/README.md
[wifi]: targets/bsp/include/bsp_wifi.h
[client]: targets/clients/l2_meshtastic.py
[target]: targets/target.json

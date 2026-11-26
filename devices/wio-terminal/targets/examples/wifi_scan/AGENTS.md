# wifi_scan — Agent Notes

## Purpose

Use the RTL8720DN coprocessor to scan nearby Wi-Fi networks and show the five
strongest SSIDs by RSSI. The sketch does not use credentials, associate with an
access point or contact a cloud service.

## Hardware facts

- Wireless coprocessor: Realtek RTL8720DN, connected to the ATSAMD51 over
  SPI1/UART. CHIP_PU is GPIO 80 and IRQ0 is GPIO 11. Source: schematic v1.2
  sheet 06 and core 1.8.6 `variants/wio_terminal/variant.h`; see
  [`../../README.md`](../../README.md).
- The product supports 802.11 a/b/g/n on 2.4 GHz and 5 GHz. This demo displays
  only SSID and RSSI; it does not label or verify the band of each result.
- Software path: `Seeed Arduino rpcWiFi@1.1.0` over
  `Seeed Arduino rpcUnified@2.1.4`, with the exact dependency pins in
  the Target manifest (`mbedtls` 3.0.2, `FS` 2.1.5, `SFUD` 2.0.2).
- Arduino flashing writes only the ATSAMD51. The RTL8720DN must run compatible
  eRPC firmware; see the Target manifest and the vendor firmware guide.

## Build

From the repository root, run `codecraft validate --target arduino/wio-terminal --example wifi_scan`. The exact core and libraries are declared in target.json.

## Expected behavior

- LCD title: `WI-FI SCAN`; status starts at `starting radio...` and then shows
  `ready` after `WiFi.mode(WIFI_STA)` succeeds.
- A scan runs at boot and every 10 seconds. The status reports
  `<count> found / top <n>`, and the LCD shows up to five entries ordered by
  RSSI.
- Serial prints one line per retained network in the form
  `<rank> <SSID> <RSSI>`; empty SSIDs are shown as `(hidden)`.
- `WiFi.mode(WIFI_STA) failed` or `WiFi.scanNetworks() failed` appears in the
  serial log when initialization/scanning fails. The radio is never associated
  with a network.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/wio-terminal/observations.md); a successful build does not certify physical behavior.

## Limits

- Scan only: no credential entry, association, IP stack test, connection or
  cloud path.
- Top five networks only; the screen does not show BSSID/channel/security and
  does not distinguish 2.4 GHz from 5 GHz.
- Missing or mismatched RTL firmware produces radio failures that compile-time
  checks cannot detect.
- The RTL firmware version used for the on-device pass was not recorded;
  installation and version checks remain part of the reproducible setup.

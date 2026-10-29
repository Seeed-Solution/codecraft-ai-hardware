<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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
  `libraries.txt` (`mbedtls` 3.0.2, `FS` 2.1.5, `SFUD` 2.0.2).
- Arduino flashing writes only the ATSAMD51. The RTL8720DN must run compatible
  eRPC firmware; see the board README and the vendor firmware guide.

## Build

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/wifi_scan
```

Install the exact wireless libraries before building; do not assume the
Arduino IDE's latest versions are compatible. See the board
[AGENTS.md](../../AGENTS.md) for authorized upload and setup.

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

## On-device verification

1. Follow the vendor Wi-Fi firmware guide to install/verify compatible
   RTL8720DN eRPC firmware. Do not upload the sketch as a substitute for this
   step.
2. Build and upload with explicit authorization; open serial at 115200 baud.
3. Confirm the status progresses to `ready` and a scan reports networks that
   are actually present nearby.
4. Compare the strongest SSIDs/RSSI values with a known local access point;
   record the observed result rather than assuming 5 GHz coverage.
5. Run only this wireless demo while testing. Do not load the BLE sketch in
   parallel or expect concurrent Wi-Fi/BLE behavior from this repository.

Recorded on 2026-10-28: the RTL8720DN eRPC firmware already installed on the
test unit accepted the sketch; serial scans returned nearby 2.4 GHz and
5 GHz SSIDs with RSSI around -40 to -46 dBm, and the user confirmed the LCD.
`rpc_system_version()` was not queried, so no firmware version is claimed.

## Limits

- Scan only: no credential entry, association, IP stack test, connection or
  cloud path.
- Top five networks only; the screen does not show BSSID/channel/security and
  does not distinguish 2.4 GHz from 5 GHz.
- Missing or mismatched RTL firmware produces radio failures that compile-time
  checks cannot detect.
- The RTL firmware version used for the on-device pass was not recorded;
  installation and version checks remain part of the reproducible setup.

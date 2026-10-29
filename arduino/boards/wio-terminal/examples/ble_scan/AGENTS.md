<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# ble_scan — Agent Notes

## Purpose

Use the RTL8720DN to perform a five-second active BLE scan and show the five
strongest advertisements by RSSI. It does not pair with or connect to any
device.

## Hardware facts

- RTL8720DN provides BLE 5.0 and connects to the ATSAMD51 over SPI1/UART;
  CHIP_PU is GPIO 80 and IRQ0 is GPIO 11. Source: schematic v1.2 sheet 06,
  core 1.8.6 `variants/wio_terminal/variant.h`, and the product page; see
  [`../../README.md`](../../README.md).
- Software path: `Seeed Arduino rpcBLE@1.0.0` over
  `Seeed Arduino rpcUnified@2.1.4`, against official eRPC coprocessor
  firmware 2.1.3 (`rpc_system_version()`).
- The sketch calls `setActiveScan(true)`: it sends BLE scan requests to obtain
  advertisement/scan-response data. It never calls connect or pair.
- `rpcBLE@1.0.0` is incompatible with Arduino's `min`/`max` macros. The sketch
  `#undef`s both after including `TFT_eSPI.h` and before the rpcBLE headers.
  Preserve that order.

## Two coprocessor behaviours this demo depends on

Both were measured on a Wio Terminal with coprocessor firmware 2.1.3 and are
the reason this sketch differs from the vendor `BLE_scan` example.

- `BLEDevice::init()` does not start the BT stack. Call `ble_start()` once
  after it. Without that call the coprocessor keeps its BT stack down:
  `BLEDevice::getAddress()` returns `00:00:00:00:00:00` and a scan completes on
  schedule but reports zero advertisements.
- `BLEScan::start(duration, continue)` is the blocking overload. It waits on
  `m_semaphoreScanEnd` with no timeout, and it froze the LCD on this unit. Use
  the non-blocking `start(duration, scanCompleteCallback, continue)` overload
  and poll `millis()` from `loop()` instead.

Coprocessor firmware 2.1.3 also spins inside its own `ble_start()` until the BT
stack reports ready, so never call `ble_start()` while a scan is in progress.

## Build

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/ble_scan
```

Install the exact wireless dependency pins from `libraries.txt`. See the board
[AGENTS.md](../../AGENTS.md) for authorized upload and setup.

## Expected behavior

- LCD line 1: `BLE SCAN`; line 2: `RTL 2.1.3`; line 3: `local <MAC>`.
- Status box during a scan: `scanning <n>s  <count> adv`, where the count rises
  as advertisements arrive.
- After five seconds: `<count> adv / 5 shown` in green when devices were seen
  (orange when the count is zero), then up to five rows of
  `<name> <address> <rssi>` sorted by RSSI, `(unnamed)` when the advertisement
  carries no local name.
- The cycle repeats: five seconds scanning, four seconds idle.
- Serial prints the same device rows at 115200 baud.

## On-device verification

Verified on a Wio Terminal with coprocessor firmware 2.1.3:

1. The scan reported several hundred advertisements per five-second pass and
   listed real nearby devices with names, addresses and negative RSSI values.
2. The local address displayed on line 3 is non-zero, which shows the BT stack
   is up.

To re-verify:

1. Confirm the coprocessor firmware with `rpc_system_version()`; a Wi-Fi-only
   or mismatched image changes the failure mode before any scan logic runs.
2. Build and upload with explicit authorization.
3. Place a known BLE advertiser nearby (a phone, beacon or another board).
4. Confirm its name or address appears and its RSSI changes plausibly with
   distance.
5. Run only this wireless demo while testing. Do not run the Wi-Fi sketch in
   parallel or assume combined behaviour from this repository.

## Limits

- Active scanning sends scan requests; it is not a passive-only receiver.
- Top five advertisements only; no GATT service discovery, connection, pairing
  or data exchange.
- Name and address formatting depend on rpcBLE 1.0.0; do not infer privacy
  behaviour or address type from the display alone.
- The advertisement count is the number of distinct addresses reported during
  the pass, not a measure of nearby people or devices per person.

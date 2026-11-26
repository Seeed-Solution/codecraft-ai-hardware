<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# arduino/wio-terminal

Device knowledge: [AGENTS.md](../AGENTS.md). Read the [toolchain setup below](#toolchain), then run `codecraft validate --target arduino/wio-terminal --projects` from the repository root.

The Target manifest owns the FQBN, dependency pins and standalone sketches. Select one example with --example. Builds never upload; on-device procedures belong to the [Device checklist](../AGENTS.md#physical-acceptance).

## Target implementation

Hardware behavior and implementation references belong to the [Device entry](../AGENTS.md#implementation-references).

### Software integration constraints

- Two native integration requirements are recorded in
  `examples/ble_scan/AGENTS.md`: call `ble_start()` once after
  `BLEDevice::init()`, because without it the BT stack stays down and
  `BD_ADDR` reads all zeros; and use the non-blocking `BLEScan::start`
  overload, because the blocking one waits on a semaphore with no timeout and
  froze the LCD on this unit.
- `wifi_scan` and `ble_scan` each initialize and own the RTL8720DN. Run them
  as separate sketches; concurrent Wi-Fi/BLE operation is not exercised by
  this repository even though the vendor eRPC firmware is documented as
  supporting both.
- `rpcBLE@1.0.0` conflicts with the Arduino `min`/`max` macros. `ble_scan`
  undefines both before including the rpcBLE headers. Keep that include
  order when extending the sketch.
- `sd_card_log` expects a FAT32 card of at most 16 GB and mounts once. It
  does not implement hot-swap/remount handling. Card-detect GPIO 65 is
  documented but not used by the sketch. It uses Seeed Arduino FS 2.1.5 with
  an explicit SPI2 bus and CS 64; the standard Arduino `SD` library defaults
  to `SS` GPIO 50 and is not a valid substitute.

## Toolchain

The Target manifest is the executable version declaration. Use Arduino CLI 1.5.0 with these exact native dependencies. Library versions are copied from the board-maintained dependency declarations; bundled drivers come from the pinned core.

```sh
arduino-cli core update-index --additional-urls https://files.seeedstudio.com/arduino/package_seeeduino_boards_index.json
arduino-cli core install Seeeduino:samd@1.8.6 --additional-urls https://files.seeedstudio.com/arduino/package_seeeduino_boards_index.json
arduino-cli lib install --no-deps "Seeed Arduino rpcUnified@2.1.4"
arduino-cli lib install --no-deps "Seeed_Arduino_mbedtls@3.0.2"
arduino-cli lib install --no-deps "Seeed Arduino FS@2.1.5"
arduino-cli lib install --no-deps "Seeed Arduino SFUD@2.0.2"
arduino-cli lib install --no-deps "Seeed Arduino rpcWiFi@1.1.0"
arduino-cli lib install --no-deps "Seeed Arduino rpcBLE@1.0.0"
```

Use ARDUINO_DIRECTORIES_DATA, ARDUINO_DIRECTORIES_DOWNLOADS and ARDUINO_DIRECTORIES_USER for an isolated installation. The adapter fingerprints the selected core, native tools and pinned libraries, and passes explicit library paths to the compiler. Unrelated installed cores are not part of the lock. Builds write under .codecraft/builds/arduino/.

Source: https://github.com/Seeed-Studio/ArduinoCore-samd/tree/1.8.6

In an exported application, use `codecraft build-project .`; reference-repository validation commands do not replace application lock checks.

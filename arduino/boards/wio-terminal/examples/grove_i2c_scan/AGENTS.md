<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# grove_i2c_scan — Agent Notes

## Purpose

Scan the left Grove I2C connector and report responsive 7-bit addresses. It is
the first test for an external Grove I2C module and confirms that the external
`Wire` bus is separate from the onboard IMU.

## Hardware facts

- Left Grove J4 is the external I2C bus exposed as `Wire`: SDA GPIO 46,
  SCL GPIO 45. Source: schematic v1.2 sheet 05 and core 1.8.6
  `variants/wio_terminal/variant.h` (`PIN_WIRE_SDA`, `PIN_WIRE_SCL`); see
  [`../../README.md`](../../README.md).
- The connector supplies 3.3 V. Use only 3.3 V Grove I2C devices.
- J4 has no onboard I2C pull-up resistors: schematic sheet 05 shows only the
  connector, its 1 uF decoupling capacitor and DNP SWD links. A Grove I2C
  module normally provides the pull-ups. The sketch enables the SAMD51
  internal pull-ups before `Wire.begin()`; this is enough for a short bench
  bus but an external 4.7 kOhm pull-up pair is the robust fix.
- The LIS3DHTR is on `Wire1` (SDA 78 / SCL 77, address `0x18`) and must not
  appear in this scan. The right Grove J5 is not this I2C bus.
- The sketch sets `Wire` to 100 kHz and scans the seven-bit user address range
  `0x08` through `0x77` every three seconds, retaining up to 16 responding
  addresses. Reserved addresses `0x00`-`0x07` and `0x78`-`0x7F` are skipped;
  scanning them can report phantom devices on an empty bus.
- Measured on 2026-10-28: with an empty J4 bus, the first `Wire` transaction
  after an idle period can return a false ACK for the first scanned address.
  The sketch primes the bus with a reserved `0x7F` transaction before each
  scan; afterwards an empty bus reports zero devices. The SAMD51 internal
  pull-ups alone did not remove the artifact.

## Build

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/grove_i2c_scan
```

No external library is required. See the board [AGENTS.md](../../AGENTS.md)
for authorized upload and setup.

## Expected behavior

- LCD title: `GROVE I2C SCAN`; it shows `Wire: SDA 46 / SCL 45 / left Grove J4`
  and `3.3 V devices only`.
- With no device, the `0x7F` prime transaction completes first, serial prints
  no address lines, and the screen shows `no ACK on Wire` with a cable/3V3
  hint.
- With a device, the LCD shows addresses such as `0x48`, and serial prints
  `I2C device at 0x48` for each responder.
- The onboard IMU address `0x18` should not appear unless an external device
  on J4 happens to use the same address; that would be an external device, not
  the IMU.

## On-device verification

1. Build and upload with explicit authorization; open serial at 115200 baud.
2. Power down and connect one known 3.3 V Grove I2C device to the left J4.
3. Power up and confirm the device's documented address appears on the LCD and
   serial monitor within three seconds.
4. Disconnect the device and confirm the next scan reports `no ACK on Wire`.
5. Record the address and the external module part number; do not assume a
   generic address from another Grove product.

Verified on 2026-10-28: after adding the `0x7F` prime fix, an empty J4 bus
reported zero devices and no serial address lines. A temporary probe found the
onboard IMU at `0x18` on `Wire1`, confirming the scanner logic. No external
Grove I2C device was available in this session, so device detection was not
performed and remains pending.

## Limits

- Only ACK probing; it does not identify the device, read registers or test
  clock stretching.
- Up to 16 addresses are retained and displayed; the scan stops early once 16
  responders are found.
- The right J5 connector is not scanned here.
- The first-transaction false-ACK workaround is empirical; external 4.7 kOhm
  pull-ups remain the robust fix for a long or noisy bus.

<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# i2c_scan — Agent Notes

## Purpose

Enumerate ACK-ing devices on the shared hardware I2C bus of the supported
post-October-2025 kit. Use this before debugging the OLED, accelerometer,
DHT20 or SPA06-003.

## Hardware facts

- SDA is A4 (PC4, Arduino pin 18); SCL is A5 (PC5, pin 19). Source:
  Seeeduino Lotus v1.1 schematic and the `standard` core variant.
- The scan reports 7-bit addresses. Vendor documentation may print 8-bit
  write addresses, for example OLED 0x78 = 7-bit 0x3C. Source: kit wiki
  default-pin table and OLED wiki.
- Expected kit addresses on this revision: LIS3DHTR 0x19, OLED 0x3C or
  0x3D, DHT20 0x38, and SPA06-003 0x77 or 0x76. Source: kit wiki and the
  relevant module wiki pages.

## Build

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/i2c_scan
```

## Expected behavior

At 9600 baud the sketch prints a header, one `found 0xNN` line per device
with a label for known kit addresses, then `I2C scan complete: N device(s)`.
It repeats every 3 seconds.

## Physical test

1. Power the intact kit over USB and open the 9600-baud monitor.
2. Confirm 0x19, 0x38, 0x3C or 0x3D, and 0x77 or 0x76 on an intact supported
   kit. The exact OLED and pressure addresses can differ as documented.
3. If modules are broken out, disconnect one known device and confirm its
   address disappears on the next scan.

## Limitations

- An ACK is not an identity check. `air_pressure_spa06` additionally requires
  the SPA06-003 device ID 0x11 to match.
- A missing address can mean wrong wiring, power, pull-ups or a held bus; it
  does not by itself prove the module is broken.
- The scan is read-only. It does not configure or initialize any sensor.

<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# air_pressure_spa06 — Agent Notes

## Purpose

Read the SPA06-003 on the only supported kit revision, post-October-2025,
and print pressure, temperature and barometric altitude.

## Hardware facts

- This board directory targets only the post-October-2025 kit revision. The
  current kit wiki says versions after 2025.10 use SPA06-003. Source: kit
  wiki version-iteration notes and Lesson 9.
- SPA06-003 is on I2C at 0x77 by default (SDO high) or 0x76 with SDO low.
  Source: SPA06-003 wiki and `SPL07-003.h` address constants.
- The pinned `Seeed Arduino SPA06` 1.0.1 library verifies device ID 0x11
  during `begin()`. Source: library `SPL07-003.cpp` / `SPL07-003.h`.
- The sketch configures continuous measurement at 4 Hz, 32 pressure samples
  and one temperature sample. These are application settings, not the only
  valid SPA06 configuration. Source: library API constants.
- `calcAltitude()` uses 1013.25 hPa as standard sea-level pressure. Source:
  library implementation.

## Build

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/air_pressure_spa06
```

## Expected behavior

- Setup probes 0x77 first, then 0x76, and prints
  `SPA06-003 on I2C 0x77` or `SPA06-003 on I2C 0x76`.
- While measurements are available, the sketch prints lines such as:

```text
Pres: 100000.0 Pa  Temp: 25.0 C  Altitude: 123.4 m
```

- If neither address answers, it prints
  `SPA06-003 not found at 0x77 or 0x76` and stops printing readings. It does
  not retry, so power-cycle after fixing the wiring.

## Physical test

1. Run `i2c_scan` and record whether 0x77 or 0x76 is present.
2. Open the 9600-baud monitor and confirm the detected address.
3. Confirm lines repeat without blocking; the configured rate is 4 Hz but
   serial printing dominates the loop.
4. Carry the kit up or down a stair flight and compare the pressure and
   altitude trend after several samples.

## Limitations

- Altitude is barometric and relative to 1013.25 hPa, not GPS altitude and
  not QNH-corrected.
- The sensor is not calibrated for laboratory absolute accuracy; relative
  changes are more meaningful than the absolute number.
- The fixed 4 Hz/32-sample configuration is a demo choice; changing it
  requires re-checking noise and response time.
- An ACK at 0x77/0x76 is not an identity check; the sketch requires the SPA06
  library's device ID 0x11 match before reporting measurements.

<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# accelerometer — Agent Notes

## Purpose

Read the kit LIS3DHTR accelerometer and use an uncalibrated tilt/shake
heuristic to drive the kit LED.

## Hardware facts

- Grove 3-Axis Digital Accelerometer is a ST LIS3DHTR on I2C at 0x19. Source:
  kit wiki default-pin table and Lesson 10.
- The pinned
  `Grove-3-Axis-Digital-Accelerometer-2g-to-16g-LIS3DHTR` 1.2.4 library
  defaults to 0x18, so the sketch explicitly calls `lis.begin(Wire, 0x19)`.
  Source: library `LIS3DHTR.h` default address and `LIS3DHTR.cpp`.
- The kit LED is on D4, active high. Source: kit wiki default-pin table.
- The sketch configures the 2 g range, 50 Hz output data rate and high
  resolution mode. These are application settings, not the sensor's only
  valid mode. Source: library API.

## Build

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/accelerometer
```

## Expected behavior

- Setup prints `LIS3DHTR on I2C 0x19` when the sensor responds.
- Every 200 ms the sketch prints one line such as:

```text
X=0.00 g  Y=0.00 g  Z=1.00 g  |a|=1.00 g  LED=REST
```

- At rest the magnitude is near 1 g, but which axis reads 1 g depends on how
  the board is mounted.
- The LED turns on when `||a| - 1 g| > 0.20 g`, or when `|X| > 0.60 g`, or
  when `|Y| > 0.60 g`. The serial field changes to `LED=ACTIVE`.
- If the sensor is absent, setup prints
  `LIS3DHTR not found at I2C 0x19`, the LED stays off, and no samples are
  printed.

## Physical test

1. Run `i2c_scan` and confirm 0x19.
2. Open the 9600-baud monitor and confirm the magnitude is near 1 g at rest.
3. Tilt the kit slowly; record which axis changes and in which direction.
4. Tap or shake gently; confirm the LED flashes `ACTIVE` and returns to
   `REST` when motion stops.

## Measured on 2026-10-29

Seeeduino Lotus v1.1 sitting flat: `X≈-0.01 g`, `Y≈0.14 g`, `Z≈0.98 g`,
`|a|≈1.00 g`, `LED=REST`. The ~0.14 g Y bias is the part offset, not tilt.

The kit board lying flat therefore reads roughly Y/Z, not X/Z. Tilting and
shaking moved X over -0.82 to +1.16 g, Z over 0.05 to 1.40 g and |a| over
0.14 to 1.60 g; every sample during the motion reported `LED=ACTIVE` and the
board returned to `LED=REST` when still. After the shake the rest offset
moved to `X≈-0.03 g`, `Y≈0.12 g`. Source: measured result.

## Limitations

- This is not a calibrated inclinometer or shake detector; offsets, scale
  errors and mounting orientation are not corrected.
- The 0.20 g and 0.60 g thresholds are demo heuristics, not sensor limits.
- The 2 g range saturates on hard impacts; do not use it for shock or impact
  measurement.
- The library must be given 0x19 explicitly; using its 0x18 default will
  fail on this kit.
- A successful compile is not a hardware pass; record the axis orientation
  on the actual unit.

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

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example accelerometer`. The exact core and libraries are declared in target.json.

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

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

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

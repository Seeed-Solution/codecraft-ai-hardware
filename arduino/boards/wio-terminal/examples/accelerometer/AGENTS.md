<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# accelerometer — Agent Notes

## Purpose

Read the onboard ST LIS3DHTR directly over `Wire1` without an external sensor
library. The sketch proves the bus, address, `WHO_AM_I` value and basic
acceleration conversion.

## Hardware facts

- LIS3DHTR: `Wire1`, SDA GPIO 78, SCL GPIO 77, 7-bit address `0x18`.
  INT1 is GPIO 79 but is unused here. Source: schematic v1.2 sheet 07 and
  core 1.8.6 `variant.h` for the bus/pins; address `0x18` is
  `LIS3DHTR_DEFAULT_ADDRESS` in Seeed_Arduino_LIS3DHTR (`0x19` is the
  library's alternate `LIS3DHTR_ADDRESS_UPDATED`).
  See [`../../README.md`](../../README.md).
- The sketch sets `CTRL_REG1` to `0x57`: 100 Hz ODR, normal mode, X/Y/Z
  enabled. `CTRL_REG4` `0x80` enables block-data update. The default
  full-scale range is +/-2 g, so the conversion is `raw / 16000` g.
- `WHO_AM_I` register `0x0F` must read `0x33`.
- `readRegisters()` sets bit 7 on the register byte (`reg | 0x80`) so a
  multi-byte read auto-increments, matching Seeed's
  `readRegisterRegion()` implementation.

## Build

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/accelerometer
```

No external library is required. See the board [AGENTS.md](../../AGENTS.md)
for authorized upload and setup instructions.

## Expected behavior

- LCD title: `LIS3DHTR ACCELEROMETER`.
- Serial prints `LIS3DHTR online at 0x18 on Wire1` after a successful probe.
- About ten samples per second print as `x=... y=... z=...` in g.
- The LCD shows X/Y/Z, `|a|`, and an X/Y dot inside a square.
- With the board still, the magnitude should be near 1 g; tilting changes
  the axes and moves the dot.
- With `setRotation(3)`, the measured screen mapping is `displayX = -yg` and
  `displayY = xg`; Z is unchanged. The LIS3DHTR's Y axis points across the
  screen and its X axis points down the screen, and the 180-degree screen
  rotation reverses the horizontal direction.

## On-device verification

1. Build and upload with explicit authorization; open serial at 115200 baud.
2. Keep the board flat and still. Confirm the online message and a stable
   magnitude near 1 g.
3. Tilt once around each axis. Confirm the dot moves in the downhill
   direction with the measured mapping `displayX = -yg`, `displayY = xg`.
4. If the probe repeats `NOT FOUND at 0x18`, check `Wire1` and the address;
   do not switch to Grove `Wire` (GPIO 46/45).

Verified on 2026-10-28: the probe succeeded at `0x18` with `WHO_AM_I` `0x33`.
The final screen mapping was corrected on hardware to `displayX = -yg` and
`displayY = xg`; the dot then rolled in the correct downhill direction when
the board was tilted. The user confirmed the final behavior.

## Limits

- Polling at 10 Hz; INT1 and the FIFO are not used.
- Fixed +/-2 g range and a 100 Hz sensor ODR; changing either requires a
  matching scaling/configuration change.
- No offset, temperature or absolute-accuracy calibration was performed.

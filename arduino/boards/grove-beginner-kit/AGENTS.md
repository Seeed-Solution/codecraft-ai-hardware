<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# Grove Beginner Kit Agent Guide

This file applies to `arduino/boards/grove-beginner-kit/` and every example
below it. The root `AGENTS.md`, `arduino/AGENTS.md`, and
`docs/development/ai-guide.md` still apply.

## Target kit revision

This board directory supports **only the post-October-2025 Grove Beginner Kit
for Arduino revision**. The supported sensors are DHT20 at I2C 0x38 and
SPA06-003 at I2C 0x77/0x76. The 2020 DHT11/BMP280 revision is out of scope:
do not add legacy examples, compatibility branches, or pin remaps for it.

Run `i2c_scan` on the target unit before sensor work. Require 0x38 for DHT20
and require the SPA06 library's device-ID 0x11 check for SPA06-003. The
published kit schematic is the 2020 revision and does not document these
sensor connections; the current wiki plus the target unit's scan are the
evidence for the revision.

## Read first

1. `git status --short --branch`; preserve unrelated work.
2. This directory's `README.md` and its row in `docs/vendor-sources.md`.
3. The example's own `AGENTS.md` before editing its sketch.
4. The relevant vendor source: kit wiki pin table, kit schematic, Lotus v1.1
   schematic, or the pinned library's source.

Never invent a pin, bus, address, sensor variant, polarity, or unit. If the
sources do not define it, ask the user or measure it.

## Fixed platform facts

- Build core: `arduino:avr@1.8.8`.
- Build FQBN: `arduino:avr:uno`.
- Vendor board definition: `Seeeduino:avr:Lotus`; it matches the UNO target
  in MCU, clock, variant, flash/RAM limits and Optiboot upload parameters.
  The legacy core's i386 toolchain and missing 1.3.0 archive make it
  unsuitable as the repository build core.
- MCU: ATmega328P-MU, 16 MHz, 32,256-byte sketch maximum, 2 KB SRAM,
  1 KB EEPROM, 5 V logic.
- Serial monitor: 9600 baud.
- LCD: none. The only display is the 128x64 I2C OLED.
- No radio, cloud, pairing, account, camera, motor, relay, or audio recorder
  is present. Do not add such support.

## Pin and bus ownership

| Resource | Pins / instance | Rule |
|---|---|---|
| LED | D4, active high | on/off only; D4 is not PWM |
| Button | D6, pressed HIGH | use `INPUT`; module has an external pull-down |
| Potentiometer | A0 / ADC0 | 10-bit raw, 5 V nominal reference |
| Light sensor | A6 / ADC6 | raw only; not lux |
| Sound sensor | A2 / ADC2 | analog envelope; not dB SPL or audio |
| Buzzer | D5, passive | use `tone()`; never hold a static DC level |
| I2C | A4 SDA/pin 18, A5 SCL/pin 19 | 5 V bus shared by all I2C modules |
| OLED | I2C 0x3C default, 0x3D alternate | U8x8 text mode for SRAM budget |
| Accelerometer | LIS3DHTR I2C 0x19 | do not use the library's 0x18 default |
| Pressure | SPA06-003 at 0x77/0x76 | require device ID 0x11 |
| Temperature/humidity | DHT20 at I2C 0x38 | probe 0x38 before initialization |

## SRAM and flash discipline

- Use U8x8 text mode, not a full U8g2 framebuffer, unless the measured SRAM
  headroom is documented.
- One example should link only the libraries it needs. The AVR has 32 KB
  flash and 2 KB SRAM; the current largest sketch is well below that but not
  an invitation to add libraries casually.
- Avoid `String`, large global buffers, and unbounded serial formatting.

## Build and upload

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/<sketch>

./tools/validate.sh --arduino grove-beginner-kit
```

Do not run `upload`, `flash`, or avrdude without explicit authorization for
the current session. Confirm the port with `arduino-cli board list` first. A
successful compile is not hardware validation.

## Verification contract

For every new or changed demo:

1. Compile it with `arduino:avr@1.8.8` and FQBN `arduino:avr:uno`.
2. State the exact 9600-baud serial lines and any OLED text.
3. Give physical steps: cover the light sensor, speak near the sound sensor,
   press the button, rotate A0, listen to D5, tilt the accelerometer, or run
   `i2c_scan`.
4. State what must not be inferred: raw ADC is not lux, sound envelope is not
   dB SPL, barometric altitude is not GPS altitude, and a compile is not an
   on-device pass.
5. Keep the bilingual `AGENTS.md` pair aligned.
6. Report static checks, board build, and on-device result as three separate
   items.

## Known compile and runtime pitfalls

- **Do not use `INPUT_PULLUP` on D6.** The kit button module already has a
  pull-down; enabling the internal pull-up makes the idle state look pressed.
- **Do not use `analogWrite(D4, ...)`.** D4 is not a PWM pin. On AVR,
  `analogWrite` on a non-PWM pin becomes a digital on/off output and will not
  dim the LED.
- **The DHT20 library has fixed address 0x38.** Probe it before `begin()` so
  a missing sensor produces a clear serial error instead of a hang.
- **SPA06 1.0.1 emits an AVR `Wire::requestFrom` overload note.** It is a
  library warning, not a build failure; do not patch the pinned library to
  silence it.
- **The OLED can be at 0x3D.** `oled_status` switches U8x8 to the 8-bit write
  address 0x7A when only 0x3D responds. Keep the address mapping if extending
  the example.
- **Share the I2C bus, not the pins.** OLED, accelerometer, pressure and DHT20
  all use A4/A5. There is no second hardware I2C peripheral.
- **Do not reintroduce the 2020 DHT11/BMP280 demos.** They use a different
  hardware revision and are intentionally absent from this directory.

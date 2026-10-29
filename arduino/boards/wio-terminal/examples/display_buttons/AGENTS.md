<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# display_buttons — Agent Notes

## Purpose

Exercise the ILI9341 display, the three top buttons and the five-way switch
in one low-flicker sketch. This is the reference demo for LCD orientation,
button polarity and debounce behavior.

## Hardware facts

- LCD: 2.4-inch 320x240 ILI9341 on SPI3; SCK 68, MOSI 67, MISO 66, CS 69,
  DC 70, reset 71, backlight 72. `setRotation(3)` produces the 320-pixel-wide
  landscape view used by all examples. Source: schematic v1.2 sheet 07,
  core 1.8.6 `variants/wio_terminal/variant.h` and bundled LCD
  `User_Setup.h`; see [`../../README.md`](../../README.md).
- Top buttons: A=GPIO 28, B=29, C=30. Source: schematic sheet 05 and core
  `variant.h` (`WIO_KEY_A/B/C`).
- Five-way switch: up=31, left=32, right=33, down=34, press=35. Source:
  schematic sheet 04 and core `variant.h`
  (`WIO_5S_*`).
- Every switch is active low and has a pull-up. The sketch also enables the
  SAMD51 internal pull-up and debounces each input for 20 ms.

## Build

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/display_buttons
```

No external library is needed beyond the LCD library bundled in
`Seeeduino:samd@1.8.6`. Use the board [AGENTS.md](../../AGENTS.md) for the
authorized upload procedure and toolchain setup.

## Expected behavior

- LCD title: `WIO TERMINAL INPUT`.
- Gray key boxes for `A B C`, `UP LEFT PRESS RIGHT DOWN`; the active box
  turns green while held.
- Serial at 115200 baud prints one line on each stable transition, for
  example `UP pressed` and `UP released`.

## On-device verification

1. Build and upload with explicit authorization.
2. Open the serial monitor at 115200 baud.
3. Press and release each of the eight inputs once.
4. Confirm exactly one press line and one release line per action, with the
   matching LCD box changing state.
5. Confirm the display is landscape and all labels fit without clipping.

Verified on 2026-10-28: the landscape LCD rendered the full input map, A/B/C
and five-way events appeared on both LCD and serial, and the user confirmed
the demo. No electrical timing measurement was made.

## Limits

- Polling only; it does not use interrupts or wake the MCU from sleep.
- A 20 ms debounce window is intentional; do not shorten it without testing
  the actual switches.
- Debounce was confirmed by human input, not with a logic analyzer; the 20 ms
  window remains an empirical value.

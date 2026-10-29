<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# led_blink — Agent Notes

## Purpose

Minimal digital-output test for the kit's Grove LED.

## Hardware facts

- Grove LED is wired to D4 and is active high. Source: kit wiki default-pin
  table and Lesson 1.
- D4 is not a PWM pin on the ATmega328P core, so this example only blinks.
  Source: kit wiki Lesson 4 PWM-pin list and the core `standard` variant.
- The LED module has an adjustable brightness trimpot. Source: kit wiki
  Lesson 1.

## Build

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/led_blink
```

## Expected behavior

- LED is on for 500 ms, then off for 500 ms (1 Hz blink).
- Serial at 9600 baud prints `LED=ON` and `LED=OFF` once per phase, after the
  heading `led_blink: Grove LED on D4`.

## Physical test

1. Power the kit and open the 9600-baud monitor.
2. Confirm a steady 1 Hz blink.
3. If visible but dim, adjust the trimpot on the LED module; do not change
   the GPIO drive in software.

## Limitations

- This proves only D4 drive and the LED module path, not PWM brightness.
- A broken stamp-hole trace or dead LED will look like a software failure;
  check the module and D4 continuity before changing the sketch.

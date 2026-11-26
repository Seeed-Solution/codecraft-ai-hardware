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

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example led_blink`. The exact core and libraries are declared in target.json.

## Expected behavior

- LED is on for 500 ms, then off for 500 ms (1 Hz blink).
- Serial at 9600 baud prints `LED=ON` and `LED=OFF` once per phase, after the
  heading `led_blink: Grove LED on D4`.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- This proves only D4 drive and the LED module path, not PWM brightness.
- A broken stamp-hole trace or dead LED will look like a software failure;
  check the module and D4 continuity before changing the sketch.

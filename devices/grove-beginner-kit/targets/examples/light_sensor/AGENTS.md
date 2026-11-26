# light_sensor — Agent Notes

## Purpose

Read the kit light sensor and use a dark threshold to switch the kit LED.

## Hardware facts

- Grove Light is wired to A6 (ADC6). Source: kit wiki default-pin table and
  Lesson 5.
- The vendor example turns the LED on when raw < 200 and off otherwise.
  Source: kit wiki Lesson 5.
- Grove LED is on D4, active high. D4 is not PWM. Source: kit wiki.

## Build

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example light_sensor`. The exact core and libraries are declared in target.json.

## Expected behavior

Every 250 ms at 9600 baud the sketch prints:

```text
A6 raw=123 mV=601 LED=ON
```

The LED turns on below raw 200 and off above raw 260. The two thresholds add
hysteresis. Covering the sensor should lower the light level; observe the raw
direction on the actual unit rather than assuming a lux conversion.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- Raw ADC and estimated millivolts are not lux.
- The 200/260 thresholds are application choices; adjust after measuring the
  actual enclosure and lighting.
- This does not test D4 PWM because D4 has no PWM function.

# potentiometer — Agent Notes

## Purpose

Read the kit's rotary potentiometer and demonstrate a calibrated-threshold
digital action without PWM.

## Hardware facts

- Grove Rotary Potentiometer is wired to A0. Source: kit wiki default-pin
  table and Lesson 3.
- The ATmega328P ADC returns 0-1023 against the 5 V reference in this kit.
  Source: kit wiki Lesson 3 and the core `standard` variant.
- Grove LED is on D4, which is not a PWM pin. Source: kit wiki Lesson 4.

## Build

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example potentiometer`. The exact core and libraries are declared in target.json.

## Expected behavior

At 9600 baud, every 200 ms the sketch prints a line such as:

```text
A0 raw=512 mV=2502 pct=50 LED=ON
```

The LED turns on above raw 532 and off below raw 492. The two thresholds
provide hysteresis around midpoint. Endpoints may not reach exactly 0 or
1023.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- This is not PWM dimming. Rewiring the LED module to D3 is required for PWM.
- Millivolts use a nominal 5000 mV reference and are approximate.
- The midpoint thresholds are application choices, not electrical limits.

<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/potentiometer
```

## Expected behavior

At 9600 baud, every 200 ms the sketch prints a line such as:

```text
A0 raw=512 mV=2502 pct=50 LED=ON
```

The LED turns on above raw 532 and off below raw 492. The two thresholds
provide hysteresis around midpoint. Endpoints may not reach exactly 0 or
1023.

## Physical test

1. Open the 9600-baud monitor.
2. Rotate the potentiometer from stop to stop and record the raw range.
3. Confirm the LED switches near the middle. Do not assume clockwise means
   increasing until observed on the unit.

## Limitations

- This is not PWM dimming. Rewiring the LED module to D3 is required for PWM.
- Millivolts use a nominal 5000 mV reference and are approximate.
- The midpoint thresholds are application choices, not electrical limits.

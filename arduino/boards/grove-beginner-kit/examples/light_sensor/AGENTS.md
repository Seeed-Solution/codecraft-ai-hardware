<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/light_sensor
```

## Expected behavior

Every 250 ms at 9600 baud the sketch prints:

```text
A6 raw=123 mV=601 LED=ON
```

The LED turns on below raw 200 and off above raw 260. The two thresholds add
hysteresis. Covering the sensor should lower the light level; observe the raw
direction on the actual unit rather than assuming a lux conversion.

## Physical test

1. Open the 9600-baud monitor in room light and record the raw value.
2. Cover the sensor with your hand; confirm the raw value moves toward the
   dark threshold and the LED turns on.
3. Remove your hand; confirm the LED turns off above the upper threshold.

## Measured on 2026-10-29

Seeeduino Lotus v1.1, indoor room light: A6 reads raw 776-781 (3.8 V) with
`LED=OFF`. A hand fully covering the photoresistor drives A6 to 0-26 and the
LED turns on; a brief partial cover gave raw 76-118, which also lit the LED.
Removing the hand restores raw 779 and `LED=OFF`. Source: measured result.

Two traps worth knowing when repeating this:

- Covering the wrong module leaves A6 unchanged at about 777. The
  photoresistor is the small disc with a zigzag pattern.
- A6 and A7 both respond to the same photoresistor; A7 falls only to about
  raw 80. Use A6, which is the documented pin.

## Limitations

- Raw ADC and estimated millivolts are not lux.
- The 200/260 thresholds are application choices; adjust after measuring the
  actual enclosure and lighting.
- This does not test D4 PWM because D4 has no PWM function.

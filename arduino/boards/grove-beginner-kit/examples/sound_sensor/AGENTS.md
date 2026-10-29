<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# sound_sensor — Agent Notes

## Purpose

Observe the analog sound envelope and trigger the LED on a short sound peak.

## Hardware facts

- Grove Sound is wired to A2 (ADC2). Source: kit wiki default-pin table and
  Lesson 6.
- The vendor example uses raw > 400 to turn on the LED. Source: kit wiki
  Lesson 6.
- The output is an analog sound envelope, not a digital microphone stream.
  Source: kit wiki Lesson 6 (analog signal and LED practice).
- Grove LED is on D4, active high. Source: kit wiki.

## Build

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/sound_sensor
```

## Expected behavior

The sketch takes a 50 ms peak window from A2 and prints one line per window at
9600 baud:

```text
A2 peak=180 LED=OFF
```

When the peak exceeds raw 400, the LED turns on for at least another 100 ms.
Use the serial plotter to see the envelope change while speaking or clapping.

## Physical test

1. Open the 9600-baud monitor or plotter in a quiet room.
2. Clap or speak near the sound sensor; confirm the printed peak rises and
   the LED turns on.
3. Let the room settle; confirm the LED returns off after the hold time.

## Measured on 2026-10-29

Seeeduino Lotus v1.1, quiet room: 50 ms peaks stay at 100-170 with
`LED=OFF`. Two hand claps about 20 cm from the microphone produced
`peak=561` and `peak=686`, each followed by `LED=ON`; the next samples held
`LED=ON` at `peak=162` and `282` before returning to `LED=OFF`. Source:
measured result.

The 400 threshold therefore has roughly 2x headroom over room noise and the
clap envelope reaches well above it. A clap from across the room does not
reliably trigger; bring the hand close to the microphone capsule.

## Limitations

- This is not calibrated dB SPL and cannot record audio.
- The raw-400 threshold is a vendor example, not a universal room threshold;
  record the quiet/peak values and adjust only after measurement.
- The sensor's analog output may saturate; do not infer waveform shape from
  the peak value.

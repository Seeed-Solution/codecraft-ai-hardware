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

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example sound_sensor`. The exact core and libraries are declared in target.json.

## Expected behavior

The sketch takes a 50 ms peak window from A2 and prints one line per window at
9600 baud:

```text
A2 peak=180 LED=OFF
```

When the peak exceeds raw 400, the LED turns on for at least another 100 ms.
Use the serial plotter to see the envelope change while speaking or clapping.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- This is not calibrated dB SPL and cannot record audio.
- The raw-400 threshold is a vendor example, not a universal room threshold;
  record the quiet/peak values and adjust only after measurement.
- The sensor's analog output may saturate; do not infer waveform shape from
  the peak value.

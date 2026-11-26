# buzzer_tones — Agent Notes

## Purpose

Verify the kit's passive piezo buzzer with a reproducible tone sequence.

## Hardware facts

- Grove Buzzer is wired to D5. Source: kit wiki default-pin table and
  Lesson 4.
- The kit buzzer is passive: it needs an AC/square wave and will not produce
  a sustained tone from a static HIGH level. Source: kit wiki Lesson 4.
- The sketch uses the Arduino `tone()` API on D5.

## Build

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example buzzer_tones`. The exact core and libraries are declared in target.json.

## Expected behavior

- Four audible tones in order: 440 Hz/250 ms, 880 Hz/250 ms, 1760 Hz/250 ms,
  2000 Hz/400 ms, with 80 ms gaps between tones.
- Then 1.5 s of silence, repeated.
- Serial at 9600 baud prints each frequency and `silence 1500 ms`.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- `tone()` fixes the waveform and voltage; the sketch does not control
  perceived volume.
- Do not combine this example with other Timer2-dependent PWM without
  checking the AVR timer allocation.
- A successful compile does not prove the buzzer or its transistor path.

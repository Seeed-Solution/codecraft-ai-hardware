<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/buzzer_tones
```

## Expected behavior

- Four audible tones in order: 440 Hz/250 ms, 880 Hz/250 ms, 1760 Hz/250 ms,
  2000 Hz/400 ms, with 80 ms gaps between tones.
- Then 1.5 s of silence, repeated.
- Serial at 9600 baud prints each frequency and `silence 1500 ms`.

## Physical test

1. Open the 9600-baud monitor and listen.
2. Confirm four distinct ascending tones and the silent gap.
3. If no sound, verify D5 and the passive buzzer module; do not replace
   `tone()` with a static `digitalWrite(HIGH)`.

## Limitations

- `tone()` fixes the waveform and voltage; the sketch does not control
  perceived volume.
- Do not combine this example with other Timer2-dependent PWM without
  checking the AVR timer allocation.
- A successful compile does not prove the buzzer or its transistor path.

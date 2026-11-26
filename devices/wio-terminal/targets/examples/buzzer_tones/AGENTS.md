# buzzer_tones — Agent Notes

## Purpose

Drive the onboard passive buzzer with finite tones controlled by the three top
buttons. It demonstrates the correct AC/PWM drive and safe stop behavior.

## Hardware facts

- `WIO_BUZZER` is GPIO 12 / TCC0 channel 4. It drives Q4 and passive buzzer
  B4013AM422-042. Source: schematic v1.2 sheet 04, core 1.8.6
  `variants/wio_terminal/variant.h`, and the core pin table; see
  [`../../README.md`](../../README.md).
- The buzzer is passive: a DC level produces no sustained tone. This sketch
  uses the SAMD core `tone()` timer API.
- Top buttons A, B and C are GPIO 28, 29 and 30, active low with pull-ups.

## Build

From the repository root, run `codecraft validate --target arduino/wio-terminal --example buzzer_tones`. The exact core and libraries are declared in target.json.

## Expected behavior

- LCD title: `PASSIVE BUZZER`; prompt: `A: 440 Hz   B: 4 kHz   C: stop`.
- A plays 440 Hz for 250 ms, B plays 4 kHz for 120 ms, and C calls `noTone()`
  then drives GPIO 12 low.
- Serial prints `tone 440 Hz for 250 ms`, `tone 4000 Hz for 120 ms` or
  `buzzer stopped` for the corresponding button.
- The last-action field changes colour for each command.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/wio-terminal/observations.md); a successful build does not certify physical behavior.

## Limits

- Only three fixed tones; there is no melody, volume control or audio codec.
- `tone()` uses a hardware timer resource. If adding PWM-dependent code,
  check timer conflicts before reusing GPIO 12.
- B is high-pitched and brief; a phone microphone or hearing range may make
  the result hard to judge. Verify with the physical board.
- No on-device volume or frequency measurement was made; audibility was
  confirmed by the user.

<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/buzzer_tones
```

No external library is required. See the board [AGENTS.md](../../AGENTS.md)
for authorized upload and setup.

## Expected behavior

- LCD title: `PASSIVE BUZZER`; prompt: `A: 440 Hz   B: 4 kHz   C: stop`.
- A plays 440 Hz for 250 ms, B plays 4 kHz for 120 ms, and C calls `noTone()`
  then drives GPIO 12 low.
- Serial prints `tone 440 Hz for 250 ms`, `tone 4000 Hz for 120 ms` or
  `buzzer stopped` for the corresponding button.
- The last-action field changes colour for each command.

## On-device verification

1. Build and upload with explicit authorization; open serial at 115200 baud.
2. Press A and confirm a short, audible 440 Hz tone and the serial line.
3. Press B and confirm a higher-pitched, shorter beep.
4. Press A or B and then C; confirm the output stops and GPIO 12 is low.
5. Check that no tone continues indefinitely after a button action.

Verified on 2026-10-28: the board printed `tone 440 Hz for 250 ms`,
`tone 4000 Hz for 120 ms`, and `buzzer stopped` for repeated A/B/C presses.
The user confirmed that both tones were audible, the LCD action field followed
each press, and no tone continued after its finite duration or after C.

## Limits

- Only three fixed tones; there is no melody, volume control or audio codec.
- `tone()` uses a hardware timer resource. If adding PWM-dependent code,
  check timer conflicts before reusing GPIO 12.
- B is high-pitched and brief; a phone microphone or hearing range may make
  the result hard to judge. Verify with the physical board.
- No on-device volume or frequency measurement was made; audibility was
  confirmed by the user.

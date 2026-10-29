<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# microphone_level — Agent Notes

## Purpose

Measure the onboard analog microphone as a relative sound-level meter. The
sketch removes the ADC DC bias and reports peak-to-peak counts, RMS counts and
a colour-changing bar.

## Hardware facts

- The onboard electret microphone feeds an LMV358 gain stage; its output is
  `MIC_OUTPUT`, connected to `WIO_MIC` (GPIO 39 / ADC channel 12).
  Source: schematic v1.2 sheet 04 and core 1.8.6
  `variants/wio_terminal/variant.h`; see
  [`../../README.md`](../../README.md).
- The sketch reads 200 samples with 40 microseconds between reads (about an
  8 ms window) once every 100 ms.
- ADC resolution is 12 bits with a nominal 3.3 V reference. The displayed mV
  value is an estimate, not a calibrated acoustic measurement.

## Build

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/microphone_level
```

No external library is required. See the board [AGENTS.md](../../AGENTS.md)
for authorized upload and setup.

## Expected behavior

- LCD title: `MICROPHONE LEVEL`; it shows peak-to-peak, RMS and `relative
  level` with a green/yellow/red bar.
- Serial prints `mic_mean=... pp=... rms=...` about ten times per second.
- Speaking, whistling or tapping near the microphone increases the bar and
  the peak-to-peak/RMS values. Room silence should sit near zero after the DC
  term is removed.

## On-device verification

1. Build and upload with explicit authorization; open serial at 115200 baud.
2. With the room quiet, record a short baseline for mean, peak-to-peak and
   RMS.
3. Speak or whistle at a normal distance. Confirm all three values rise and
   fall with the sound.
4. Stop making noise and confirm the values return near the baseline.
5. If the signal is stuck at 0 or 4095, inspect the microphone path and the
   analog reference before changing the sketch.

Verified on 2026-10-28: with the room quiet, peak-to-peak was approximately
162-666 counts and RMS approximately 37-129 counts. During speech/whistling,
peak-to-peak reached 3458 counts and RMS reached approximately 1578 counts;
after the sound stopped, both returned to the quiet baseline. The user also
confirmed that the LCD level bar followed the sound.

## Limits

- Monophonic analog envelope only; it is not a digital I2S microphone and
  cannot record audio without a sampler and storage design.
- No dB SPL calibration, A-weighting or frequency response correction.
- The 8 ms sampling window is not a continuous capture; short transients can
  be missed.
- The levels are relative ADC measurements; the observed baseline varies with
  room noise and microphone DC bias.

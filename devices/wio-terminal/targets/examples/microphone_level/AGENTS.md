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

From the repository root, run `codecraft validate --target arduino/wio-terminal --example microphone_level`. The exact core and libraries are declared in target.json.

## Expected behavior

- LCD title: `MICROPHONE LEVEL`; it shows peak-to-peak, RMS and `relative
  level` with a green/yellow/red bar.
- Serial prints `mic_mean=... pp=... rms=...` about ten times per second.
- Speaking, whistling or tapping near the microphone increases the bar and
  the peak-to-peak/RMS values. Room silence should sit near zero after the DC
  term is removed.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/wio-terminal/observations.md); a successful build does not certify physical behavior.

## Limits

- Monophonic analog envelope only; it is not a digital I2S microphone and
  cannot record audio without a sampler and storage design.
- No dB SPL calibration, A-weighting or frequency response correction.
- The 8 ms sampling window is not a continuous capture; short transients can
  be missed.
- The levels are relative ADC measurements; the observed baseline varies with
  room noise and microphone DC bias.

# light_sensor — Agent Notes

## Purpose

Read the onboard photodiode through the SAMD51 ADC and show a filtered raw
value, an estimated millivolt value and a bar. This is a relative-light demo,
not a calibrated lux meter.

## Hardware facts

- Signal: `WIO_LIGHT`, GPIO 27 / ADC channel 15, from the photodiode front
  end on schematic v1.2 sheet 05. Source: schematic and core 1.8.6
  `variant.h`; see [`../../README.md`](../../README.md).
- The product page lists a 400-1050 nm sensor response.
- The sketch sets 12-bit ADC resolution and uses a nominal 3.3 V reference.
  The mV display is derived from the ADC count; it is an estimate.
- The demo samples every 100 ms and applies a 3:1 rolling filter.

## Build

From the repository root, run `codecraft validate --target arduino/wio-terminal --example light_sensor`. The exact core and libraries are declared in target.json.

## Expected behavior

- LCD title: `LIGHT SENSOR`; it shows the pin, raw value, filtered mV and a
  green bar.
- Serial prints lines such as `light_raw=1234 filtered=1210.5`.
- Covering and uncovering the sensor window changes the ADC value. The sketch
  deliberately does not assert whether the raw count rises or falls.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/wio-terminal/observations.md); a successful build does not certify physical behavior.

## Limits

- No lux calibration, spectral correction or dark-current calibration.
- The analog reference is treated as nominal 3.3 V; board tolerance affects
  the displayed mV estimate.
- The raw baseline varies with ambient light; no universal threshold is
  provided.

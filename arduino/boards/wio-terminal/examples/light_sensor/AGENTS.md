<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/light_sensor
```

No external library is needed. See the board [AGENTS.md](../../AGENTS.md) for
authorized upload and setup.

## Expected behavior

- LCD title: `LIGHT SENSOR`; it shows the pin, raw value, filtered mV and a
  green bar.
- Serial prints lines such as `light_raw=1234 filtered=1210.5`.
- Covering and uncovering the sensor window changes the ADC value. The sketch
  deliberately does not assert whether the raw count rises or falls.

## On-device verification

1. Build and upload with explicit authorization; open serial at 115200 baud.
2. Record the raw value under normal room light.
3. Fully cover the sensor window and record the new value after it settles.
4. Uncover it and confirm the value returns in the opposite direction.
5. Use the observed direction and range to choose an application threshold;
   do not reuse a threshold from another unit without checking.

Verified on 2026-10-28: raw counts around 14-20 were observed under room
light, and the user confirmed the LCD/serial demo. The cover/uncover
direction was not recorded in this run; measure it before setting a
threshold.

## Limits

- No lux calibration, spectral correction or dark-current calibration.
- The analog reference is treated as nominal 3.3 V; board tolerance affects
  the displayed mV estimate.
- The raw baseline varies with ambient light; no universal threshold is
  provided.

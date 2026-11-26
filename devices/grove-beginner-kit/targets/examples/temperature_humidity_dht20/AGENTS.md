# temperature_humidity_dht20 — Agent Notes

## Purpose

Read the DHT20 temperature/humidity sensor on the only supported kit
revision, post-October-2025.

## Hardware facts

- This board directory targets only the post-October-2025 kit revision. The
  current kit wiki says versions after 2025.10 use DHT20, and a black
  humidity/temperature detector identifies that revision. Source: kit wiki
  version-iteration notes and DHT20 section.
- The DHT20 is on I2C `Wire` and the pinned
  `Grove Temperature And Humidity Sensor` 2.0.2 library uses fixed address
  0x38 (`DEFAULT_IIC_ADDR`). Source: library `DHT.h`.
- The published kit schematic predates this sensor and does not document the
  DHT20 wiring. Confirm 0x38 with `i2c_scan`; do not invent D3 wiring for an
  I2C DHT20. Source: kit schematic archive, kit wiki, and the DHT20 library.
- I2C is shared with the OLED and other modules on A4/pin 18 (SDA) and
  A5/pin 19 (SCL), 5 V logic. Source: Seeeduino Lotus v1.1 schematic.
- The sketch constructs `DHT dht(0, DHT20)`. The first argument is an unused
  constructor parameter in this library mode; DHT20 mode is I2C-only and does not use a data
  pin. Source: library `DHT.cpp` / `DHT.h`.

## Build

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example temperature_humidity_dht20`. The exact core and libraries are declared in target.json.

## Expected behavior

- Setup prints `DHT20 on I2C 0x38` at 9600 baud.
- Every second a successful read prints one line such as:

```text
Humidity: 50.0 %  Temperature: 24.0 C
```

- If 0x38 is absent at boot, the sketch prints
  `DHT20 not found at I2C 0x38` and stays in a one-second delay loop. It does
  not retry, so power-cycle after fixing the wiring.
- A read failure after a successful probe prints
  `DHT20 read failed; check I2C 0x38 and sensor power`.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- This is not a calibrated reference; the DHT20 is for trends and general
  environmental monitoring, not laboratory measurement.
- The fixed 0x38 address means only one DHT20 can share this bus.
- The sketch does not compute dew point or heat index and does not control
  the sensor heater.
- The published schematic does not contain the DHT20 data path; a successful
  compile is not proof that the kit revision is DHT20.

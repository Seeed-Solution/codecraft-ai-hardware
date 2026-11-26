<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# arduino/grove-beginner-kit

Device knowledge: [AGENTS.md](../AGENTS.md). Read the [toolchain setup below](#toolchain), then run `codecraft validate --target arduino/grove-beginner-kit --projects` from the repository root.

The Target manifest owns the FQBN, dependency pins and standalone sketches. Select one example with --example. Builds never upload; on-device procedures belong to the [Device checklist](../AGENTS.md#physical-acceptance).

## Native implementation

- Build identity: `arduino:avr:uno` uses the ATmega328P, standard variant,
  16 MHz, AVR_UNO and 32,256-byte sketch limit. This matches the Lotus v1.1
  board configuration. Source: Seeeduino AVR 1.2.1 boards.txt and
  [ArduinoCore-avr 1.8.8](https://github.com/arduino/ArduinoCore-avr/tree/1.8.8).
  Only the declared core is supported by this Target.

## Toolchain

The Target manifest is the executable version declaration. Use Arduino CLI 1.5.0 with these exact native dependencies. Library versions are copied from the board-maintained dependency declarations; bundled drivers come from the pinned core.

```sh
arduino-cli core update-index
arduino-cli core install arduino:avr@1.8.8
arduino-cli lib install --no-deps "U8g2@2.36.19"
arduino-cli lib install --no-deps "Grove Temperature And Humidity Sensor@2.0.2"
arduino-cli lib install --no-deps "Seeed Arduino SPA06@1.0.1"
arduino-cli lib install --no-deps "Grove-3-Axis-Digital-Accelerometer-2g-to-16g-LIS3DHTR@1.2.4"
```

Use ARDUINO_DIRECTORIES_DATA, ARDUINO_DIRECTORIES_DOWNLOADS and ARDUINO_DIRECTORIES_USER for an isolated installation. The adapter fingerprints the selected core, native tools and pinned libraries, and passes explicit library paths to the compiler. Unrelated installed cores are not part of the lock. Builds write under .codecraft/builds/arduino/.

Source: https://github.com/arduino/ArduinoCore-avr/tree/1.8.8

In an exported application, use `codecraft build-project .`; reference-repository validation commands do not replace application lock checks.

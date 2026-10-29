<p align="right">
  <strong>English</strong> · <a href="README.zh_CN.md">简体中文</a>
</p>

# Grove Beginner Kit for Arduino

Status: **10/10 Grove device demos plus `i2c_scan` are build-verified and
on-device verified on a post-2025 Seeeduino Lotus v1.1 with
`arduino:avr@1.8.8`.** Every kit module has its own example; `i2c_scan` is an
additional bus diagnostic, not an eleventh device. All four I2C devices, the
OLED, the button, a full potentiometer sweep, the buzzer sequence, the light
and sound thresholds, and the accelerometer tilt produced the expected
serial output and physical effect; the evidence is recorded in
[docs/on-device-validation.md](../../../docs/on-device-validation.md). This
board directory targets only the post-October-2025 kit revision with DHT20
and SPA06-003. The 2020 DHT11/BMP280 revision is out of scope.

## Verified facts

- MCU: Microchip ATmega328P-MU, 16 MHz, 32 KB flash (32,256-byte sketch
  maximum), 2 KB SRAM and 1 KB EEPROM, 5 V logic. Seeeduino Lotus v1.0 uses
  a CH340 USB bridge; v1.1 uses CP2102N. Source:
  [Seeeduino Lotus wiki](https://wiki.seeedstudio.com/Seeeduino_Lotus/) and
  [v1.1 schematic](https://files.seeedstudio.com/wiki/Seeeduino_Lotus/res/Seeeduino%20Lotus%20v1.1%20SCH.pdf).
- Board identity: the vendor core defines `Seeeduino:avr:Lotus`. Its
  `boards.txt` entry uses an ATmega328P, `standard` variant, 16 MHz,
  `AVR_UNO`, 32,256-byte maximum sketch and 115200-baud Optiboot upload.
  The repository builds the functionally matching `arduino:avr:uno` target
  with `arduino:avr@1.8.8`. The legacy core's bundled avr-gcc 4.9.2 is an
  i386-only executable on macOS, and the legacy index advertises an avr
  1.3.0 archive URL that returns 404. Source:
  [Seeeduino AVR 1.2.1 archive](https://raw.githubusercontent.com/Seeed-Studio/Seeed_Platform/master/Seeeduino_AVR_v1.2.1.tar.bz2)
  and [ArduinoCore-avr 1.8.8](https://github.com/arduino/ArduinoCore-avr/tree/1.8.8).
- Hardware I2C: A4 is SDA (PC4, Arduino pin 18), A5 is SCL (PC5, pin 19), and
  the Grove connectors are 5 V logic. Source: Seeeduino Lotus v1.1
  schematic and the core's `standard` variant.
- Kit LED: Grove LED on D4, active high, with a board trimpot for brightness.
  D4 is not a PWM pin on this core. Source: kit wiki default-pin table and
  Lesson 1.
- Button: Grove Button on D6. The vendor example configures `INPUT` and reads
  pressed as `HIGH`, so the module provides its own pull-down. Source: kit
  wiki Lesson 2.
- Rotary potentiometer: Grove Rotary Potentiometer on A0; `analogRead()`
  returns 0-1023 using the 5 V reference. Source: kit wiki Lesson 3.
- Buzzer: Grove passive piezo buzzer on D5; it needs a square wave such as
  `tone()`, not a static DC level. Source: kit wiki Lesson 4.
- Light sensor: Grove Light on A6; the vendor example turns the LED on below
  raw 200. This is an uncalibrated analog level, not lux. Source: kit wiki
  Lesson 5.
- Sound sensor: Grove Sound on A2; the vendor example uses raw > 400 to
  indicate a sound peak. The output is an analog envelope, not calibrated
  dB SPL and not recorded audio. Source: kit wiki Lesson 6.
- OLED: 0.96-inch 128x64 SSD1306/SSD1315-compatible display on I2C. The kit
  wiki gives 0x78 as the 8-bit write address, which is 0x3C in 7-bit form;
  0x3D is accepted as an alternate in the demo. Source: kit wiki default-pin
  table and [OLED wiki](https://wiki.seeedstudio.com/Grove-OLED-Display-0.96-SSD1315/).
- Temperature/humidity: the supported post-October-2025 kit uses DHT20 on
  I2C. The current vendor example calls `Wire.begin()`, the Grove DHT library
  uses fixed I2C address 0x38, and the wiki says a black
  humidity/temperature detector identifies this revision. Source: current kit
  wiki DHT20 section and
  [Grove_Temperature_And_Humidity_Sensor](https://github.com/Seeed-Studio/Grove_Temperature_And_Humidity_Sensor).
- Air pressure: the supported post-October-2025 kit uses SPA06-003 on I2C at
  0x77 by default, 0x76 optional. The SPA06 library verifies device ID 0x11.
  Source:
  [SPA06-003 wiki](https://wiki.seeedstudio.com/grove_barometer_sensor_spa06_003/).
- Accelerometer: ST LIS3DHTR on I2C at 0x19. Source: kit wiki default-pin
  table and Lesson 10.

Most unchanged board connections above come from the
[Grove Beginner Kit wiki](https://wiki.seeedstudio.com/Grove-Beginner-Kit-For-Arduino/)
and the kit's
[schematic/PCB archive](https://files.seeedstudio.com/wiki/Grove-Beginner-Kit-For-Arduino/res/Grove-Beginner-Kit-for-Arduino-SCH-PCB.zip)
(the published schematic is dated 2020, revision v1.4). That schematic
predates the DHT20/SPA06-003 revision and does not document their sensor
routing. The current wiki and an `i2c_scan` of the target unit are the
authority for 0x38 and the SPA06-003 address.

## Not provided

The kit contains no Wi-Fi/BLE radio, motor driver, relay, battery/charger,
camera, or digital audio recorder. The "sound sensor" is an analog envelope
output. Do not add network, pairing, account, telemetry, or cloud code: this
repository is a local hardware reference only.

## Toolchain and dependencies

Build target used by this directory:

- Core: `arduino:avr@1.8.8`
- FQBN: `arduino:avr:uno`
- Serial monitor: 9600 baud for every sketch in `examples/`

Vendor IDE board (documented for reference, not used for the repository
build): `Seeeduino:avr:Lotus`, legacy index
`https://raw.githubusercontent.com/Seeed-Studio/Seeed_Platform/master/package_legacy_seeeduino_boards_index.json`.
Pin `Seeeduino:avr@1.2.1` if that board is required; do not use the indexed
1.3.0 entry, whose archive URL is missing.

Install the exact build core and libraries:

```bash
arduino-cli core install arduino:avr@1.8.8
cd arduino/boards/grove-beginner-kit
while IFS= read -r library; do
  [[ -z "$library" || "$library" == \#* ]] && continue
  arduino-cli lib install "$library"
done < libraries.txt
```

| Dependency | Version | Used by |
|---|---|---|
| `arduino:avr` | 1.8.8 | every example |
| `U8g2` | 2.36.19 | `oled_status` |
| `Grove Temperature And Humidity Sensor` | 2.0.2 | `temperature_humidity_dht20` |
| `Seeed Arduino SPA06` | 1.0.1 | `air_pressure_spa06` |
| `Grove-3-Axis-Digital-Accelerometer-2g-to-16g-LIS3DHTR` | 1.2.4 | `accelerometer` |

## Capability matrix

| Capability | Hardware | Default connection | Example | State |
|---|---|---|---|---|
| I2C diagnostics | ATmega328P TWI | SDA A4/pin 18, SCL A5/pin 19 | `i2c_scan` | on-device verified |
| Digital output | Grove LED | D4, active high | `led_blink` | on-device verified; emission not instrumented |
| Digital input | Grove Button | D6, pressed HIGH | `button_led` | on-device verified |
| Analog input | Rotary Potentiometer | A0 | `potentiometer` | on-device verified; full 0-1023 sweep |
| PWM/tone output | Passive buzzer | D5 | `buzzer_tones` | on-device verified; audibility not instrumented |
| Analog sensor | Light sensor | A6 | `light_sensor` | on-device verified; LED on when covered |
| Analog sensor | Sound sensor | A2 | `sound_sensor` | on-device verified; clap peak 561/686 |
| Display | SSD1306/SSD1315 OLED | I2C 0x3C (0x3D alt) | `oled_status` | on-device verified; display confirmed |
| Temperature/humidity | DHT20 | I2C 0x38 | `temperature_humidity_dht20` | on-device verified |
| Pressure | SPA06-003 | I2C 0x77/0x76 | `air_pressure_spa06` | on-device verified |
| Motion | LIS3DHTR | I2C 0x19 | `accelerometer` | on-device verified; axes track tilt |

`on-device verified` means the sketch ran on the target unit on 2026-10-29
and its serial output matched the expected behavior, with the physical
interaction confirmed by the person operating the board. Analog examples
report raw ADC counts rather than engineering units; the measured
room-light, sound-peak and acceleration baselines are recorded in each
example's `AGENTS.md`.

## Build and upload

From the repository root:

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/<sketch>

./tools/validate.sh --arduino grove-beginner-kit
```

On 2026-10-29 all 10 device demos plus `i2c_scan` were uploaded through the
`arduino:avr:uno` target to a Seeeduino Lotus v1.1 using its CP2102N USB
bridge. For a connected board, identify the port and build/upload:

```bash
arduino-cli board list
arduino-cli upload -p <port> \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/led_blink
```

Flashing requires explicit authorization for the current session. If the
portable target cannot upload to a particular Lotus revision, select
`Seeeduino Lotus` in Arduino IDE with the vendor core and use the same sketch;
do not change pins to work around an upload failure.

## Known constraints and source conflicts

- **Only the post-October-2025 revision is supported.** Run `i2c_scan` and
  require DHT20 at 0x38. If it is absent, stop and confirm the hardware
  revision; do not add a DHT11 fallback or apply the 2020 D3 wiring.
- **The published schematic predates the current sensors.** It does not
  document DHT20 or SPA06-003 routing. Treat 0x38 as the DHT20 probe result
  and require the SPA06 library's device-ID 0x11 check before claiming the
  pressure sensor is supported.
- **D4 has no PWM.** The LED demo is on/off only. PWM dimming needs the LED
  module broken out and reconnected to D3.
- **The button module has an external pull-down.** Use `INPUT`; `INPUT_PULLUP`
  would make an idle button read pressed.
- **I2C is shared and 5 V.** Address 0x19/0x3C-or-0x3D/0x38/0x77 may all be
  present on one board. Do not drive 3.3 V-only external devices from this
  bus without level shifting.
- **SRAM is 2 KB.** Use U8x8 text mode or stream readings; do not add a full
  framebuffer or large global buffers without checking the build size.
- **Raw analog values are not engineering units.** The potentiometer, light,
  and sound examples report ADC counts and estimated millivolts only. Lux and
  dB SPL require calibration that this repository has not performed.

## Examples

- `i2c_scan` — scans the ATmega328P hardware I2C bus and labels the kit's
  known 0x19, 0x38, 0x3C/0x3D and 0x76/0x77 devices.
- `led_blink` — blinks the Grove LED on D4 at 1 Hz and prints both phases.
- `button_led` — mirrors the D6 button onto the D4 LED with 20 ms debounce.
- `potentiometer` — reads A0, prints raw/mV/percent and switches the LED at
  the midpoint with hysteresis; no PWM dimming.
- `buzzer_tones` — plays 440, 880, 1760 and 2000 Hz tones on the passive
  D5 buzzer.
- `light_sensor` — reads A6 and turns the LED on in the dark using the vendor
  raw-200 threshold plus hysteresis.
- `sound_sensor` — reports a 50 ms peak from A2 and lights the LED above the
  vendor raw-400 threshold.
- `oled_status` — draws a static title, 128x64/I2C identification and an
  uptime counter on the 0x3C/0x3D OLED.
- `temperature_humidity_dht20` — probes I2C 0x38 and reads the DHT20 every
  second.
- `air_pressure_spa06` — probes 0x77/0x76 and reads the SPA06-003.
- `accelerometer` — reads the LIS3DHTR at 0x19 and lights the LED for an
  uncalibrated tilt/shake heuristic.

Every example has its own bilingual `AGENTS.md` pair with exact hardware
facts, expected serial/OLED output, physical checks and limitations.

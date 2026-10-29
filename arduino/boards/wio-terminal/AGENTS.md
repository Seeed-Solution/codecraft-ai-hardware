<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# Wio Terminal Agent Guide

This file applies to `arduino/boards/wio-terminal/` and every example below
it. The root `AGENTS.md`, `arduino/AGENTS.md`, and
`docs/development/ai-guide.md` still apply.

## Read first

1. `git status --short --branch`; preserve unrelated work.
2. This directory's `README.md` and its row in `docs/vendor-sources.md`.
3. The example's own `AGENTS.md` before editing its sketch.
4. The relevant vendor source: schematic v1.2, core
   `variants/wio_terminal/variant.h`, or the bundled LCD `User_Setup.h`.

Never invent a pin, bus, address, polarity, or hardware capability. If the
four sources above do not define it, ask the user.

## Fixed platform facts

- FQBN: `Seeeduino:samd:seeed_wio_terminal`.
- Core: `Seeeduino:samd@1.8.6`; board-manager URL is recorded in
  `README.md`.
- MCU: ATSAMD51P19A, 512 KB flash, 192 KB RAM, standard 120 MHz.
- Serial monitor: USB CDC at 115200 baud.
- The LCD/Pin setup is bundled inside the core as `Seeed_Arduino_LCD`; do
  not add an upstream `TFT_eSPI` dependency.
- External libraries are allowed only when pinned in `libraries.txt` and
  the board README. Keep sketches standalone and dependency-minimal.
- Do not add touchscreen, battery/charger, speaker/codec, or camera support:
  those functions are not present on this board. See the board README's
  "Not provided" section before inventing a new peripheral.

## Bus and peripheral ownership

| Resource | Pins / instance | Rule |
|---|---|---|
| LCD | SPI3; SCK 68, MOSI 67, CS 69, DC 70, RST 71, BL 72 | use the core's `TFT_eSPI`; do not reconfigure these pins |
| microSD | SPI2; SCK 63, MOSI 62, MISO 61, CS 64, detect 65 | use Seeed Arduino FS with explicit `SDCARD_SS_PIN` + `SDCARD_SPI`; never rely on default `SS` GPIO 50 |
| IMU | `Wire1`; SDA 78, SCL 77, address `0x18`, INT1 79 | internal bus; do not confuse with Grove `Wire` |
| Grove I2C | `Wire`; SDA 46, SCL 45, left J4 | 3.3 V devices only; the IMU is not on this bus |
| Buzzer | GPIO 12, passive | needs `tone()`/PWM; never hold a DC high level to "make sound" |
| IR emitter | GPIO 14, active high through Q13 | transmit-only 940 nm path |
| Buttons | A/B/C 28/29/30; five-way 31/32/33/34/35 | active low; use pull-ups and debounce |
| RTL8720DN | SPI1/UART; CHIP_PU 80, IRQ0 11 | exclusive wireless coprocessor; one wireless demo at a time |

The core defines four SPI peripherals. Do not remap one demo's bus to make a
new peripheral fit: record the conflict and propose a design first.

The LIS3DHTR address is `0x18`, the default in Seeed_Arduino_LIS3DHTR.
The library's `0x19` constant is an alternate address, not the Wio Terminal
default; do not change the address without a successful probe at `0x19`.

## Wireless prerequisite

Arduino upload does not update the RTL8720DN. Wi-Fi and BLE require compatible
eRPC firmware on the coprocessor. Use Seeed's
[Wi-Fi firmware update guide](https://wiki.seeedstudio.com/Wio-Terminal-Network-Overview/)
and the `ambd_flash_tool`; verify it from Arduino with
`rpc_system_version()` before debugging Wi-Fi/BLE code.

`wifi_scan` and `ble_scan` each initialize and own the coprocessor. Run them
separately. The vendor eRPC firmware is documented to support Wi-Fi and BLE
together, but this repository has no tested concurrent example.

## BLE include-order pitfall

The Arduino core may define `min` and `max` macros that break the
`rpcBLE` 1.0.0 headers. `ble_scan` deliberately does this after
`TFT_eSPI.h` and before the rpcBLE includes:

```cpp
#ifdef min
#undef min
#endif
#ifdef max
#undef max
#endif
```

If a new sketch includes rpcBLE, keep the same ordering. Do not include the
rpcBLE headers first and then try to patch compilation errors by renaming
library symbols.

## Build and upload

From the repository root:

```bash
./tools/validate.sh --static
./tools/validate.sh --arduino wio-terminal

arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/<sketch>
```

Do not run `upload`, `flash`, or an RTL firmware tool without explicit user
authorization for the current session. Before any upload, confirm the port
with `arduino-cli board list`; the application USB ID is `2886:802d`, and
the bootloader is `2886:002d`.

## Verification contract

For every new or changed demo:

1. Compile it with the exact FQBN and pinned core.
2. State the expected LCD text/pattern and 115200-baud serial lines.
3. Provide physical steps: which sensor to cover, which key to press, which
   Grove/SD/IR accessory to attach, or which radio environment to use.
4. State what must not be inferred, for example raw ADC is not lux, analog
   microphone level is not recorded audio, and IR transmission needs a
   receiver to prove a valid frame.
5. Keep bilingual `AGENTS.md` and `AGENTS.zh_CN.md` pairs aligned.
6. Report compile, board build, and on-device result as three separate
   items. Never call a successful build hardware validation.

If the wireless demos fail after a successful upload, first verify the RTL
firmware and then the dependency versions; do not randomly change SPI or
UART pins.

## Source priority and known source conflicts

Use: vendor specification / measured result, then the board pin header, then
BSP implementation, then vendor guide/demo. Two known details:

- Schematic page 04 is the five-way switch, buzzer and microphone; page 05
  contains the top A/B/C buttons, Grove headers, light sensor and IR path.
  Do not cite page 05 for the five-way switch.
- The core macro `WIO_LIS3DH_WiRE` is misspelled. Use `Wire1` and the
  separate `WIO_LIS3DH_SDA` / `WIO_LIS3DH_SCL` macros.

When a source disagrees with reality, record the measured result and both
sources in the board README; do not silently "fix" one of them.

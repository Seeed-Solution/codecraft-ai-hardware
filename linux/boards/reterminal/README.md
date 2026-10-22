# reTerminal (CM4)

Status: **Phase 1** — peripheral facts extracted, mainline-only overlay and
demos written from vendor sources. Not yet verified on hardware.

## Verified facts

Source for every row: Seeed-Studio/seeed-linux-dtoverlays
`overlays/rpi/reTerminal-overlay.dts` (file header MIT), pinned at commit
`1d53c3cf2c96` (2026-10-21). The overlay declares "reTerminal V1.9".

| Function | Interface | Address / pin | Notes |
|---|---|---|---|
| Buttons F1–F4 | MCP23008 IO expander, pins 0/2/1/3 (note the 1↔2 swap) | expander @ I2C1 0x38 | active low; KEY_A/S/D/F (codes 30/31/32/33); 5 ms debounce |
| Power button | GPIO13 (SoC) | — | active low, KEY_SLEEP (142), pull-up |
| User LEDs ×3 | MCP23008 pins 4/5/6 | — | active low, `usr_led0..2` |
| Buzzer | MCP23008 pin 7 | — | active **high**, exposed as `usr_buzzer` LED node |
| Expander IRQ | GPIO6 (SoC) | — | falling edge, pull-up |
| Light sensor | LTR-303ALS | I2C1 @0x29 | iio device `ltr303` |
| Accelerometer | LIS331DLH | I2C1 @0x19 | iio device `lis331dlh` |
| RTC | PCF8563 | I2C3 @0x51 | bus at 100 kHz |
| Display + touch | ILI9881x over DSI1, config @ I2C1 0x45 | — | needs Seeed's out-of-tree modules; see `overlays/README.md` |

Board I2C1 runs at 300 kHz per the vendor overlay.

## Layout

- `overlays/reterminal-peripherals-overlay.dts` — mainline-only overlay
  (buttons, LEDs, buzzer, light sensor, accelerometer, RTC, power key).
  Fresh Apache-2.0 implementation from the vendor facts; compiles with a
  bare `dtc`.
- `overlays/README.md` — build/install steps, and when to use Seeed's full
  vendor overlay instead.
- `demos/buttons_evdev.py` — prints button presses via evdev (no deps).
- `demos/leds_buzzer_sysfs.py` — blinks the 3 LEDs and beeps the buzzer
  via `/sys/class/leds` (no deps; restores everything off on exit).
- `demos/sensors_sysfs.py` — reads lux and acceleration via IIO sysfs
  (no deps).
- `demos/header_gpio_blink.py` — libgpiod v2 example on the 40-pin header
  (BCM4 / physical pin 7); shows the chip/line addressing rule.

## Target OS

64-bit Raspberry Pi OS (bookworm or newer; demos assume Python 3 and, for
the header demo, gpiod 2.x). Not yet run on hardware — the first person to
flash it should update this section with the exact image and kernel
version (`uname -a`, `cat /etc/os-release`).

## Quick check after boot

```sh
ls /sys/class/leds/            # usr_led0 usr_led1 usr_led2 usr_buzzer
cat /sys/class/input/event*/device/name | grep gpio-keys
cat /sys/bus/iio/devices/iio:device*/name   # ltr303, lis331dlh
```

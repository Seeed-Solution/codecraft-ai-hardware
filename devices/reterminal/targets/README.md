<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# linux/reterminal

Device knowledge: [AGENTS.md](../AGENTS.md). Hardware facts are maintained there and in its references.

Build/check: `codecraft validate --target linux/reterminal` from the repository root.

See `target.json` for examples, native interface references and exact toolchain selection. Build results and device observations are separate.

## Native implementation

### Layout

- `overlays/reterminal-peripherals-overlay.dts` — mainline-only overlay
  (buttons, LEDs, buzzer, light sensor, accelerometer, RTC, power key).
  Apache-2.0 implementation from the vendor facts; compiles with a
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

### Target OS

64-bit Raspberry Pi OS (bookworm or newer; demos assume Python 3 and, for
the header demo, gpiod 2.x). Physical acceptance must identify the exact image
and kernel version (`uname -a`, `cat /etc/os-release`) in Device evidence.
No current source-bound hardware result is recorded.

### Quick check after boot

```sh
ls /sys/class/leds/            # usr_led0 usr_led1 usr_led2 usr_buzzer
cat /sys/class/input/event*/device/name | grep gpio-keys
cat /sys/bus/iio/devices/iio:device*/name   # ltr303, lis331dlh
```

## Toolchain

Use Python 3 and dtc for host checks. On-device OS/kernel and optional gpiod versions must be recorded on the selected unit; they remain unknown until observed.

In an exported application, use `codecraft build-project .`; reference-repository validation commands do not replace application lock checks.

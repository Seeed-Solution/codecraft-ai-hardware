# reTerminal overlays

Two options, depending on what you need:

## 1. `reterminal-peripherals-overlay.dts` (this directory)

Mainline-only overlay for everything except the display: MCP23008 IO
expander (4 buttons, 3 LEDs, buzzer), LTR-303ALS light sensor, LIS331DLH
accelerometer, PCF8563 RTC, power key on GPIO13.

Build and install on the reTerminal (Raspberry Pi OS 64-bit):

```sh
dtc -@ -I dts -O dtb -o reterminal-peripherals.dtbo reterminal-peripherals-overlay.dts
sudo cp reterminal-peripherals.dtbo /boot/firmware/overlays/
# add to /boot/firmware/config.txt, then reboot:
#   dtoverlay=reterminal-peripherals
```

The file uses plain numeric constants on purpose so a bare `dtc` (no
kernel headers / cpp) can compile it.

## 2. Vendor overlay (display included)

The 5" DSI panel (ILI9881x) and its touch need Seeed's out-of-tree kernel
modules. For those, install the vendor overlay from
[Seeed-Studio/seeed-linux-dtoverlays](https://github.com/Seeed-Studio/seeed-linux-dtoverlays)
(`overlays/rpi/reTerminal-overlay.dts`, file header MIT; the surrounding
repo carries the kernel's GPLv2 COPYING, which is why we link rather than
vendor it). That overlay covers the peripherals too — do not load both.

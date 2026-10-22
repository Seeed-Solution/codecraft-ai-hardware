# SenseCAP Watcher

Status: **Phase 0** — facts verified, BSP extraction pending. The most
capable (and heaviest) board; extract last, after the pilot boards land.

## Verified facts

- SoC: ESP32-S3, 32 MB flash, octal PSRAM (80 MHz) with XIP
  (`SPIRAM_XIP_FROM_PSRAM`, fetch instructions + rodata).
  Source: muse `esp32/devices/sdkconfig.muse-sensecap-watcher`.
- Camera: Himax chip driven over the SSCMA protocol.
  Source: muse `esp32/components/camera/` (`camera_sscma.c`, `sscma_proto.c`).
- LCD, touch, knob, IO expander, audio, battery and SD card pins live in the
  vendor BSP: `SenseCAP-Watcher-Firmware/components/sensecap-watcher/include/sensecap-watcher.h`.
- Power: USB-free power-sweep methodology and on-device findings exist in
  muse `esp32/docs/power/sensecap-watcher*.md` and `ppk2.md`; carry these
  documents over when the board lands.
- ESP-IDF version: TBD (muse defaults vs vendor firmware to be reconciled).

## Vendor source

[Seeed-Studio/SenseCAP-Watcher-Firmware](https://github.com/Seeed-Studio/SenseCAP-Watcher-Firmware);
`examples/factory_firmware/` is the shipping firmware. xiaozhi-esp32's
sensecap-watcher board is a second reference.

## Extraction plan

1. `bsp/include/bsp_pins.h` — from the vendor BSP header.
2. `esp32/components/sscma_client/` — extract from muse `components/camera/`
   with Meta copyright headers retained (Apache 2.0).
3. `bsp/` — display, touch, knob, audio, battery.
4. `examples/` — `display_demo`, `camera_capture`, `battery_status`,
   `knob_input`.

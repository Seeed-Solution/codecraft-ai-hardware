# SenseCAP Watcher

Status: **Phase 1** — camera pipeline landed and build-verified. Display,
touch, knob, audio, battery and SD card are still pending (see the backlog in
docs/). Pin facts for all of them are already in `bsp/include/bsp_pins.h`.

## Verified facts

- SoC: ESP32-S3, 32 MB flash, 8 MB octal PSRAM (80 MHz) with XIP
  (`SPIRAM_XIP_FROM_PSRAM`, fetch instructions + rodata).
  Source: muse `esp32/devices/sdkconfig.muse-sensecap-watcher`.
- Camera: Himax WiseEye2 vision coprocessor, Seeed SSCMA protocol over SPI2
  (12 MHz, mode 0), power/sync/reset on a PCA9535 IO expander (I2C0 @0x21).
  Driver: `esp32/components/sscma_client/` (from muse `components/camera/`,
  Apache 2.0, Meta headers retained). Board glue: `bsp/bsp_camera.c`.
  The chip is reset by power-cycling its rail (muse-verified); the vendor's
  expander reset pin stays an input.
- LCD, touch, knob, IO expander, audio, battery and SD card pins live in the
  vendor BSP: `SenseCAP-Watcher-Firmware/components/sensecap-watcher/include/sensecap-watcher.h`
  and are transcribed with sources into `bsp/include/bsp_pins.h`.
- Power: USB-free power-sweep methodology and on-device findings exist in
  muse `esp32/docs/power/sensecap-watcher*.md` and `ppk2.md`; carry these
  documents over when the power examples land.

## Vendor source

[Seeed-Studio/SenseCAP-Watcher-Firmware](https://github.com/Seeed-Studio/SenseCAP-Watcher-Firmware);
`examples/factory_firmware/` is the shipping firmware. xiaozhi-esp32's
sensecap-watcher board is a second reference.

## Examples

- `camera_capture` — one JPEG over SSCMA: powers the Himax rail, boots the
  chip, samples at 640x480, logs size/dimensions, powers back off.

Build (ESP-IDF v6.0.2):

```sh
source ~/esp/esp-idf-v6.0.2/export.sh
cd esp32/boards/sensecap-watcher/examples/camera_capture
idf.py build flash monitor
```

> **Note** — flashing erases the vendor firmware. The factory image lives in
> the Watcher's `nvsfactory` region (0x9000-0x3B000); back it up first if you
> want to restore it later.

## Remaining extraction plan

1. `bsp/` — display (SPD2010 QSPI 412x412), touch, knob, audio
   (ES8311 + ES7243), battery (ADC on GPIO3 via the divider rail).
2. SD card: SPI2 device 2 (CS=GPIO46, detect on expander pin 4), sharing the
   camera's bus — the camera driver tolerates an already-initialized bus.
3. More examples: `display_demo`, `battery_status`, `knob_input`.

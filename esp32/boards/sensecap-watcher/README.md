# SenseCAP Watcher

Status: **Phase 4** — camera, display, touch, audio, battery and knob
landed and are build-verified. The SD card is still pending; pin facts are
already in `bsp/include/bsp_pins.h`.

Audio note: esp_codec_dev is pinned at 1.6.2 (needs the IDF v6
`i2c_master` bus_handle support); the vendor-pinned 1.2.0 only drives the
legacy I2C driver. The touch component (esp_lcd_touch_spd2010 0.0.1, kept
at the vendor-pinned version) still `#include`s the legacy `driver/i2c.h`,
so builds print IDF's EOL warning; it never calls the legacy API — every
transfer goes through the esp_lcd io handle on the new bus driver.

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
- `display_color_bars` — raw esp_lcd on the SPD2010: vertical color bands
  drawn one row at a time, then a backlight fade. No LVGL.
- `battery_status` — presence (expander BAT_DET) + terminal voltage every
  second (gated 62k/20k divider onto ADC1 channel 2).
- `knob_input` — quadrature wheel via PCNT (GPIO41/42) and the push on
  expander pin 3 (active low).
- `touch_read` — polls the SPD2010 touch controller (I2C1, SDA=GPIO39,
  SCL=GPIO38) and logs press coordinates; the interrupt line is expander
  pin 5, not a real GPIO, so the driver polls.
- `audio_tone` — 1 kHz sine on the ES8311 DAC + PA (I2S0), then a volume
  sweep; replays every 5 s.
- `mic_level` — RMS level meter from the ES7243/ES7243E mic ADC (variant
  auto-detected on I2C0), with a dBFS bar graph.

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

1. SD card: SPI2 device 2 (CS=GPIO46, detect on expander pin 4, rail on pin
   8), sharing the camera's bus — the camera driver tolerates an
   already-initialized bus.
2. On-hardware verification of everything above (builds pass; electrical
   behavior unverified).

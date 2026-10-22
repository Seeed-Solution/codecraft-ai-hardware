# Seeed reSpeaker Lite (XIAO ESP32-S3)

Status: **Phase 0** — facts verified, BSP extraction pending.

## Verified facts

- Module: XIAO ESP32-S3, 8 MB flash (DIO 80 MHz), 8 MB octal PSRAM (80 MHz).
  Source: muse `esp32/devices/sdkconfig.seeed-respeaker-lite`.
- Button: XIAO BOOT button on GPIO 0. Source: same.
- Audio: XMOS-processed microphone over 16 kHz I2S. The XMOS must run
  Seeed's **16 kHz I2S firmware v1.0.9**; the USB-audio and 48 kHz I2S
  images do not work. Flash the XMOS through the larger board's USB port
  per Seeed's instructions — flashing the ESP32 does not update the XMOS.
  Source: muse `esp32/devices/seeed-respeaker-lite.md`.
- Status: RGB status LED. Source: same.
- USB console: connect to the **small XIAO board**; it exposes the ESP32's
  USB Serial/JTAG console (`303a:1001`). Source: same.
- RF: attach the XIAO's external antenna; 2.4 GHz Wi-Fi only. Source: same.
- ESP-IDF: **v6.0.1**. Source: same.

## Vendor source

[respeaker/reSpeaker_Lite](https://github.com/respeaker/reSpeaker_Lite):
`doc/images/pinout.png`, I2C examples, `xmos_firmwares/`.

## Extraction plan

1. `bsp/include/bsp_pins.h` — pin map from the vendor pinout + muse sdkconfig.
2. `bsp/` — I2S audio in (16 kHz), RGB LED, button.
3. `examples/` — `audio_meter`, `button_led`, `wifi_scan`.

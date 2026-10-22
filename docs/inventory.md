# Phase 0 Inventory Matrix

The extraction baseline. Every fact below cites its source; `muse:` paths are
relative to the muse-gadget-sdk repository (Apache 2.0, Meta Platforms), and
`vendor:` entries point to the authoritative upstream listed in
[vendor-sources.md](vendor-sources.md).

## ESP32 boards

### SenseCAP Watcher

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3, 32 MB flash, octal PSRAM with XIP (`SPIRAM_XIP_FROM_PSRAM`) | `muse:esp32/devices/sdkconfig.muse-sensecap-watcher` | confirmed |
| Display + touch | LCD, touch, knob, IO expander pins live in the vendor BSP | `vendor:SenseCAP-Watcher-Firmware components/sensecap-watcher/include/sensecap-watcher.h` | pending |
| Audio | codec pins in vendor BSP | same as above | pending |
| Camera | Himax chip via SSCMA protocol | `muse:esp32/components/camera/` (`camera_sscma.c`, `sscma_proto.c`) | pending |
| Battery / SD | pins in vendor BSP | same as display row | pending |
| Power profile | USB-free sweep methodology + findings | `muse:esp32/docs/power/sensecap-watcher*.md`, `ppk2.md` | docs only, carry over |
| IDF version | per muse default (verify against vendor firmware) | `muse:esp32/README.md` | verify |

### SenseCAP Indicator (D1 / D1S / D1Pro / D1L)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3, 8 MB flash (QIO 80 MHz), 8 MB octal PSRAM | `muse:esp32/devices/sdkconfig.sensecap-indicator` | confirmed |
| Display | 4 inch 480x480 ST7701S RGB; 450 KB framebuffer in octal PSRAM | same | confirmed |
| Input | top button, active low, GPIO 38 | same | confirmed |
| Status indication | no status LED; display edge bars used instead | same | confirmed |
| Sensors | RP2040 co-processor handles sensors; unused in the muse profile | same | out of scope for v1 |
| USB | CH340 USB-UART bridge | same | confirmed |
| IDF version | verify against vendor repo | vendor sources | verify |

### reTerminal E1001 (7.5" mono e-ink)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3, 16 MB flash (DIO 80 MHz), octal PSRAM 80 MHz | `muse:esp32/devices/sdkconfig.reterminal-e1001` | confirmed |
| Display | UC8179-driven mono e-ink | same (`LED_BACKEND_RETERMINAL_UC8179`) | pending (waveform + refresh driver) |
| Input | green button, GPIO 3 | same | confirmed |
| IDF version | verify | vendor sources | verify |

### reTerminal E1002 (7.3" Spectra 6 color e-ink)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | ESP32-S3, 16 MB flash (DIO 80 MHz), octal PSRAM 80 MHz | `muse:esp32/devices/sdkconfig.reterminal-e1002` | confirmed |
| Display | Spectra 6 color e-ink | same (`LED_BACKEND_RETERMINAL_SPECTRA6`) | pending (color waveform handling) |
| Input | green button, GPIO 3 | same | confirmed |
| IDF version | verify | vendor sources | verify |

### Seeed reSpeaker Lite (XIAO ESP32-S3)

| Capability | Fact | Source | Extraction status |
|---|---|---|---|
| SoC / memory | XIAO ESP32-S3, 8 MB flash (DIO 80 MHz), 8 MB octal PSRAM | `muse:esp32/devices/sdkconfig.seeed-respeaker-lite` | confirmed |
| Audio in | XMOS-processed microphone over 16 kHz I2S; XMOS must run Seeed **16 kHz I2S firmware v1.0.9** (USB-audio and 48 kHz images incompatible) | `muse:esp32/devices/seeed-respeaker-lite.md` | confirmed |
| Input | XIAO BOOT button, GPIO 0 (push-to-talk in muse profile) | sdkconfig + doc above | confirmed |
| Status | RGB status LED | same | pending (driver extraction) |
| USB | small XIAO board USB exposes USB Serial/JTAG console (`303a:1001`) | doc above | confirmed |
| RF | attach XIAO external antenna; 2.4 GHz Wi-Fi only | doc above | confirmed |
| IDF version | ESP-IDF v6.0.1 | doc above | confirmed |

## Linux boards

| Board | Capability notes | Source | Status |
|---|---|---|---|
| reTerminal (CM4) | display, buttons, buzzer, RTC, light sensor via dtoverlay + sysfs | vendor wiki (see vendor-sources.md) | planned |
| reComputer series | Jetson-based; GPIO header maps | vendor wiki | planned |

## Cross-cutting dependencies to resolve during extraction

| Dependency | Needed by | Plan |
|---|---|---|
| SSCMA client (`camera_sscma.c`, `sscma_proto.c`) | SenseCAP Watcher camera | extract into `esp32/components/sscma_client/` with Meta copyright headers retained |
| LVGL version per board | all display boards | declare per board; do not assume one version |
| ESP-IDF version split | reSpeaker Lite requires v6.0.1; others TBD | declare per board in board README; CI matrix must cover both |

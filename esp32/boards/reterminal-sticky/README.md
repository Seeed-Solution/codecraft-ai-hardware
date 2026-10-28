# Seeed reTerminal Sticky

Status: **BSP landed** — `bsp/` (SSD1677 e-paper, keys, SHT4x) and three
examples build clean on ESP-IDF v6.0.2. On-device verification not yet
performed.

## Verified facts

- SoC/memory: ESP32-S3, 8 MB octal PSRAM (80 MHz). Flash: the vendor
  factory firmware and muse's board header say 32 MB; muse's sdkconfig
  configures 16 MB conservatively. This board keeps 16 MB (works on
  either chip) until the flash chip is confirmed on hardware.
  Sources: muse `esp32/components/muse/boards/board_reterminal_sticky.c`
  header, `esp32/devices/sdkconfig.reterminal-sticky`.
- Display: 3.97" SSD1677 e-paper, native 800x480, used portrait 480x800;
  1 bpp mono with full + fast refresh, plus a 4-gray image mode. SPI2:
  SCLK=13, MOSI=14, MISO=12, CS=15, DC=16, RST=17 (active low), BUSY=18
  (high = busy), panel power EN=47, 20 MHz.
  Source: muse `esp32/main/epd_ssd1677.c` (MIT), protocol from
  Seeed-reTerminal-E10xx-Firmware `factory_fw`.
- Keys: AI=GPIO 4, Up=GPIO 5, Down=GPIO 6, active low.
  Source: muse `board_reterminal_sticky.c`.
- SHT4x temperature/humidity @0x44 on I2C1 (SDA=1, SCL=0), shared with
  the BQ27220 fuel gauge @0x55.
  Sources: muse `esp32/main/reterminal_sht4x.c` (Sticky bus pins),
  `board_reterminal_sticky.c` (gauge).
- PDM microphone on I2S0: CLK=19, DATA=20, power EN=38 (active high).
  Source: muse `board_reterminal_sticky.c`.

## Extracted, driver not yet ported

- GT911 capacitive touch on I2C0 (SDA=3, SCL=2), address 0x5D (fallback
  0x14), power EN=42, RST=41, INT=21; the digitizer counts Y from the
  bottom, so Y is flipped in software.
- microSD over SDSPI on the panel's SPI2 bus: CS=8, power EN=10,
  card-detect=11 (low = inserted), MISO=12.
- Charger enable on GPIO 39: the muse board file drives it low at boot,
  but the active level is unconfirmed against factory_fw — treat as
  unverified until measured.

## Vendor source

The muse-gadget-sdk headers credit Seeed's
`Seeed-reTerminal-E10xx-Firmware` `factory_fw`
(`main/boards/reterminal_e1005/`, MIT, JoJang2u) as the original board
source, but that repo could not be located publicly at the time of
writing, so muse's field-verified files are the citable source here.
Board-level reference:
[reTerminal Sticky hardware overview](https://www.seeedstudio.com/sticky/docs/en/device-guide/hardware-overview/).

## BSP

| Header | Capability |
|---|---|
| `bsp_epd.h` | SSD1677 panel, 480x800 portrait: mono full/fast refresh + 4-gray mode |
| `bsp_button.h` | AI/Up/Down keys with debounce task |
| `bsp_sht4x.h` | SHT4x temperature/humidity on I2C1 |

Pin facts with sources: `bsp/include/bsp_pins.h`.

## Examples

- `epd_hello` — test pattern, full refresh, then an inverted fast refresh.
- `sht4x` — temperature/humidity log once a second.
- `button_hello` — log AI/Up/Down presses.

Each is a standalone IDF project sharing the board `sdkconfig.defaults`
via symlink.

Build (ESP-IDF v6.0.2):

```bash
source ~/esp/esp-idf-v6.0.2/export.sh
./tools/validate.sh --esp32 reterminal-sticky
```

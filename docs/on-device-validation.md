# On-device validation checklist

Everything in this repository is **build-verified** (ESP-IDF v6.0.2 /
Seeeduino SAMD 1.8.6 / Arduino AVR 1.8.8 / Raspberry Pi OS), but most drivers
have not yet run on real hardware.
This checklist is the gate for moving a board from "build-verified" to
"field-tested". The Wio Terminal section includes results recorded on
2026-10-28; other board sections remain build-verified unless noted. Run each
item, record the OS image / IDF version and the result, and update the board
README's Status line.

Legend: ✅ pass · ❌ fail (file an issue with logs) · ➖ skipped (why)

## SenseCAP Watcher (ESP32-S3)

| # | Check | Example / how | Expected |
|---|-------|---------------|----------|
| 1 | Camera | `camera_capture` | JPEG frame saved, plausible size |
| 2 | Display | `display_color_bars` | bars render, correct order/orientation |
| 3 | Touch | `touch_read` | coordinates track finger |
| 4 | Audio out | `audio_tone` | audible tone, no clipping |
| 5 | Mic | `mic_level` | level rises when speaking |
| 6 | Knob | `knob_input` | events follow rotation/press |
| 7 | Battery | `battery_status` | plausible mV, charging flag when USB in |
| 8 | microSD | `sdcard_test` | mount + write/read-back OK (FAT card) |

## SenseCAP Indicator (ESP32-S3)

| # | Check | Example / how | Expected |
|---|-------|---------------|----------|
| 1 | Display | `display_color_bars` | ST7701S RGB bars render |
| 2 | Button | `button_backlight` | press toggles backlight |
| 3 | Touch | `touch_read` | FT6336U or GT911 probed, coordinates sane |

## reTerminal E1001 / E1002 (ESP32-S3)

| # | Check | Example / how | Expected |
|---|-------|---------------|----------|
| 1 | EPD | `epd_hello` | pattern renders fully, no ghosting streaks; E1002 shows 6 colors |
| 2 | Keys/LED | `board_health` | all three keys register, LED toggles |
| 3 | SHT4x + battery | `sht4x_battery` | plausible temp/RH/mV |
| 4 | RTC | `board_health` | time sets and survives reboot |
| 5 | Charger | `board_health` | SY6974B part ID match, charge status on USB |
| 6 | Mic | `board_health` | PDM samples non-zero, level follows noise |
| 7 | Buzzer | `board_health` | audible beep |
| 8 | microSD | `sdcard_test` | mount + write/read-back OK; **also verify the panel still works after the SD bus init** (shared SPI2) |

## reTerminal (CM4, Linux)

| # | Check | Demo / how | Expected |
|---|-------|------------|----------|
| 1 | Buttons | `buttons_evdev.py` | F1/F2/F3/O events |
| 2 | LEDs + buzzer | `leds_buzzer_sysfs.py` | LEDs cycle, buzzer chirps |
| 3 | Sensors | `sensors_sysfs.py` | lux + accel readings plausible |
| 4 | RTC | `timedatectl` / `hwclock -r` | PCF8563 keeps time across power cycle (battery fitted) |
| 5 | Header GPIO | `header_gpio_blink.py` | BCM4 toggles (scope or LED) |
| 6 | Display + touch | vendor overlay | desktop/console on panel, touch tracks |

## Wio Terminal (SAMD51, Arduino)

Required toolchain: `Seeeduino:samd@1.8.6` plus the exact libraries pinned in
`arduino/boards/wio-terminal/libraries.txt`. Wi-Fi/BLE additionally require
compatible RTL8720DN eRPC firmware; it is not updated by an Arduino upload.

| # | Check | Example / how | Expected |
|---|-------|---------------|----------|
| 1 | Display + all inputs | `display_buttons` | landscape 320x240 UI; every A/B/C and five-way event appears on LCD and serial |
| 2 | Accelerometer | `accelerometer` | `0x18` probe succeeds; X/Y/Z change with tilt; still magnitude near 1 g |
| 3 | Light sensor | `light_sensor` | raw/mV changes when the sensor window is covered; direction recorded, not assumed |
| 4 | Analog microphone | `microphone_level` | peak-to-peak/RMS rise when speaking or whistling, return near baseline in silence |
| 5 | Passive buzzer | `buzzer_tones` | A gives 440 Hz/250 ms, B gives 4 kHz/120 ms, C stops the output |
| 6 | microSD | `sd_card_log` | FAT32 card ≤16 GB mounts; root lists; `/wio_terminal.log` receives a line |
| 7 | IR emitter | `ir_nec_sender` | a 38 kHz receiver decodes NEC address `0x00` and commands `0x45`/`0x10`/`0x02` |
| 8 | Grove I2C | `grove_i2c_scan` | a known 3.3 V J4 device's address appears; removing it yields no ACK |
| 9 | Wi-Fi | `wifi_scan` | RTL eRPC responds; nearby SSIDs/RSSI appear; record the firmware version; no association is attempted |
| 10 | BLE | `ble_scan` | RTL firmware verified; nearby BLE advertiser appears; no pairing/connection |

Run the Wi-Fi and BLE demos one at a time. The repository has no concurrent
wireless example.

Recorded results (2026-10-28):

- ✅ `display_buttons` — landscape LCD rendered the input map; A/B/C and
  five-way events appeared on LCD and serial; user confirmed.
- ✅ `accelerometer` — online at `0x18` (`WHO_AM_I` `0x33`); final mapping
  `displayX = -yg`, `displayY = xg`; dot rolled correctly with tilt; user
  confirmed.
- ✅ `light_sensor` — raw baseline approximately 14-20 counts under room
  light; user confirmed. Cover/uncover direction was not recorded.
- ✅ `microphone_level` — quiet peak-to-peak 162-666, RMS 37-129; speech or
  whistling reached peak-to-peak 3458 and RMS about 1578, then returned to
  baseline; user confirmed the LCD bar.
- ✅ `buzzer_tones` — 440 Hz and 4 kHz tones were audible, C stopped the
  output, and serial matched the A/B/C actions.
- ✅ `sd_card_log` — FAT32 card mounted, six root entries listed, and
  `/wio_terminal.log` grew from 98 to 147 bytes after a reset; LCD remained
  normal.
- ➖ `ir_nec_sender` — 940 nm emission observed on an iPhone camera at the
  back emitter; a 38 kHz receiver has not decoded the NEC frames.
- ➖ `grove_i2c_scan` — empty J4 reports zero devices after the `0x7F` prime
  fix; no external Grove device was available, so device detection remains
  pending.
- ✅ `wifi_scan` — the RTL8720DN eRPC firmware already installed on the test
  unit accepted the sketch; serial scans returned nearby 2.4 GHz and 5 GHz
  SSIDs with RSSI around -40 to -46 dBm, and the user confirmed the LCD.
  `rpc_system_version()` was not queried, so no RTL firmware version is
  recorded.
- ✅ `ble_scan` — coprocessor firmware 2.1.3 (`rpc_system_version()`); after
  an explicit `ble_start()` each five-second pass reported several hundred
  advertisements and listed nearby devices with names, addresses and RSSI; the
  user confirmed the LCD. The blocking `BLEScan::start` overload froze the UI
  on this unit, so the demo uses the non-blocking overload.


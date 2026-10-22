# On-device validation checklist

Everything in this repository is **build-verified** (ESP-IDF v6.0.2 /
Raspberry Pi OS), but most drivers have not yet run on real hardware.
This checklist is the gate for moving a board from "build-verified" to
"field-tested". Run each item, record the OS image / IDF version and the
result, and update the board README's Status line.

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
| 2 | Button | `button_events` (or board demo) | press events logged |
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

When reporting results, note: board revision, OS image and kernel
(`uname -a`) or IDF version (`idf.py --version`), and power source.

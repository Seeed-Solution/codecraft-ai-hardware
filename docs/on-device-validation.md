# On-device validation checklist

Everything in this repository is **build-verified** (ESP-IDF v6.0.2 /
Seeeduino SAMD 1.8.6 / Arduino AVR 1.8.8 / Raspberry Pi OS), but most drivers
have not yet run on real hardware.
This checklist is the gate for moving a board from "build-verified" to
"field-tested". The Wio Terminal section includes results recorded on
2026-10-28, and the Grove Beginner Kit section includes results recorded on
2026-10-29; other board sections remain build-verified unless noted. Run each
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

## Grove Beginner Kit for Arduino (ATmega328P, Arduino)

Required toolchain: `arduino:avr@1.8.8` with FQBN `arduino:avr:uno`, plus the
exact libraries pinned in `arduino/boards/grove-beginner-kit/libraries.txt`.
Target hardware: **post-October-2025 kit revision only**. Run `i2c_scan`
first and require DHT20 at 0x38 plus an SPA06-003 ACK at 0x77/0x76; the
SPA06 sketch additionally verifies device ID 0x11. The 2020 DHT11/BMP280
revision is out of scope and has no sketch in this directory.

| # | Check | Example / how | Expected |
|---|-------|---------------|----------|
| 1 | I2C inventory | `i2c_scan` | 0x19, 0x38, 0x3C/0x3D and 0x77/0x76 appear; record the exact addresses |
| 2 | LED | `led_blink` | steady 1 Hz on/off; serial prints both phases |
| 3 | Button | `button_led` | LED follows D6; one line per debounced transition |
| 4 | Potentiometer | `potentiometer` | raw range spans a useful part of 0-1023; LED switches near midpoint with hysteresis |
| 5 | Passive buzzer | `buzzer_tones` | four ascending tones (440/880/1760/2000 Hz) and a silent gap |
| 6 | Light sensor | `light_sensor` | raw value falls when covered and the LED turns on below the dark threshold |
| 7 | Sound sensor | `sound_sensor` | clap/speech raises the 50 ms peak; LED lights above raw 400 and releases after the hold |
| 8 | OLED | `oled_status` | four text rows and a once-per-second uptime counter; note orientation |
| 9 | DHT20 | `temperature_humidity_dht20` | `DHT20 on I2C 0x38`; plausible readings every second |
| 10 | SPA06-003 | `air_pressure_spa06` | detection at 0x77 or 0x76; pressure/temperature/altitude lines repeat |
| 11 | Accelerometer | `accelerometer` | `LIS3DHTR on I2C 0x19`; still magnitude near 1 g; axes change with tilt; LED indicates motion |

Record the USB bridge (CH340 on Lotus v1.0, CP2102N on v1.1), the power
source, and the `arduino:avr` core version. The board revision must still be
confirmed by the 0x38 I2C result. A compile is not an on-device pass.

When reporting results, note: board revision, power source, and the relevant
toolchain version (Arduino core, ESP-IDF `idf.py --version`, or OS/kernel
`uname -a`).

Recorded results (2026-10-29, Seeeduino Lotus v1.1, CP2102N, USB-powered,
`arduino:avr@1.8.8`):

- ✅ `i2c_scan` — 9600-baud serial reported 0x19 LIS3DHTR, 0x38 DHT20,
  0x3C OLED and 0x77 SPA06-003, repeated across scans. This confirms the
  post-October-2025 revision and the expected four-device I2C inventory.
- ✅ `led_blink` — serial repeatedly printed `LED=ON` and `LED=OFF` at the
  500 ms half-period after reset. LED emission itself was not captured by an
  instrument.
- ✅ `button_led` — boot state was `Button=RELEASED LED=OFF`; four physical
  press/release cycles produced matching `Button=PRESSED LED=ON` and
  `Button=RELEASED LED=OFF` lines.
- ✅ `potentiometer` — turning A0 through its full travel swept `raw` from 0
  to 1023 (0-100 %, 0-5000 mV). The LED switched on above the 532 threshold
  and off below 492, and held its state at `raw=503` and `526`, inside the
  hysteresis band.
- ✅ `buzzer_tones` — serial continuously cycled `440`, `880`, `1760` and
  `2000 Hz` followed by `silence 1500 ms`. Audible output was not captured by
  an instrument.
- ✅ `light_sensor` — ambient light read `raw=776-781` (`3.8 V`) with
  `LED=OFF`. Covering the photoresistor drove `raw` to 0-26 and printed
  `LED=ON`; removing the hand returned to `raw=779` and `LED=OFF`. The pass
  through `raw=21-26` stayed `LED=ON`, confirming the 200/260 hysteresis. The
  user confirmed the LED lit and went out. A first covering attempt produced
  no change because the wrong module was covered; a six-pin analog probe then
  showed A6 responding (777 to 0) while A7 tracked it down to about 80.
- ✅ `sound_sensor` — ambient 50 ms peaks stayed at 100-170 with `LED=OFF`.
  Two hand claps next to the module produced `peak=561 LED=ON` and later
  `peak=686 LED=ON`; the next sample held `LED=ON` at `peak=162`/`282` and
  then released to `LED=OFF`, confirming the 100 ms hold. The user confirmed
  the LED effect.
- ✅ `oled_status` — serial printed monotonically increasing `uptime` values
  from 2 s onward after the OLED initialized. The user confirmed that the
  panel displayed correctly on 2026-10-29.
- ✅ `temperature_humidity_dht20` — initialized at 0x38 and reported
  43.8–44.0 %RH at 26.2 C with about one sample per second.
- ✅ `air_pressure_spa06` — the library accepted SPA06-003 (device ID 0x11)
  at 0x77 and reported approximately 101170 Pa, 27.4 C and 12.8 m
  barometric altitude.
- ✅ `accelerometer` — initialized at 0x19 and rested at `X≈-0.01`,
  `Y≈0.14`, `Z≈0.98`, `|a|≈1.00 g`, `LED=REST`. Tilting and shaking drove
  the axes through X -0.82 to +1.16 g, Z 0.05 to 1.40 g and |a| 0.14 to
  1.60 g, and every sample during the motion printed `LED=ACTIVE`; the board
  returned to `LED=REST` once still. The rest offset shifted slightly after
  the shake (X -0.03 g, Y 0.12 g), consistent with the LIS3DHTR offset.

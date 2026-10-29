<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# oled_status — Agent Notes

## Purpose

Verify the kit's 128x64 OLED on the shared hardware I2C bus and show a live
uptime counter without allocating a full framebuffer.

## Hardware facts

- The kit OLED is a 0.96-inch 128x64 SSD1306/SSD1315-compatible display on
  hardware I2C, SDA A4/pin 18 and SCL A5/pin 19. Source: kit wiki
  default-pin table and Lesson 7; Seeeduino Lotus v1.1 schematic.
- The vendor pin table gives 0x78 as the 8-bit write address. The sketch uses
  the 7-bit form 0x3C and accepts 0x3D as an alternate. Source: kit wiki and
  the Grove OLED wiki.
- U8x8 text mode is used instead of a U8g2 full framebuffer because the
  ATmega328P has only 2 KB SRAM. Source: Seeeduino Lotus wiki memory figures
  and the U8g2/U8x8 API.
- The sketch calls `setFlipMode(1)`, which rotates the panel output 180
  degrees. This is an application orientation choice, not an I2C property.
  Source: the U8g2 `U8X8::setFlipMode()` API.

## Build

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/oled_status
```

## Expected behavior

- Setup prints `oled_status: OLED on I2C 0x3C/0x3D` once at 9600 baud.
- The panel shows four 16-column text rows: `GROVE BEGINNER`,
  `KIT / OLED TEST`, `128x64 I2C OLED`, and `uptime 0 s`. The last row
  advances about once per second.
- The serial monitor prints `uptime 1 s`, `uptime 2 s`, ... at the same time.
- If 0x3C does not ACK but 0x3D does, the sketch calls
  `setI2CAddress(0x7A)` (the 8-bit form of 0x3D) before initialization.
- If neither address ACKs, serial prints
  `OLED init failed; expected 0x3C or 0x3D`; the panel can stay blank while
  uptime lines continue on serial.

## Physical test

1. Run `i2c_scan` and record whether 0x3C or 0x3D is present.
2. Open the 9600-baud monitor and confirm the four text rows appear.
3. Watch for at least three counter updates and confirm the panel and serial
   agree.
4. If the text is upside down, the panel is mounted opposite to the
   `setFlipMode(1)` assumption; set it to 0 and re-check. Do not change the
   I2C address to work around orientation.

## Limitations

- U8x8 text mode supports 16 columns by 8 rows of 8x8 cells; this sketch does
  not test graphics, scrolling, or custom fonts.
- An I2C ACK proves the controller responds, not that the panel is healthy;
  a blank but ACK-ing display can be a contrast, orientation, or panel fault.
- The sketch supports SSD1306/SSD1315-compatible controllers only.
- A successful compile does not prove the OLED or its wiring.

/*
 * oled_status - draw a static status screen and uptime on the kit OLED.
 *
 * Source: Grove Beginner Kit wiki default-pin table and Lesson 7, OLED on
 * hardware I2C at 0x78 (8-bit) / 0x3C (7-bit), 128x64. The UI uses U8x8 text
 * mode to keep SRAM usage small on the ATmega328P.
 */

#include <U8x8lib.h>
#include <Wire.h>

U8X8_SSD1306_128X64_NONAME_HW_I2C display(U8X8_PIN_NONE);

bool i2cPresent(uint8_t address) {
  Wire.beginTransmission(address);
  return Wire.endTransmission() == 0;
}

void setup() {
  Serial.begin(9600);
  Wire.begin();

  if (!i2cPresent(0x3C) && i2cPresent(0x3D)) {
    display.setI2CAddress(0x7A);
  }

  if (!display.begin()) {
    Serial.println(F("OLED init failed; expected 0x3C or 0x3D"));
  }
  display.setPowerSave(0);
  display.setFlipMode(1);
  display.setFont(u8x8_font_chroma48medium8_r);
  display.clear();

  display.drawString(0, 0, "GROVE BEGINNER");
  display.drawString(0, 2, "KIT / OLED TEST");
  display.drawString(0, 4, "128x64 I2C OLED");
  display.drawString(0, 6, "uptime 0 s");

  Serial.println(F("oled_status: OLED on I2C 0x3C/0x3D"));
}

void loop() {
  static uint32_t lastUpdateMs = 0;
  static uint32_t seconds = 0;

  if ((uint32_t)(millis() - lastUpdateMs) >= 1000) {
    lastUpdateMs = millis();
    ++seconds;

    char line[17];
    snprintf(line, sizeof(line), "uptime %lu s", seconds);
    display.drawString(0, 6, line);
    Serial.println(line);
  }
}

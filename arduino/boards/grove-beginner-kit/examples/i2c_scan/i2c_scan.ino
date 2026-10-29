/*
 * i2c_scan - enumerate devices on the kit's hardware I2C bus.
 *
 * Source: Seeeduino Lotus schematic v1.1 (ATmega328P I2C on A4/SDA, A5/SCL)
 * and the Grove Beginner Kit wiki default-pin table. This board directory
 * targets only the post-October-2025 kit revision.
 * The scan is read-only and reports 7-bit addresses, not the 8-bit form
 * sometimes printed by vendor documentation.
 */

#include <Wire.h>

void printDevice(uint8_t address) {
  Serial.print(F("  found 0x"));
  if (address < 0x10) {
    Serial.print('0');
  }
  Serial.print(address, HEX);

  switch (address) {
    case 0x19:
      Serial.print(F("  LIS3DHTR accelerometer"));
      break;
    case 0x38:
      Serial.print(F("  DHT20 temperature/humidity"));
      break;
    case 0x3C:
    case 0x3D:
      Serial.print(F("  SSD1306/SSD1315 OLED"));
      break;
    case 0x76:
    case 0x77:
      Serial.print(F("  SPA06-003 pressure sensor"));
      break;
    default:
      break;
  }
  Serial.println();
}

void scanBus() {
  uint8_t count = 0;

  Serial.println(F("I2C scan: SDA=A4/18, SCL=A5/19, 7-bit addresses"));
  for (uint8_t address = 1; address < 127; ++address) {
    Wire.beginTransmission(address);
    if (Wire.endTransmission() == 0) {
      printDevice(address);
      ++count;
    }
  }

  Serial.print(F("I2C scan complete: "));
  Serial.print(count);
  Serial.println(F(" device(s)"));
}

void setup() {
  Serial.begin(9600);
  Wire.begin();
  delay(500);
  scanBus();
}

void loop() {
  delay(3000);
  scanBus();
}

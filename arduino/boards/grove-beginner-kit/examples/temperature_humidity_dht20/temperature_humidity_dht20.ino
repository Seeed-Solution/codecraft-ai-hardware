/*
 * temperature_humidity_dht20 - read the post-2025 kit DHT20 over I2C.
 *
 * Source: Grove Beginner Kit wiki current DHT20 example (I2C, Wire.begin())
 * and the Grove Temperature And Humidity Sensor library DHT20 API. This
 * board directory targets only the post-October-2025 revision. The published
 * kit schematic predates DHT20, so probe 0x38 before initialization.
 */

#include "DHT.h"
#include <Wire.h>

const uint8_t kDht20Address = 0x38;
DHT dht(0, DHT20);

bool i2cPresent(uint8_t address) {
  Wire.beginTransmission(address);
  return Wire.endTransmission() == 0;
}

void setup() {
  Serial.begin(9600);
  Wire.begin();

  if (!i2cPresent(kDht20Address)) {
    Serial.println(F("DHT20 not found at I2C 0x38"));
    while (true) {
      delay(1000);
    }
  }

  dht.begin();
  Serial.println(F("DHT20 on I2C 0x38"));
}

void loop() {
  float values[2] = {0};
  if (dht.readTempAndHumidity(values) == 0) {
    Serial.print(F("Humidity: "));
    Serial.print(values[0], 1);
    Serial.print(F(" %  Temperature: "));
    Serial.print(values[1], 1);
    Serial.println(F(" C"));
  } else {
    Serial.println(F("DHT20 read failed; check I2C 0x38 and sensor power"));
  }

  delay(1000);
}

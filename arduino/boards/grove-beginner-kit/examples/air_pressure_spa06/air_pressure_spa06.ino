/*
 * air_pressure_spa06 - read the post-2025 kit SPA06-003 pressure sensor.
 *
 * Source: Grove SPA06-003 wiki (I2C 0x77 default, 0x76 alternate) and the
 * Seeed Arduino SPA06 library, which verifies device ID 0x11. This example
 * probes both addresses before configuring continuous measurements.
 */

#include <Wire.h>
#include "SPL07-003.h"

SPL07_003 spl;
bool sensorReady = false;

bool tryAddress(uint8_t address) {
  if (spl.begin(address, &Wire)) {
    Serial.print(F("SPA06-003 on I2C 0x"));
    if (address < 0x10) {
      Serial.print('0');
    }
    Serial.println(address, HEX);
    return true;
  }
  return false;
}

void setup() {
  Serial.begin(9600);
  Wire.begin();

  sensorReady = tryAddress(SPL07_ADDR_DEF) || tryAddress(SPL07_ADDR_ALT);
  if (!sensorReady) {
    Serial.println(F("SPA06-003 not found at 0x77 or 0x76"));
    return;
  }

  spl.setPressureConfig(SPL07_4HZ, SPL07_32SAMPLES);
  spl.setTemperatureConfig(SPL07_4HZ, SPL07_1SAMPLE);
  spl.setMode(SPL07_CONT_PRES_TEMP);
}

void loop() {
  if (!sensorReady) {
    delay(1000);
    return;
  }

  if (spl.pressureAvailable() || spl.temperatureAvailable()) {
    const double pressurePa = spl.readPressure();
    const double temperatureC = spl.readTemperature();
    const double altitudeM = spl.calcAltitude();

    Serial.print(F("Pres: "));
    Serial.print(pressurePa, 1);
    Serial.print(F(" Pa  Temp: "));
    Serial.print(temperatureC, 1);
    Serial.print(F(" C  Altitude: "));
    Serial.print(altitudeM, 1);
    Serial.println(F(" m"));
  }
}

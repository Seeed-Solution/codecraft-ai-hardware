/*
 * accelerometer - read the kit LIS3DHTR and indicate tilt/shake.
 *
 * Source: Grove Beginner Kit wiki default-pin table and Lesson 10,
 * LIS3DHTR on I2C at 0x19. The kit LED on D4 is active high. This is a
 * simple, uncalibrated heuristic: gravity magnitude near 1 g at rest.
 */

#include <Wire.h>
#include "LIS3DHTR.h"
#include <math.h>

const uint8_t kLedPin = 4;
const uint8_t kLisAddress = 0x19;

LIS3DHTR<TwoWire> lis;
bool sensorReady = false;
uint32_t lastPrintMs = 0;

void setup() {
  pinMode(kLedPin, OUTPUT);
  digitalWrite(kLedPin, LOW);
  Serial.begin(9600);
  Wire.begin();

  lis.begin(Wire, kLisAddress);
  delay(100);
  sensorReady = static_cast<bool>(lis);

  if (sensorReady) {
    lis.setFullScaleRange(LIS3DHTR_RANGE_2G);
    lis.setOutputDataRate(LIS3DHTR_DATARATE_50HZ);
    lis.setHighSolution(true);
    Serial.println(F("LIS3DHTR on I2C 0x19"));
  } else {
    Serial.println(F("LIS3DHTR not found at I2C 0x19"));
  }
}

void loop() {
  if (!sensorReady) {
    delay(1000);
    return;
  }

  const float x = lis.getAccelerationX();
  const float y = lis.getAccelerationY();
  const float z = lis.getAccelerationZ();
  const float magnitude = sqrt(x * x + y * y + z * z);
  const bool active = fabs(magnitude - 1.0f) > 0.20f || fabs(x) > 0.60f ||
                      fabs(y) > 0.60f;
  digitalWrite(kLedPin, active ? HIGH : LOW);

  if ((uint32_t)(millis() - lastPrintMs) >= 200) {
    lastPrintMs = millis();
    Serial.print(F("X="));
    Serial.print(x, 2);
    Serial.print(F(" g  Y="));
    Serial.print(y, 2);
    Serial.print(F(" g  Z="));
    Serial.print(z, 2);
    Serial.print(F(" g  |a|="));
    Serial.print(magnitude, 2);
    Serial.print(F(" g  LED="));
    Serial.println(active ? F("ACTIVE") : F("REST"));
  }
}

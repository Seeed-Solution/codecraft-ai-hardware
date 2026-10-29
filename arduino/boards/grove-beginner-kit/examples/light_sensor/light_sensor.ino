/*
 * light_sensor - use the kit's light sensor to switch the kit LED.
 *
 * Source: Grove Beginner Kit wiki default-pin table and Lesson 5, light on
 * A6 and LED on D4. The vendor example uses a raw threshold of 200; this
 * sketch adds hysteresis so the LED does not chatter at the threshold.
 */

const uint8_t kLightPin = A6;
const uint8_t kLedPin = 4;
const uint16_t kTurnOnBelow = 200;
const uint16_t kTurnOffAbove = 260;

bool ledOn = false;
uint32_t lastPrintMs = 0;

uint16_t readAverage(uint8_t pin) {
  uint32_t sum = 0;
  for (uint8_t i = 0; i < 8; ++i) {
    sum += analogRead(pin);
    delay(1);
  }
  return static_cast<uint16_t>(sum / 8);
}

void setup() {
  pinMode(kLightPin, INPUT);
  pinMode(kLedPin, OUTPUT);
  digitalWrite(kLedPin, LOW);
  Serial.begin(9600);
  Serial.println(F("light_sensor: Grove Light on A6 -> LED D4"));
}

void loop() {
  const uint16_t raw = readAverage(kLightPin);
  if (!ledOn && raw < kTurnOnBelow) {
    ledOn = true;
  } else if (ledOn && raw > kTurnOffAbove) {
    ledOn = false;
  }
  digitalWrite(kLedPin, ledOn ? HIGH : LOW);

  if ((uint32_t)(millis() - lastPrintMs) >= 250) {
    lastPrintMs = millis();
    const uint16_t millivolts =
        static_cast<uint32_t>(raw) * 5000UL / 1023UL;
    Serial.print(F("A6 raw="));
    Serial.print(raw);
    Serial.print(F(" mV="));
    Serial.print(millivolts);
    Serial.print(F(" LED="));
    Serial.println(ledOn ? F("ON") : F("OFF"));
  }
}

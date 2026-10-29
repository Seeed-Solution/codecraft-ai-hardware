/*
 * potentiometer - read the kit's rotary potentiometer on A0.
 *
 * Source: Grove Beginner Kit wiki default-pin table, potentiometer on A0.
 * The kit LED is on D4, which is not a PWM pin on the ATmega328P. This demo
 * therefore uses an on/off threshold; dimming requires moving the LED to D3.
 */

const uint8_t kPotPin = A0;
const uint8_t kLedPin = 4;
const uint16_t kOnThreshold = 532;
const uint16_t kOffThreshold = 492;

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
  pinMode(kPotPin, INPUT);
  pinMode(kLedPin, OUTPUT);
  digitalWrite(kLedPin, LOW);
  Serial.begin(9600);
  Serial.println(F("potentiometer: Grove Rotary Potentiometer on A0"));
}

void loop() {
  const uint16_t raw = readAverage(kPotPin);

  if (!ledOn && raw > kOnThreshold) {
    ledOn = true;
  } else if (ledOn && raw < kOffThreshold) {
    ledOn = false;
  }
  digitalWrite(kLedPin, ledOn ? HIGH : LOW);

  if ((uint32_t)(millis() - lastPrintMs) >= 200) {
    lastPrintMs = millis();
    const uint16_t millivolts =
        static_cast<uint32_t>(raw) * 5000UL / 1023UL;
    const uint8_t percent = map(raw, 0, 1023, 0, 100);

    Serial.print(F("A0 raw="));
    Serial.print(raw);
    Serial.print(F(" mV="));
    Serial.print(millivolts);
    Serial.print(F(" pct="));
    Serial.print(percent);
    Serial.print(F(" LED="));
    Serial.println(ledOn ? F("ON") : F("OFF"));
  }
}

/*
 * led_blink - blink the kit's Grove LED module.
 *
 * Source: Grove Beginner Kit wiki default-pin table, LED on D4.
 * The module is active high and has an adjustable brightness trimpot.
 */

const uint8_t kLedPin = 4;
const uint16_t kHalfPeriodMs = 500;

void setup() {
  pinMode(kLedPin, OUTPUT);
  digitalWrite(kLedPin, LOW);
  Serial.begin(9600);
  Serial.println(F("led_blink: Grove LED on D4"));
}

void loop() {
  digitalWrite(kLedPin, HIGH);
  Serial.println(F("LED=ON"));
  delay(kHalfPeriodMs);

  digitalWrite(kLedPin, LOW);
  Serial.println(F("LED=OFF"));
  delay(kHalfPeriodMs);
}

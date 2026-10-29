/*
 * button_led - mirror the kit button state onto the kit LED.
 *
 * Source: Grove Beginner Kit wiki default-pin table and Lesson 2, button on
 * D6 with pressed=HIGH and LED on D4. Do not enable an internal pull-up here:
 * the module provides its own pull-down, so INPUT_PULLUP would read pressed
 * while the button is idle.
 */

const uint8_t kButtonPin = 6;
const uint8_t kLedPin = 4;
const uint16_t kDebounceMs = 20;

bool lastRaw = LOW;
bool stableState = LOW;
uint32_t lastRawChangeMs = 0;

void setup() {
  pinMode(kButtonPin, INPUT);
  pinMode(kLedPin, OUTPUT);

  lastRaw = digitalRead(kButtonPin) == HIGH;
  stableState = lastRaw;
  digitalWrite(kLedPin, stableState ? HIGH : LOW);
  lastRawChangeMs = millis();

  Serial.begin(9600);
  Serial.println(F("button_led: Grove Button D6 -> Grove LED D4"));
  Serial.println(stableState ? F("Button=PRESSED LED=ON")
                             : F("Button=RELEASED LED=OFF"));
}

void loop() {
  const bool rawState = digitalRead(kButtonPin) == HIGH;
  if (rawState != lastRaw) {
    lastRaw = rawState;
    lastRawChangeMs = millis();
  }

  if (stableState != lastRaw &&
      (uint32_t)(millis() - lastRawChangeMs) >= kDebounceMs) {
    stableState = lastRaw;
    digitalWrite(kLedPin, stableState ? HIGH : LOW);
    Serial.println(stableState ? F("Button=PRESSED LED=ON")
                               : F("Button=RELEASED LED=OFF"));
  }
}

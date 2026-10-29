/*
 * sound_sensor - show the kit's analog sound envelope on the serial plotter
 * and light the kit LED above a short-window peak threshold.
 *
 * Source: Grove Beginner Kit wiki default-pin table and Lesson 6, sound on
 * A2 and LED on D4; the vendor threshold is raw > 400. This is an envelope
 * output, not calibrated dB SPL and not recorded audio.
 */

const uint8_t kSoundPin = A2;
const uint8_t kLedPin = 4;
const uint16_t kWindowMs = 50;
const uint16_t kThreshold = 400;
const uint16_t kHoldMs = 100;

uint32_t holdUntilMs = 0;

void setup() {
  pinMode(kSoundPin, INPUT);
  pinMode(kLedPin, OUTPUT);
  digitalWrite(kLedPin, LOW);
  Serial.begin(9600);
  Serial.println(F("sound_sensor: Grove Sound on A2"));
}

void loop() {
  uint16_t peak = 0;
  const uint32_t startMs = millis();
  while ((uint32_t)(millis() - startMs) < kWindowMs) {
    const uint16_t sample = analogRead(kSoundPin);
    if (sample > peak) {
      peak = sample;
    }
    delay(2);
  }

  if (peak > kThreshold) {
    holdUntilMs = millis() + kHoldMs;
  }
  const bool ledOn = static_cast<int32_t>(millis() - holdUntilMs) < 0;
  digitalWrite(kLedPin, ledOn ? HIGH : LOW);

  Serial.print(F("A2 peak="));
  Serial.print(peak);
  Serial.print(F(" LED="));
  Serial.println(ledOn ? F("ON") : F("OFF"));
}

/*
 * buzzer_tones - drive the kit's passive piezo buzzer.
 *
 * Source: Grove Beginner Kit wiki default-pin table and Lesson 4, buzzer on
 * D5. The module is passive, so it needs a square wave; a static HIGH level
 * will not produce a sustained tone.
 */

const uint8_t kBuzzerPin = 5;
const uint16_t kFrequencies[] = {440, 880, 1760, 2000};
const uint16_t kDurationsMs[] = {250, 250, 250, 400};

void setup() {
  pinMode(kBuzzerPin, OUTPUT);
  noTone(kBuzzerPin);
  Serial.begin(9600);
  Serial.println(F("buzzer_tones: Grove Buzzer on D5"));
}

void loop() {
  for (uint8_t i = 0; i < sizeof(kFrequencies) / sizeof(kFrequencies[0]);
       ++i) {
    Serial.print(F("tone "));
    Serial.print(kFrequencies[i]);
    Serial.println(F(" Hz"));
    tone(kBuzzerPin, kFrequencies[i], kDurationsMs[i]);
    delay(kDurationsMs[i]);
    noTone(kBuzzerPin);
    delay(80);
  }

  Serial.println(F("silence 1500 ms"));
  delay(1500);
}

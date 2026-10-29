/*
 * Wio Terminal onboard passive buzzer demo.
 *
 * Hardware facts:
 * - WIO_BUZZER is GPIO 12 (TCC0 channel 4), driving Q4 and the onboard
 *   passive buzzer B4013AM422-042.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 4 ("Buzzer & MIC") and
 *   Seeed-Studio/ArduinoCore-samd 1.8.6 variants/wio_terminal/variant.h.
 * - The buzzer needs an AC/PWM waveform; it is not driven by a DC level.
 *   Source: Seeed Wio Terminal Buzzer guide:
 *   https://wiki.seeedstudio.com/Wio-Terminal-Buzzer/
 * - This sketch uses the SAMD core tone() timer API. Each button starts a
 *   finite tone, so loop() remains responsive and no tone runs forever.
 */

#include <TFT_eSPI.h>

TFT_eSPI tft;

struct Button {
  uint8_t pin;
  const char *label;
  bool lastRaw;
  bool stable;
  uint32_t changedMs;
};

Button buttons[] = {
    {WIO_KEY_A, "A", false, false, 0},
    {WIO_KEY_B, "B", false, false, 0},
    {WIO_KEY_C, "C", false, false, 0},
};

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("PASSIVE BUZZER", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("WIO_BUZZER = GPIO 12", 10, 42, 2);
  tft.drawString("A: 440 Hz   B: 4 kHz   C: stop", 10, 70, 2);
  tft.drawRect(10, 120, 300, 50, TFT_DARKGREY);
  tft.drawString("last action", 10, 184, 2);
}

void drawAction(const char *text, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(280);
  tft.drawString(text, 18, 132, 4);
  tft.setTextPadding(0);
}

bool pressed(Button &button) {
  const bool raw = digitalRead(button.pin) == LOW;
  if (raw != button.lastRaw) {
    button.lastRaw = raw;
    button.changedMs = millis();
  }
  if (millis() - button.changedMs < 20 || raw == button.stable) {
    return false;
  }
  button.stable = raw;
  return raw;
}

void setup() {
  Serial.begin(115200);
  for (Button &button : buttons) {
    pinMode(button.pin, INPUT_PULLUP);
  }
  pinMode(WIO_BUZZER, OUTPUT);

  tft.begin();
  tft.setRotation(3);
  drawScreen();
  drawAction("ready", TFT_GREEN);
}

void loop() {
  if (pressed(buttons[0])) {
    tone(WIO_BUZZER, 440, 250);
    drawAction("A: 440 Hz", TFT_CYAN);
    Serial.println("tone 440 Hz for 250 ms");
  }
  if (pressed(buttons[1])) {
    tone(WIO_BUZZER, 4000, 120);
    drawAction("B: 4 kHz", TFT_YELLOW);
    Serial.println("tone 4000 Hz for 120 ms");
  }
  if (pressed(buttons[2])) {
    noTone(WIO_BUZZER);
    digitalWrite(WIO_BUZZER, LOW);
    drawAction("C: stopped", TFT_ORANGE);
    Serial.println("buzzer stopped");
  }
  delay(5);
}

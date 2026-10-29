/*
 * Wio Terminal display + button capability demo.
 *
 * Hardware facts:
 * - 2.4 inch 320x240 ILI9341 LCD on SPI3, with LCD_SS_PIN / LCD_DC /
 *   LCD_RESET / LCD_BACKLIGHT from the Seeed SAMD core 1.8.6 variant:
 *   variants/wio_terminal/variant.h and Seeed_Arduino_LCD/User_Setup.h.
 * - WIO_KEY_A/B/C are GPIO 28/29/30, active low.
 * - WIO_5S_UP/LEFT/RIGHT/DOWN/PRESS are GPIO 31/32/33/34/35, active low.
 * - The top buttons K1-K3 are on schematic page 5 ("Header Interface and
 *   Button"); the five-way switch SW1 is on page 4 ("Buzzer & MIC"). All
 *   switches connect to GND with pull-ups.
 *   Source: Wio-Terminal-SCH-v1.2.pdf pages 4-5 and variant.h in
 *   Seeed-Studio/ArduinoCore-samd 1.8.6.
 *
 * The sketch redraws only changed labels, so it also serves as a low-flicker
 * display pattern for the other examples.
 */

#include <TFT_eSPI.h>

TFT_eSPI tft;

struct InputState {
  uint8_t pin;
  const char *label;
  bool stablePressed;
  bool lastRawPressed;
  uint32_t lastChangeMs;
};

InputState keys[] = {
    {WIO_KEY_A, "A", false, false, 0},
    {WIO_KEY_B, "B", false, false, 0},
    {WIO_KEY_C, "C", false, false, 0},
};

InputState directions[] = {
    {WIO_5S_UP, "UP", false, false, 0},
    {WIO_5S_LEFT, "LEFT", false, false, 0},
    {WIO_5S_PRESS, "PRESS", false, false, 0},
    {WIO_5S_RIGHT, "RIGHT", false, false, 0},
    {WIO_5S_DOWN, "DOWN", false, false, 0},
};

void drawKey(int x, int y, int w, int h, const char *label, bool pressed) {
  const uint16_t background = pressed ? TFT_GREEN : TFT_DARKGREY;
  tft.fillRoundRect(x, y, w, h, 5, background);
  tft.setTextColor(pressed ? TFT_BLACK : TFT_WHITE, background);
  tft.drawCentreString(label, x + w / 2, y + (h / 2) - 8, 2);
}

void updateInput(InputState &input) {
  const bool rawPressed = digitalRead(input.pin) == LOW;
  if (rawPressed != input.lastRawPressed) {
    input.lastRawPressed = rawPressed;
    input.lastChangeMs = millis();
  }
  if ((millis() - input.lastChangeMs) < 20 || rawPressed == input.stablePressed) {
    return;
  }
  input.stablePressed = rawPressed;
  Serial.print(input.label);
  Serial.println(rawPressed ? " pressed" : " released");
}

void setup() {
  Serial.begin(115200);

  for (InputState &input : keys) {
    pinMode(input.pin, INPUT_PULLUP);
  }
  for (InputState &input : directions) {
    pinMode(input.pin, INPUT_PULLUP);
  }

  tft.begin();
  tft.setRotation(3);
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawCentreString("WIO TERMINAL INPUT", 160, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawCentreString("Hold a key: serial and LCD mirror its state", 160, 34, 2);

  for (int i = 0; i < 3; ++i) {
    drawKey(28 + i * 92, 58, 76, 34, keys[i].label, keys[i].stablePressed);
  }

  drawKey(140, 100, 48, 30, "UP", directions[0].stablePressed);
  drawKey(84, 134, 52, 30, "LEFT", directions[1].stablePressed);
  drawKey(140, 134, 48, 30, "PRESS", directions[2].stablePressed);
  drawKey(192, 134, 52, 30, "RIGHT", directions[3].stablePressed);
  drawKey(140, 168, 48, 30, "DOWN", directions[4].stablePressed);
}

void loop() {
  bool changedKeys[3] = {};
  bool changedDirections[5] = {};

  for (int i = 0; i < 3; ++i) {
    const bool before = keys[i].stablePressed;
    updateInput(keys[i]);
    changedKeys[i] = before != keys[i].stablePressed;
  }
  for (int i = 0; i < 5; ++i) {
    const bool before = directions[i].stablePressed;
    updateInput(directions[i]);
    changedDirections[i] = before != directions[i].stablePressed;
  }

  for (int i = 0; i < 3; ++i) {
    if (changedKeys[i]) {
      drawKey(28 + i * 92, 58, 76, 34, keys[i].label, keys[i].stablePressed);
    }
  }
  if (changedDirections[0]) {
    drawKey(140, 100, 48, 30, "UP", directions[0].stablePressed);
  }
  if (changedDirections[1]) {
    drawKey(84, 134, 52, 30, "LEFT", directions[1].stablePressed);
  }
  if (changedDirections[2]) {
    drawKey(140, 134, 48, 30, "PRESS", directions[2].stablePressed);
  }
  if (changedDirections[3]) {
    drawKey(192, 134, 52, 30, "RIGHT", directions[3].stablePressed);
  }
  if (changedDirections[4]) {
    drawKey(140, 168, 48, 30, "DOWN", directions[4].stablePressed);
  }

  delay(5);
}

/*
 * Wio Terminal 940 nm infrared emitter demo (NEC frame sender).
 *
 * Hardware facts:
 * - WIO_IR is GPIO 14. It drives Q13, which sinks current through the
 *   onboard 940 nm IR LED D10; IR_CTL is active high.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 5 ("Header Interface and
 *   Button", IR_CTL / Q13 / D10) and Seeed-Studio/ArduinoCore-samd 1.8.6
 *   variants/wio_terminal/variant.h (WIO_IR). The 940 nm wavelength is
 *   listed on https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/.
 * - The emitter is on the back of the enclosure, to the left of the
 *   microSD slot. Source:
 *   https://wiki.seeedstudio.com/Wio-Terminal-Infrared-Emitter/.
 * - The sketch bit-bangs a 38 kHz NEC frame. It intentionally uses no
 *   IRremote dependency so the example is self-contained.
 *
 * Human verification: point the emitter at a known 38 kHz IR receiver or
 * use a phone camera that does not filter near-IR. A phone screen or camera
 * may show only a faint pulse; that alone is not proof of a valid NEC frame.
 */

#include <TFT_eSPI.h>

static constexpr uint32_t kCarrierHalfPeriodUs = 13;
static constexpr uint16_t kNecHeaderMarkUs = 9000;
static constexpr uint16_t kNecHeaderSpaceUs = 4500;
static constexpr uint16_t kNecBitMarkUs = 560;
static constexpr uint16_t kNecZeroSpaceUs = 560;
static constexpr uint16_t kNecOneSpaceUs = 1690;

TFT_eSPI tft;

struct Button {
  uint8_t pin;
  const char *label;
  uint8_t command;
  bool lastRaw;
  bool stable;
  uint32_t changedMs;
};

Button buttons[] = {
    {WIO_KEY_A, "A", 0x45, false, false, 0},
    {WIO_KEY_B, "B", 0x10, false, false, 0},
    {WIO_KEY_C, "C", 0x02, false, false, 0},
};

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("IR NEC SENDER", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("WIO_IR = GPIO 14 / 940 nm", 10, 42, 2);
  tft.drawString("NEC: 38 kHz, addr 0x00", 10, 64, 2);
  tft.drawString("A: 0x45   B: 0x10   C: 0x02", 10, 94, 2);
  tft.drawRect(10, 126, 300, 44, TFT_DARKGREY);
  tft.drawString("last frame", 10, 184, 2);
}

void drawLastFrame(const char *text, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(280);
  tft.drawString(text, 18, 140, 4);
  tft.setTextPadding(0);
}

void carrierMark(uint32_t durationUs) {
  const uint32_t start = micros();
  while (micros() - start < durationUs) {
    digitalWrite(WIO_IR, HIGH);
    delayMicroseconds(kCarrierHalfPeriodUs);
    digitalWrite(WIO_IR, LOW);
    delayMicroseconds(kCarrierHalfPeriodUs);
  }
}

void sendNecByte(uint8_t value) {
  for (uint8_t bit = 0; bit < 8; ++bit) {
    carrierMark(kNecBitMarkUs);
    delayMicroseconds((value & (1U << bit)) ? kNecOneSpaceUs : kNecZeroSpaceUs);
  }
}

void sendNecFrame(uint8_t address, uint8_t command) {
  carrierMark(kNecHeaderMarkUs);
  delayMicroseconds(kNecHeaderSpaceUs);
  sendNecByte(address);
  sendNecByte(static_cast<uint8_t>(~address));
  sendNecByte(command);
  sendNecByte(static_cast<uint8_t>(~command));
  carrierMark(kNecBitMarkUs);
  digitalWrite(WIO_IR, LOW);
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
  pinMode(WIO_IR, OUTPUT);
  digitalWrite(WIO_IR, LOW);

  tft.begin();
  tft.setRotation(3);
  drawScreen();
  drawLastFrame("ready", TFT_GREEN);
}

void loop() {
  for (Button &button : buttons) {
    if (!pressed(button)) {
      continue;
    }
    char label[24];
    snprintf(label, sizeof(label), "%s: cmd 0x%02X", button.label, button.command);
    drawLastFrame(label, TFT_CYAN);
    Serial.print("sending NEC addr=0x00 cmd=");
    Serial.println(button.command, HEX);
    sendNecFrame(0x00, button.command);
  }
  delay(5);
}

/*
 * Wio Terminal Grove I2C bus scanner.
 *
 * Hardware facts:
 * - The left Grove connector J4 carries the external I2C bus. The SAMD
 *   core exposes it as Wire with SDA GPIO 46 and SCL GPIO 45; the same net
 *   is available on the 40-pin header as I2C1 (BCM2/BCM3).
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 5 ("Header Interface and
 *   Button", J4) and Seeed-Studio/ArduinoCore-samd 1.8.6
 *   variants/wio_terminal/variant.h (PIN_WIRE_SDA / PIN_WIRE_SCL).
 * - The onboard LIS3DHTR is deliberately on a separate internal bus,
 *   Wire1, so it must not appear in this scan.
 * - J4 has no onboard I2C pull-up resistors. Source: schematic v1.2 page
 *   05 (J4 shows only the connector, decoupling and DNP SWD links). A Grove
 *   I2C module normally supplies the pull-ups; this sketch also enables the
 *   SAMD51 internal pull-ups so an empty bus does not float into a phantom
 *   ACK.
 *
 * Connect only 3.3 V Grove I2C devices. The scan is read-only: it sends the
 * address byte and checks for ACK, then stops.
 */

#include <TFT_eSPI.h>
#include <Wire.h>

TFT_eSPI tft;
uint32_t lastScanMs = 0;

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("GROVE I2C SCAN", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("Wire: SDA 46 / SCL 45 / left Grove J4", 10, 42, 2);
  tft.drawString("3.3 V devices only", 10, 64, 2);
  tft.drawRect(10, 90, 300, 30, TFT_DARKGREY);
  tft.drawString("addresses found", 10, 134, 2);
}

void drawStatus(const char *text, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(280);
  tft.drawString(text, 18, 97, 2);
  tft.setTextPadding(0);
}

void primeBus() {
  // Measured 2026-10-28: the first Wire transaction after an idle period
  // can report a false ACK for the first scanned address when J4 is empty.
  // A reserved-address transaction absorbs that artifact.
  Wire.beginTransmission(0x7F);
  (void)Wire.endTransmission();
  delayMicroseconds(50);
}

void scanBus() {
  uint8_t addresses[16];
  uint8_t found = 0;

  primeBus();
  for (uint8_t address = 0x08; address <= 0x77 && found < 16; ++address) {
    Wire.beginTransmission(address);
    if (Wire.endTransmission() == 0) {
      addresses[found++] = address;
      Serial.print("I2C device at 0x");
      if (address < 16) {
        Serial.print('0');
      }
      Serial.println(address, HEX);
    }
  }

  char status[32];
  snprintf(status, sizeof(status), "%u device%s", found, found == 1 ? "" : "s");
  drawStatus(status, found ? TFT_GREEN : TFT_ORANGE);

  tft.fillRect(10, 152, 300, 80, TFT_BLACK);
  tft.setTextColor(TFT_CYAN, TFT_BLACK);
  tft.setTextPadding(300);
  if (found == 0) {
    tft.drawString("no ACK on Wire", 10, 152, 2);
    tft.drawString("check Grove cable and 3V3", 10, 176, 2);
  } else {
    for (uint8_t i = 0; i < found; ++i) {
      char line[16];
      snprintf(line, sizeof(line), "0x%02X", addresses[i]);
      tft.drawString(line, 10 + (i % 8) * 36, 152 + (i / 8) * 26, 2);
    }
  }
  tft.setTextPadding(0);
}

void setup() {
  Serial.begin(115200);
  pinMode(PIN_WIRE_SDA, INPUT_PULLUP);
  pinMode(PIN_WIRE_SCL, INPUT_PULLUP);
  Wire.begin();
  Wire.setClock(100000);

  tft.begin();
  tft.setRotation(3);
  drawScreen();
  drawStatus("scanning...", TFT_YELLOW);
}

void loop() {
  if (millis() - lastScanMs < 3000) {
    delay(10);
    return;
  }
  lastScanMs = millis();
  scanBus();
}

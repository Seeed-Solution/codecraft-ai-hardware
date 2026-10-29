/*
 * Wio Terminal onboard LIS3DHTR accelerometer demo.
 *
 * Hardware facts:
 * - LIS3DHTR is on the internal I2C bus Wire1 (SDA 78, SCL 77), 7-bit
 *   address 0x18, with INT1 on GPIO 79.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 7 ("Gyroscope & LCD") and
 *   Seeed-Studio/ArduinoCore-samd 1.8.6 variants/wio_terminal/variant.h.
 *   The address follows Seeed's Wio Terminal IMU example, which calls
 *   lis.begin(Wire1) and therefore uses LIS3DHTR_DEFAULT_ADDRESS 0x18.
 * - Register semantics, WHO_AM_I value 0x33, 100 Hz ODR setting and the
 *   2 g conversion (raw16 / 16000 = g) follow Seeed's LIS3DHTR library
 *   1.2.4, src/LIS3DHTR.cpp and src/LIS3DHTR.h.
 *
 * The demo deliberately uses the vendor core's Wire1 directly; no external
 * sensor library is needed. Hold the board still during boot so the displayed
 * magnitude settles near 1 g.
 */

#include <TFT_eSPI.h>
#include <Wire.h>

static constexpr uint8_t kLisAddress = 0x18;
static constexpr uint8_t kRegWhoAmI = 0x0F;
static constexpr uint8_t kRegCtrl1 = 0x20;
static constexpr uint8_t kRegCtrl4 = 0x23;
static constexpr uint8_t kRegOutXl = 0x28;
static constexpr float kCountsPerG = 16000.0f;

TFT_eSPI tft;
bool sensorPresent = false;
uint32_t lastProbeMs = 0;
uint32_t lastSampleMs = 0;
int16_t lastPointX = 0;
int16_t lastPointY = 0;

bool writeRegister(uint8_t reg, uint8_t value) {
  Wire1.beginTransmission(kLisAddress);
  Wire1.write(reg);
  Wire1.write(value);
  return Wire1.endTransmission() == 0;
}

bool readRegisters(uint8_t reg, uint8_t *buffer, uint8_t length) {
  Wire1.beginTransmission(kLisAddress);
  // LIS3DHTR requires bit 7 for address auto-increment during a
  // multi-byte read. Seeed's LIS3DHTR driver does the same in
  // readRegisterRegion().
  Wire1.write(static_cast<uint8_t>(reg | 0x80));
  if (Wire1.endTransmission(false) != 0) {
    return false;
  }
  if (Wire1.requestFrom(kLisAddress, length) != length) {
    return false;
  }
  for (uint8_t i = 0; i < length; ++i) {
    buffer[i] = Wire1.read();
  }
  return true;
}

bool probeSensor() {
  uint8_t whoAmI = 0;
  if (!readRegisters(kRegWhoAmI, &whoAmI, 1) || whoAmI != 0x33) {
    return false;
  }

  // 100 Hz, normal mode, X/Y/Z enabled. BDU keeps a 6-byte burst coherent.
  if (!writeRegister(kRegCtrl1, 0x57)) {
    return false;
  }
  delay(5);
  return writeRegister(kRegCtrl4, 0x80);
}

bool readAcceleration(float &xg, float &yg, float &zg) {
  uint8_t data[6] = {};
  if (!readRegisters(kRegOutXl, data, sizeof(data))) {
    return false;
  }
  const int16_t rawX = static_cast<int16_t>((data[1] << 8) | data[0]);
  const int16_t rawY = static_cast<int16_t>((data[3] << 8) | data[2]);
  const int16_t rawZ = static_cast<int16_t>((data[5] << 8) | data[4]);
  xg = rawX / kCountsPerG;
  yg = rawY / kCountsPerG;
  zg = rawZ / kCountsPerG;
  return true;
}

void drawStaticScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("LIS3DHTR ACCELEROMETER", 10, 8, 2);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("Wire1: SDA 78 / SCL 77 / addr 0x18", 10, 30, 1);
  tft.drawRect(174, 66, 132, 132, TFT_DARKGREY);
  tft.drawFastHLine(174, 132, 132, TFT_DARKGREY);
  tft.drawFastVLine(240, 66, 132, TFT_DARKGREY);
}

void drawConnectionStatus(const char *message, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(200);
  tft.drawString(message, 10, 52, 2);
  tft.setTextPadding(0);
}

void drawVector(const char *label, float value, int y) {
  char buffer[20];
  snprintf(buffer, sizeof(buffer), "%s %+6.2f g", label, value);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.setTextPadding(160);
  tft.drawString(buffer, 10, y, 2);
  tft.setTextPadding(0);
}

void drawPoint(float xg, float yg) {
  const int16_t centerX = 240;
  const int16_t centerY = 132;
  const float scale = 40.0f;
  int16_t x = centerX + static_cast<int16_t>(constrain(xg * scale, -52.0f, 52.0f));
  int16_t y = centerY + static_cast<int16_t>(constrain(yg * scale, -52.0f, 52.0f));

  if (lastPointX != 0 || lastPointY != 0) {
    tft.fillCircle(lastPointX, lastPointY, 5, TFT_BLACK);
  }
  tft.drawFastHLine(174, 132, 132, TFT_DARKGREY);
  tft.drawFastVLine(240, 66, 132, TFT_DARKGREY);
  tft.fillCircle(x, y, 5, TFT_CYAN);
  lastPointX = x;
  lastPointY = y;
}

void setup() {
  Serial.begin(115200);
  Wire1.begin();
  Wire1.setClock(400000);

  tft.begin();
  tft.setRotation(3);
  drawStaticScreen();
  drawConnectionStatus("probing sensor...", TFT_YELLOW);
}

void loop() {
  if (!sensorPresent && millis() - lastProbeMs >= 1000) {
    lastProbeMs = millis();
    sensorPresent = probeSensor();
    if (sensorPresent) {
      drawConnectionStatus("sensor online", TFT_GREEN);
      Serial.println("LIS3DHTR online at 0x18 on Wire1");
    } else {
      drawConnectionStatus("NOT FOUND at 0x18", TFT_RED);
      Serial.println("LIS3DHTR not found at 0x18 on Wire1");
    }
  }

  if (!sensorPresent || millis() - lastSampleMs < 100) {
    return;
  }
  lastSampleMs = millis();

  float xg = 0;
  float yg = 0;
  float zg = 0;
  if (!readAcceleration(xg, yg, zg)) {
    sensorPresent = false;
    drawConnectionStatus("sensor read failed", TFT_RED);
    return;
  }

  // The board is used in landscape: the LIS3DHTR's Y axis points across the
  // screen (logical X) and its X axis points down the screen (logical Y).
  // The 180-degree screen rotation reverses the horizontal axis only, so X
  // is negated while Y keeps the sensor sign to make the ball roll downhill.
  // Measured on hardware; see AGENTS.md for the validation record.
  const float displayX = -yg;
  const float displayY = xg;

  drawVector("X", displayX, 74);
  drawVector("Y", displayY, 94);
  drawVector("Z", zg, 114);
  drawVector("|a|", sqrtf(xg * xg + yg * yg + zg * zg), 134);
  drawPoint(displayX, displayY);

  Serial.print("x=");
  Serial.print(displayX, 3);
  Serial.print(" y=");
  Serial.print(displayY, 3);
  Serial.print(" z=");
  Serial.println(zg, 3);
}

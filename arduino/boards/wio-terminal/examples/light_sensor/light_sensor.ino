/*
 * Wio Terminal onboard light-sensor demo.
 *
 * Hardware facts:
 * - The photodiode sensor output is wired to WIO_LIGHT, GPIO 27 /
 *   ADC channel 15, through the analog front end on schematic page 5.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 5 ("Header Interface and
 *   Button", SENSOR-PD15-22C/TR8) and Seeed-Studio/ArduinoCore-samd
 *   1.8.6 variants/wio_terminal/variant.h (WIO_LIGHT).
 * - SAMD51 ADC is configured for 12-bit reads in this sketch. The reported
 *   millivolt value assumes the nominal 3.3 V analog reference and is an
 *   estimate, not a calibrated lux measurement.
 *
 * The raw value direction is not asserted: cover and uncover the sensor and
 * use the observed change to calibrate an application-specific threshold.
 */

#include <TFT_eSPI.h>

static constexpr uint16_t kAdcMax = 4095;
static constexpr float kAdcReferenceMv = 3300.0f;

TFT_eSPI tft;
float filtered = 0;
uint32_t lastSampleMs = 0;

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("LIGHT SENSOR", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("WIO_LIGHT = GPIO 27 / ADC channel 15", 10, 42, 2);
  tft.drawString("Cover and uncover the sensor window", 10, 64, 2);
  tft.drawRect(10, 148, 300, 24, TFT_DARKGREY);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("raw", 10, 112, 2);
  tft.drawString("mV ", 10, 184, 2);
}

void drawValue(int y, int x, const char *value) {
  tft.setTextColor(TFT_CYAN, TFT_BLACK);
  tft.setTextPadding(210);
  tft.drawString(value, x, y, 4);
  tft.setTextPadding(0);
}

void setup() {
  Serial.begin(115200);
  analogReadResolution(12);

  tft.begin();
  tft.setRotation(3);
  drawScreen();
  filtered = analogRead(WIO_LIGHT);
}

void loop() {
  if (millis() - lastSampleMs < 100) {
    return;
  }
  lastSampleMs = millis();

  const int raw = analogRead(WIO_LIGHT);
  filtered = (filtered * 3.0f + raw) / 4.0f;

  char rawText[20];
  char mvText[20];
  snprintf(rawText, sizeof(rawText), "%4d / 4095", raw);
  snprintf(mvText, sizeof(mvText), "%4.0f mV", filtered * kAdcReferenceMv / kAdcMax);
  drawValue(110, 78, rawText);
  drawValue(180, 78, mvText);

  const int barWidth = map(static_cast<int>(filtered), 0, kAdcMax, 0, 296);
  tft.fillRect(12, 150, 296, 20, TFT_BLACK);
  if (barWidth > 0) {
    tft.fillRect(12, 150, barWidth, 20, TFT_GREEN);
  }
  tft.drawRect(10, 148, 300, 24, TFT_DARKGREY);

  Serial.print("light_raw=");
  Serial.print(raw);
  Serial.print(" filtered=");
  Serial.println(filtered, 1);
}

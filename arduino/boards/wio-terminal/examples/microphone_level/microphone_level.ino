/*
 * Wio Terminal onboard microphone level demo.
 *
 * Hardware facts:
 * - The onboard microphone is an analog electret microphone followed by an
 *   LMV358 gain stage; its output is MIC_OUTPUT, wired to WIO_MIC
 *   (GPIO 39 / ADC channel 12).
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 4 ("Buzzer & MIC", MIC1 and
 *   LMV358IDGKR) and Seeed-Studio/ArduinoCore-samd 1.8.6
 *   variants/wio_terminal/variant.h (WIO_MIC).
 * - The sketch removes the measured DC bias and reports AC peak-to-peak,
 *   RMS, and a relative level bar. It does not claim calibrated dB SPL.
 *
 * Testing: speak, whistle, or tap near the microphone and watch the bar.
 */

#include <TFT_eSPI.h>

static constexpr uint16_t kAdcMax = 4095;
static constexpr float kAdcReferenceMv = 3300.0f;
static constexpr int kSamples = 200;
static constexpr int kSampleDelayUs = 40;

TFT_eSPI tft;
uint32_t lastWindowMs = 0;

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("MICROPHONE LEVEL", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("WIO_MIC = GPIO 39 / ADC channel 12", 10, 42, 2);
  tft.drawString("Speak or whistle near the mic", 10, 64, 2);
  tft.drawRect(10, 160, 300, 24, TFT_DARKGREY);
  tft.drawString("peak-to-peak", 10, 108, 2);
  tft.drawString("RMS", 10, 132, 2);
  tft.drawString("relative level", 10, 194, 2);
}

void drawMetric(int y, const char *text) {
  tft.setTextColor(TFT_CYAN, TFT_BLACK);
  tft.setTextPadding(220);
  tft.drawString(text, 120, y, 2);
  tft.setTextPadding(0);
}

void captureWindow() {
  uint32_t sum = 0;
  uint16_t minimum = kAdcMax;
  uint16_t maximum = 0;
  uint16_t samples[kSamples];

  for (int i = 0; i < kSamples; ++i) {
    samples[i] = analogRead(WIO_MIC);
    sum += samples[i];
    minimum = min(minimum, samples[i]);
    maximum = max(maximum, samples[i]);
    delayMicroseconds(kSampleDelayUs);
  }

  const float mean = static_cast<float>(sum) / kSamples;
  float squareSum = 0;
  for (int i = 0; i < kSamples; ++i) {
    const float deviation = samples[i] - mean;
    squareSum += deviation * deviation;
  }

  const float rms = sqrtf(squareSum / kSamples);
  const uint16_t peakToPeak = maximum - minimum;
  const float peakToPeakMv = peakToPeak * kAdcReferenceMv / kAdcMax;

  char ppText[32];
  char rmsText[32];
  snprintf(ppText, sizeof(ppText), "%u counts / %4.0f mV", peakToPeak, peakToPeakMv);
  snprintf(rmsText, sizeof(rmsText), "%5.1f counts", rms);
  drawMetric(106, ppText);
  drawMetric(130, rmsText);

  const int barWidth = constrain(map(peakToPeak, 0, 1800, 0, 296), 0, 296);
  tft.fillRect(12, 162, 296, 20, TFT_BLACK);
  if (barWidth > 0) {
    uint16_t color = barWidth > 240 ? TFT_RED : (barWidth > 140 ? TFT_YELLOW : TFT_GREEN);
    tft.fillRect(12, 162, barWidth, 20, color);
  }
  tft.drawRect(10, 160, 300, 24, TFT_DARKGREY);

  Serial.print("mic_mean=");
  Serial.print(mean, 1);
  Serial.print(" pp=");
  Serial.print(peakToPeak);
  Serial.print(" rms=");
  Serial.println(rms, 2);
}

void setup() {
  Serial.begin(115200);
  analogReadResolution(12);

  tft.begin();
  tft.setRotation(3);
  drawScreen();
}

void loop() {
  if (millis() - lastWindowMs < 100) {
    return;
  }
  lastWindowMs = millis();
  captureWindow();
}

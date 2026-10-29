/*
 * Wio Terminal local Wi-Fi scan demo.
 *
 * Hardware facts:
 * - The wireless coprocessor is a Realtek RTL8720DN connected to the
 *   ATSAMD51 over SPI/UART with CHIP_PU on GPIO 80 and IRQ0 on GPIO 11.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 6 ("WIFI+BLE") and
 *   Seeed-Studio/ArduinoCore-samd 1.8.6 variants/wio_terminal/variant.h.
 * - The product supports 2.4 GHz and 5 GHz 802.11 a/b/g/n.
 *   Source: https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/
 * - Communication uses Seeed Arduino rpcWiFi 1.1.0 over rpcUnified 2.1.4.
 *   The board README pins the complete dependency set.
 *
 * This is a local scan: no credentials, association, or cloud service.
 * If scanning fails, verify that the RTL8720DN eRPC firmware is present and
 * that another wireless demo is not holding the coprocessor.
 */

#include <TFT_eSPI.h>
#include <rpcWiFi.h>

static constexpr int kMaxShownNetworks = 5;

struct Network {
  String ssid;
  int32_t rssi;
};

TFT_eSPI tft;
Network topNetworks[kMaxShownNetworks];
uint32_t lastScanMs = 0;
bool firstScan = true;

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("WI-FI SCAN", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("RTL8720DN / local scan / no credentials", 10, 42, 2);
  tft.drawRect(10, 68, 300, 30, TFT_DARKGREY);
  tft.drawString("networks by RSSI", 10, 112, 2);
}

void drawStatus(const char *text, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(280);
  tft.drawString(text, 18, 75, 2);
  tft.setTextPadding(0);
}

void insertNetwork(const String &ssid, int32_t rssi, int &count) {
  int position = count;
  while (position > 0 && rssi > topNetworks[position - 1].rssi) {
    if (position < kMaxShownNetworks) {
      topNetworks[position] = topNetworks[position - 1];
    }
    --position;
  }
  if (position < kMaxShownNetworks) {
    topNetworks[position].ssid = ssid;
    topNetworks[position].rssi = rssi;
    if (count < kMaxShownNetworks) {
      ++count;
    }
  }
}

void runScan() {
  drawStatus("scanning...", TFT_YELLOW);
  const int16_t networkCount = WiFi.scanNetworks();
  if (networkCount < 0) {
    drawStatus("scan failed", TFT_RED);
    Serial.println("WiFi.scanNetworks() failed");
    WiFi.scanDelete();
    return;
  }

  int retained = 0;
  for (int i = 0; i < networkCount; ++i) {
    String ssid = WiFi.SSID(i);
    if (ssid.length() == 0) {
      ssid = "(hidden)";
    }
    insertNetwork(ssid, WiFi.RSSI(i), retained);
  }

  char status[40];
  snprintf(status, sizeof(status), "%d found / top %d", networkCount, retained);
  drawStatus(status, retained ? TFT_GREEN : TFT_ORANGE);

  tft.fillRect(10, 132, 300, 96, TFT_BLACK);
  tft.setTextColor(TFT_CYAN, TFT_BLACK);
  tft.setTextPadding(300);
  for (int i = 0; i < retained; ++i) {
    String name = topNetworks[i].ssid.substring(0, 22);
    char line[48];
    snprintf(line, sizeof(line), "%d %-22s %4ld", i + 1, name.c_str(),
             static_cast<long>(topNetworks[i].rssi));
    tft.drawString(line, 10, 132 + i * 18, 2);
    Serial.println(line);
  }
  tft.setTextPadding(0);

  WiFi.scanDelete();
}

void setup() {
  Serial.begin(115200);

  tft.begin();
  tft.setRotation(3);
  drawScreen();
  drawStatus("starting radio...", TFT_YELLOW);

  if (!WiFi.mode(WIFI_STA)) {
    drawStatus("radio init failed", TFT_RED);
    Serial.println("WiFi.mode(WIFI_STA) failed");
  } else {
    WiFi.disconnect();
    drawStatus("ready", TFT_GREEN);
  }
}

void loop() {
  if (!firstScan && millis() - lastScanMs < 10000) {
    delay(20);
    return;
  }
  firstScan = false;
  lastScanMs = millis();
  runScan();
}

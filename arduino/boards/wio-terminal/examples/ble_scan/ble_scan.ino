/*
 * Wio Terminal local BLE scan demo.
 *
 * Hardware facts:
 * - The RTL8720DN coprocessor provides BLE and is connected to the
 *   ATSAMD51 over SPI/UART; CHIP_PU is GPIO 80 and IRQ0 is GPIO 11.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 6 ("WIFI+BLE") and
 *   Seeed-Studio/ArduinoCore-samd 1.8.6 variants/wio_terminal/variant.h.
 * - The product lists BLE 5.0 support.
 *   Source: https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/
 * - The sketch uses Seeed Arduino rpcBLE 1.0.0 over rpcUnified 2.1.4
 *   against the official eRPC coprocessor firmware 2.1.3.
 *   Versions and the complete dependency set are pinned in the board README.
 *
 * Measured on a Wio Terminal running coprocessor firmware 2.1.3:
 * - ble_start() must be called once after BLEDevice::init(); otherwise the
 *   coprocessor never brings its BT stack up, BD_ADDR reads 00:00:00:00:00:00
 *   and a scan returns no advertisements.
 * - BLEScan::start(duration, false) blocks on a semaphore with no timeout and
 *   froze the UI on this unit. This demo uses the non-blocking
 *   start(duration, callback, continue) overload and polls from loop().
 *
 * This is a local active scan: it sends BLE scan requests for discovery but
 * does not pair, connect, or transmit application data. Run it separately
 * from the Wi-Fi demo because both demonstrations own the coprocessor.
 */

#include <TFT_eSPI.h>

#ifdef min
#undef min
#endif
#ifdef max
#undef max
#endif

#include <rpcBLEDevice.h>
#include <BLEScan.h>
#include <BLEAdvertisedDevice.h>
#include <erpc/erpc_port.h>

static constexpr int kMaxShownDevices = 5;
static constexpr uint32_t kScanMs = 5000;
static constexpr uint32_t kIdleMs = 4000;

struct DeviceSummary {
  String name;
  String address;
  int rssi;
};

enum ScanPhase {
  kArming,
  kScanning,
  kIdle,
};

class ScanCallbacks : public BLEAdvertisedDeviceCallbacks {
  void onResult(BLEAdvertisedDevice device) override;
};

TFT_eSPI tft;
BLEScan *bleScan = nullptr;
DeviceSummary topDevices[kMaxShownDevices];
volatile uint32_t advertCount = 0;
int storedCount = 0;
ScanPhase phase = kArming;
uint32_t scanStartMs = 0;
uint32_t idleStartMs = 0;

void drawText(int16_t x, int16_t y, uint8_t size, const char *text, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(300);
  tft.drawString(text, x, y, size);
  tft.setTextPadding(0);
}

void drawStatus(const char *text, uint16_t color) {
  drawText(18, 83, 2, text, color);
}

void insertDevice(const String &name, const String &address, int rssi, int &count) {
  int position = count;
  while (position > 0 && rssi > topDevices[position - 1].rssi) {
    if (position < kMaxShownDevices) {
      topDevices[position] = topDevices[position - 1];
    }
    --position;
  }
  if (position < kMaxShownDevices) {
    topDevices[position].name = name;
    topDevices[position].address = address;
    topDevices[position].rssi = rssi;
    if (count < kMaxShownDevices) {
      ++count;
    }
  }
}

void ScanCallbacks::onResult(BLEAdvertisedDevice device) {
  ++advertCount;
  String name = device.getName().c_str();
  if (name.length() == 0) {
    name = "(unnamed)";
  }
  insertDevice(name, String(device.getAddress().toString().c_str()), device.getRSSI(),
               storedCount);
}

void onScanComplete(BLEScanResults) {}

void drawList() {
  tft.fillRect(10, 118, 300, 100, TFT_BLACK);
  for (int i = 0; i < storedCount; ++i) {
    String name = topDevices[i].name.substring(0, 13);
    char line[56];
    snprintf(line, sizeof(line), "%-13s %s %4d", name.c_str(),
             topDevices[i].address.c_str(), topDevices[i].rssi);
    drawText(10, 118 + i * 16, 1, line, TFT_CYAN);
    Serial.println(line);
  }
}

void armScan() {
  advertCount = 0;
  storedCount = 0;
  tft.fillRect(10, 118, 300, 100, TFT_BLACK);
  drawStatus("arming scan...", TFT_YELLOW);
  Serial.println("scan: arming");

  if (!bleScan->start(kScanMs / 1000, onScanComplete, false)) {
    drawStatus("scan start failed", TFT_RED);
    Serial.println("scan: start failed");
    phase = kIdle;
    idleStartMs = millis();
    return;
  }
  scanStartMs = millis();
  phase = kScanning;
}

void finishScan() {
  bleScan->stop();

  char status[40];
  snprintf(status, sizeof(status), "%lu adv / %d shown",
           static_cast<unsigned long>(advertCount), storedCount);
  drawStatus(status, storedCount ? TFT_GREEN : TFT_ORANGE);
  drawList();
  Serial.printf("scan: done (%lu adv)\n", static_cast<unsigned long>(advertCount));

  bleScan->clearResults();
  phase = kIdle;
  idleStartMs = millis();
}

void pollScan() {
  const uint32_t elapsed = millis() - scanStartMs;
  if (elapsed < kScanMs) {
    char status[40];
    snprintf(status, sizeof(status), "scanning %lus  %lu adv",
             static_cast<unsigned long>(elapsed / 1000 + 1),
             static_cast<unsigned long>(advertCount));
    drawStatus(status, TFT_YELLOW);
    return;
  }
  finishScan();
}

void setup() {
  Serial.begin(115200);

  tft.begin();
  tft.setRotation(3);
  tft.fillScreen(TFT_BLACK);
  drawText(10, 8, 4, "BLE SCAN", TFT_WHITE);
  tft.drawRect(10, 76, 300, 30, TFT_DARKGREY);
  drawStatus("starting radio...", TFT_YELLOW);

  char *version = rpc_system_version();
  String rtlVersion = version ? version : "unknown";
  if (version) {
    erpc_free(version);
  }
  drawText(10, 44, 2, ("RTL " + rtlVersion).c_str(), TFT_CYAN);

  BLEDevice::init("Wio Terminal");
  // Required on coprocessor firmware 2.1.3: the BT stack only comes up in
  // ble_start(), which BLEDevice::init() does not call.
  ble_start();
  drawText(10, 62, 1, ("local " + String(BLEDevice::getAddress().toString().c_str())).c_str(),
           TFT_DARKGREY);

  bleScan = BLEDevice::getScan();
  bleScan->setAdvertisedDeviceCallbacks(new ScanCallbacks());
  bleScan->setActiveScan(true);
  bleScan->setInterval(100);
  bleScan->setWindow(99);
  drawStatus("ready", TFT_GREEN);
}

void loop() {
  switch (phase) {
    case kArming:
      armScan();
      break;
    case kScanning:
      pollScan();
      break;
    case kIdle:
      if (millis() - idleStartMs >= kIdleMs) {
        phase = kArming;
      }
      break;
  }
  delay(100);
}

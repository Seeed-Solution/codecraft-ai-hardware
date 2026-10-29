/*
 * Wio Terminal microSD card demo.
 *
 * Hardware facts:
 * - The microSD socket (ST-TF-003J) uses the dedicated SPI2 bus:
 *   SCK 63, MOSI 62, MISO 61, CS 64, card-detect 65.
 *   Source: Wio-Terminal-SCH-v1.2.pdf page 2 ("ATSAMD51P19A") and
 *   Seeed-Studio/ArduinoCore-samd 1.8.6 variants/wio_terminal/variant.h
 *   (SDCARD_SPI, SDCARD_*_PIN).
 * - The product page states a maximum supported card size of 16 GB.
 *   Source: https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/
 * - Seeed Arduino FS 2.1.5 provides an SDFS::begin() overload that accepts an
 *   explicit SPIClass. Mount with SDCARD_SPI (SPI2) and SDCARD_SS_PIN (CS 64);
 *   the standard Arduino SD library would default to SS (GPIO 50) instead.
 *
 * Use a FAT32-formatted card. The sketch lists the root directory and appends
 * one line to /wio_terminal.log each time a card is mounted.
 */

#include <SPI.h>
#include <Seeed_Arduino_FS.h>
#include <TFT_eSPI.h>

TFT_eSPI tft;
bool mounted = false;
bool logged = false;
uint32_t lastRetryMs = 0;

void drawScreen() {
  tft.fillScreen(TFT_BLACK);
  tft.setTextColor(TFT_WHITE, TFT_BLACK);
  tft.drawString("MICROSD CARD", 10, 8, 4);
  tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
  tft.drawString("SPI2: SCK 63 / MOSI 62 / MISO 61 / CS 64", 10, 42, 1);
  tft.drawString("FAT32, maximum 16 GB", 10, 58, 2);
  tft.drawRect(10, 80, 300, 30, TFT_DARKGREY);
  tft.drawString("root directory", 10, 122, 2);
}

void drawStatus(const char *text, uint16_t color) {
  tft.setTextColor(color, TFT_BLACK);
  tft.setTextPadding(280);
  tft.drawString(text, 18, 87, 2);
  tft.setTextPadding(0);
}

void listRoot() {
  tft.fillRect(10, 140, 300, 92, TFT_BLACK);
  tft.setTextColor(TFT_CYAN, TFT_BLACK);
  tft.setTextPadding(300);
  File root = SD.open("/");
  if (!root) {
    tft.drawString("cannot open root", 10, 140, 2);
    tft.setTextPadding(0);
    return;
  }

  int shown = 0;
  while (shown < 6) {
    File entry = root.openNextFile();
    if (!entry) {
      break;
    }
    const char *name = entry.name();
    const char *slash = strrchr(name, '/');
    if (slash != nullptr) {
      name = slash + 1;
    }
    char line[40];
    snprintf(line, sizeof(line), "%s%s", entry.isDirectory() ? "[D] " : "    ",
             name);
    tft.drawString(line, 10, 140 + shown * 14, 1);
    Serial.println(line);
    entry.close();
    ++shown;
  }
  root.close();
  tft.setTextPadding(0);
  if (shown == 0) {
    tft.drawString("(empty root)", 10, 140, 2);
  }
}

void writeLogLine() {
  File log = SD.open("/wio_terminal.log", FILE_APPEND);
  if (!log) {
    Serial.println("could not open /wio_terminal.log");
    return;
  }
  log.print("wio-terminal sd_card_log mounted at millis=");
  log.println(millis());
  log.close();
  Serial.println("appended /wio_terminal.log");

  File verify = SD.open("/wio_terminal.log", FILE_READ);
  if (!verify) {
    Serial.println("could not read back /wio_terminal.log");
    return;
  }
  char line[96];
  char lastLine[96] = "";
  while (verify.gets(line, sizeof(line)) != nullptr) {
    size_t length = strlen(line);
    while (length > 0 && (line[length - 1] == '\n' || line[length - 1] == '\r')) {
      line[--length] = '\0';
    }
    strncpy(lastLine, line, sizeof(lastLine) - 1);
    lastLine[sizeof(lastLine) - 1] = '\0';
  }
  const uint32_t logBytes = verify.size();
  verify.close();
  Serial.print("log_bytes=");
  Serial.print(logBytes);
  Serial.print(" last_line=");
  Serial.println(lastLine);
}

void setup() {
  Serial.begin(115200);
  const uint32_t serialWaitStart = millis();
  while (!Serial && millis() - serialWaitStart < 5000) {
    delay(10);
  }
  Serial.println("sd_card_log boot");

  tft.begin();
  tft.setRotation(3);
  drawScreen();
  drawStatus("mounting...", TFT_YELLOW);
}

void loop() {
  if (!mounted && millis() - lastRetryMs >= 2000) {
    lastRetryMs = millis();
    mounted = SD.begin(SDCARD_SS_PIN, SDCARD_SPI, 4000000UL);
    if (mounted) {
      drawStatus("mounted", TFT_GREEN);
      Serial.println("SD mounted");
      listRoot();
      if (!logged) {
        writeLogLine();
        logged = true;
      }
      tft.setTextColor(TFT_LIGHTGREY, TFT_BLACK);
      tft.drawString("log: /wio_terminal.log", 10, 232, 1);
    } else {
      SD.end();
      drawStatus("no card / mount failed", TFT_RED);
      Serial.println("SD mount failed; retrying");
    }
  }
  delay(20);
}

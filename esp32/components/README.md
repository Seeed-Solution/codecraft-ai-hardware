# Shared ESP32 Components

Components placed here must be used by at least two boards. Single-board
drivers belong in the board's own `bsp/` directory.

Landed:

- `sscma_client/` — generic still-camera abstraction plus the SSCMA-over-SPI
  backend (Seeed Himax WiseEye2 on the SenseCAP Watcher; also fits a Grove
  Vision AI V2). Extracted from muse-gadget-sdk `esp32/components/camera/`
  (Apache 2.0, Meta copyright headers retained, modifications marked).

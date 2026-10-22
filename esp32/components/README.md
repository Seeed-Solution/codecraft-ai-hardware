# Shared ESP32 Components

Components placed here must be used by at least two boards. Single-board
drivers belong in the board's own `bsp/` directory.

Planned:

- `sscma_client/` — Himax camera client for SenseCAP Watcher, extracted from
  muse-gadget-sdk `esp32/components/camera/` (Apache 2.0, Meta copyright
  headers retained). Shared if a second SSCMA board lands.

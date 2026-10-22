---
name: codecraft-device-test
description: Flash firmware to a device and verify behavior on hardware for codecraft-ai-hardware boards. Use for flashing and on-device testing; requires explicit user authorization for flashing.
---

# Device Test

1. Flashing requires explicit user authorization for each session. Ask if it
   was not granted.
2. Confirm the port and board identity before flashing (e.g. reSpeaker
   Lite's console is the small XIAO board's USB, `303a:1001`).
3. Do not perform a flash readback as a precondition, and never erase the
   whole chip unless the user explicitly asks.
4. After flashing, verify the behavior the change claims: observe logs and
   the physical function (display, LED, button, audio) and report exactly
   what was observed.
5. Note any board-specific prerequisites (e.g. the reSpeaker Lite XMOS must
   run Seeed's 16 kHz I2S firmware v1.0.9 — ESP32 flashing does not update
   the XMOS).

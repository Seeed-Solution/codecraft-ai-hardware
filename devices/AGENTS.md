# Device directory

This directory index is generated from the authored Device Markdown entries.

Each device has its own directory and AGENTS.md knowledge entry. Keep hardware knowledge, references and assets there; targets/ owns its native BSP, examples and board configuration. Use reference/ for supporting material and assets/ for actual registered media when needed. Shared stack/runtime adapters, build helpers and reusable components belong in platforms/<stack>/. Board-specific tests live in tests/devices/<device>/targets/; observations, records and logs live alongside that test subtree.

## [Grove Beginner Kit for Arduino](grove-beginner-kit/AGENTS.md)

ATmega328P kit with ten Grove modules and standalone bus diagnostics.

Use the standalone native sketches and their explicit physical test procedures.

## [reSpeaker Lite (XIAO ESP32-S3)](respeaker-lite/AGENTS.md)

XMOS I2S audio, BOOT input and RGB LED.

Use the listed standalone examples as the supported development entry points.

## [reTerminal (CM4)](reterminal/AGENTS.md)

Linux peripheral overlay and input, LED, buzzer, sensor and GPIO demos.

Use the listed standalone examples as the supported development entry points.

## [reTerminal E1001](reterminal-e1001/AGENTS.md)

Monochrome e-paper with keys, environmental sensing, power, RTC and storage.

Use the listed standalone examples as the supported development entry points.

## [reTerminal E1002](reterminal-e1002/AGENTS.md)

Six-color e-paper with keys, environmental sensing, power, RTC and storage.

Use the listed standalone examples as the supported development entry points.

## [reTerminal Sticky](reterminal-sticky/AGENTS.md)

Compact magnetic 3.97-inch 4-level grayscale e-paper device with touch, three buttons, environmental/motion sensors, RTC, microphone, buzzer, microSD, Wi-Fi/BLE and a 750 mAh battery.

Best for low-frequency, always-visible notes, weather, schedules and status dashboards. Pocket size makes it suitable for both handheld and stationary use. Stick it to magnetic surfaces, or the back of your phone. Mount it to any surface with the adhesive magnetic ring accessory.

Use the listed standalone Target examples (`epd_hello`, `button_hello`, `sht4x`) as the supported development entry points. The guides under `reference/` and the Intent routing table below map common tasks (display refresh, RTC clock, battery, navigation, sleep, microphone, microSD, IMU) to implementation patterns. Export private applications into a project workspace through the development workflow instead of editing the reference examples.

## [SenseCAP Indicator](sensecap-indicator/AGENTS.md)

RGB display, touch and a case button through native examples; sensor co-processor interfaces are outside the registered Target scope.

Use the listed standalone examples as the supported development entry points.

## [SenseCAP Watcher](sensecap-watcher/AGENTS.md)

Display, touch, camera, audio, knob, battery and microSD reference implementations.

Use the listed standalone examples as the supported development entry points.

## [Wio Terminal](wio-terminal/AGENTS.md)

SAMD51 terminal with LCD, buttons, sensors, microSD and separate Wi-Fi/BLE scan examples.

Use the standalone native sketches and their explicit physical test procedures.

## [Wio Tracker L2](wio-tracker-l2/AGENTS.md)

Portable Wio-S3 LoRa tracker for off-grid messaging. This entry covers the
L2 native Wi-Fi scan example and local connectivity workflows on L2 units
running Meshtastic.

Use the native Wi-Fi scan when developing an ESP-IDF application. For local
operations on Meshtastic firmware, use the existing client transports and
region configuration.

## [XIAO ESP32-S3 1.47-inch Touch Display](xiao-s3-147/AGENTS.md)

Small LCD, two buttons and PDM microphone; touch and storage are outside the registered Target interface scope.

Use the listed standalone examples as the supported development entry points.

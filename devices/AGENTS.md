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

Start from the Intent routing table below. The registered Target provides the standalone display, button and SHT40 examples (`epd_hello`, `button_hello`, `sht4x`). The guides under `reference/` explain display refresh, RTC, battery, navigation, sleep, microphone, microSD and IMU implementation patterns; a guide alone does not establish a supported native interface or verified hardware behavior. Export private applications into a project workspace through the development workflow instead of editing the reference examples.

## [SenseCAP Indicator](sensecap-indicator/AGENTS.md)

RGB display, touch and a case button through native examples; sensor co-processor interfaces are outside the registered Target scope.

Use the listed standalone examples as the supported development entry points.

## [SenseCAP Watcher](sensecap-watcher/AGENTS.md)

Display, touch, camera, audio, knob, battery and microSD reference implementations.

Use the listed standalone examples as the supported development entry points.

## [Wio Terminal](wio-terminal/AGENTS.md)

SAMD51 terminal with LCD, buttons, sensors, microSD and separate Wi-Fi/BLE scan examples.

Use the standalone native sketches and their explicit physical test procedures.

## [XIAO ESP32-S3 1.47-inch Touch Display](xiao-s3-147/AGENTS.md)

Small LCD, two buttons and PDM microphone; touch and storage are outside the registered Target interface scope.

Use the listed standalone examples as the supported development entry points.

/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * XIAO ESP32-S3 1.47" Touch Display pin map — the board's source of truth.
 *
 * Sources, highest priority first (see docs/vendor-sources.md):
 *  1. Seeed wiki: getting_started_1.47_inch_touch_display_esp32s3 and the
 *     XIAO ESP32-S3 Plus pin map (wiki.seeedstudio.com).
 *  2. Seeed_GFX2 (MIT): github.com/Seeed-Studio/Seeed_GFX2 — JD9853A
 *     register sequence, AXS5106L touch reports, 0519_DashBoard_147_ESP32
 *     PDM example.
 *  3. Meta muse-gadget-sdk (Apache 2.0), field-verified on one unit:
 *     esp32/components/muse/boards/board_seeed_xiao_s3_147.c and
 *     esp32/devices/seeed-xiao-s3-147.md (pin table).
 */
#pragma once

/* ---- LCD: JD9853A (ST7789-compatible), 172x320 IPS, SPI2 ---- */
#define BSP_PIN_LCD_SCLK   7   /* D8 */
#define BSP_PIN_LCD_MOSI   9   /* D10 */
#define BSP_PIN_LCD_CS     3   /* D2 */
#define BSP_PIN_LCD_DC     4   /* D3 */
#define BSP_PIN_LCD_RST    13  /* D17, shared with the touch reset */
#define BSP_PIN_LCD_BL     12  /* D18, active high */
#define BSP_LCD_HRES       172
#define BSP_LCD_VRES       320

/* ---- Capacitive touch: AXS5106L @0x63 on I2C ----
 * Reset line shared with the LCD (BSP_PIN_LCD_RST); X is mirrored to
 * match the panel orientation. */
#define BSP_PIN_TOUCH_SDA  5   /* D4 */
#define BSP_PIN_TOUCH_SCL  6   /* D5 */
#define BSP_PIN_TOUCH_INT  44  /* D7 */

/* ---- Buttons (active low, internal pull-up) ---- */
#define BSP_PIN_BUTTON_USR1 11  /* D19 */
#define BSP_PIN_BUTTON_USR2 42  /* D15 */

/* ---- PDM microphone (I2S0, the S3's only PDM receiver) ---- */
#define BSP_PIN_PDM_CLK    1   /* D0 */
#define BSP_PIN_PDM_DATA   2   /* D1 */

/* ---- microSD: CS held high so the unused shared-SPI device stays
 * deselected. The slot is otherwise unused so far. ---- */
#define BSP_PIN_SD_CS      43  /* D6 */

/* UART0's GPIO43/44 carry SD CS and the touch interrupt, so the console
 * runs on native USB Serial/JTAG (see sdkconfig.defaults). */

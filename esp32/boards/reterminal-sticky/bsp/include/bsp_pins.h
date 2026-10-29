/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal Sticky pin map — the board's source of truth.
 *
 * Sources, highest priority first (see docs/vendor-sources.md):
 *  1. Seeed reTerminal Sticky hardware overview:
 *     seeedstudio.com/sticky/docs/en/device-guide/hardware-overview/
 *  2. Seeed-reTerminal-E10xx-Firmware, factory_fw
 *     main/boards/reterminal_e1005/{config.h,reterminal_e1005.cpp} (MIT,
 *     Copyright (c) 2026 JoJang2u).
 *  3. Meta muse-gadget-sdk (Apache 2.0), field-verified on one unit:
 *     esp32/components/muse/boards/board_reterminal_sticky.c (buttons,
 *     I2C buses, BQ27220, GT911 touch, PDM mic, SD power/detect),
 *     esp32/main/epd_ssd1677.c (panel SPI pins; MIT, JoJang2u/limengdu),
 *     esp32/main/reterminal_sht4x.c (SHT4x on I2C1 for the Sticky).
 */
#pragma once

/* ---- E-paper panel: SSD1677, native 800x480, used portrait 480x800 ----
 * SPI2 bus shared with the microSD card. */
#define BSP_PIN_EPD_SCLK   13
#define BSP_PIN_EPD_MOSI   14
#define BSP_PIN_EPD_MISO   12  /* unused by the panel; needed by microSD */
#define BSP_PIN_EPD_CS     15
#define BSP_PIN_EPD_DC     16  /* high = data */
#define BSP_PIN_EPD_RST    17  /* active low */
#define BSP_PIN_EPD_BUSY   18  /* high = busy */
#define BSP_PIN_EPD_EN     47  /* panel power, active high */
#define BSP_EPD_SPI_HZ     (20 * 1000 * 1000)

/* ---- Keys (all active low, internal pull-up) ---- */
#define BSP_PIN_KEY_AI     4
#define BSP_PIN_KEY_UP     5
#define BSP_PIN_KEY_DOWN   6

/* ---- I2C1 sensor bus: SHT4x @0x44, BQ27220 fuel gauge @0x55 ---- */
#define BSP_PIN_I2C1_SDA   1
#define BSP_PIN_I2C1_SCL   0

/* ---- GT911 capacitive touch on I2C0 ----
 * Address 0x5D (fallback 0x14); the digitizer counts Y from the bottom. */
#define BSP_PIN_I2C0_SDA   3
#define BSP_PIN_I2C0_SCL   2
#define BSP_PIN_TOUCH_PWR  42  /* touch power, active high */
#define BSP_PIN_TOUCH_RST  41  /* active low */
#define BSP_PIN_TOUCH_INT  21

/* ---- PDM microphone (I2S0, the S3's only PDM receiver) ---- */
#define BSP_PIN_PDM_CLK    19
#define BSP_PIN_PDM_DATA   20
#define BSP_PIN_PDM_EN     38  /* mic power, active high */

/* ---- microSD (SDSPI on the panel's SPI2 bus) ---- */
#define BSP_PIN_SD_CS      8   /* idle high */
#define BSP_PIN_SD_PWR     10  /* card power, active high */
#define BSP_PIN_SD_DET     11  /* low = inserted */

/* ---- Charger ----
 * The muse board file drives this low at boot and logs it as "charger
 * enable"; the active level is not confirmed against factory_fw yet.
 * Treat as unverified until measured on hardware. */
#define BSP_PIN_CHARGER_EN 39

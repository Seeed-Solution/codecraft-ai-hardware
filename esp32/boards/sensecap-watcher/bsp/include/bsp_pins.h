/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher pin map — the board's source of truth.
 *
 * Sources, highest priority first (see docs/vendor-sources.md):
 *  1. Vendor: Seeed-Studio/SenseCAP-Watcher-Firmware (Apache 2.0)
 *     components/sensecap-watcher/include/sensecap-watcher.h — every pin,
 *     the PCA9535 expander map, and the power-rail assignments;
 *     components/sensecap-watcher/sensecap-watcher.c — expander address
 *     0x21 (PCA9535, A2A1A0 = 001), input/output direction masks.
 *  2. Field-verified camera wiring: Meta muse-gadget-sdk (Apache 2.0)
 *     esp32/components/muse/boards/board_sensecap_watcher.c — SPI2 pins,
 *     expander bits, and the confirmed-quirk that the Himax is reset by
 *     power-cycling its rail (expander pin 7, the vendor's SSCMA_RST, is
 *     left an input).
 *  3. Field-verified memory config: muse esp32/devices/
 *     sdkconfig.muse-sensecap-watcher — 32 MB flash, 8 MB octal PSRAM
 *     @80 MHz with XIP.
 */
#pragma once

/* ---- I2C0: general bus (expander, RTC PCF8563 @0x51, audio codecs) ---- */
#define BSP_PIN_I2C_SDA            47
#define BSP_PIN_I2C_SCL            48
#define BSP_I2C_FREQ_HZ            400000

/* ---- PCA9535 16-bit IO expander (I2C0 @0x21) ---- */
#define BSP_PIN_EXPANDER_INT       2   /* low when an input changes */

/* Expander pin numbers (vendor IO_EXPANDER_PIN_NUM_x). */
#define BSP_EXP_PIN_CHRG_DET       0   /* in: charging */
#define BSP_EXP_PIN_STDBY_DET      1   /* in: charge done */
#define BSP_EXP_PIN_VBUS_IN_DET    2   /* in: USB power present */
#define BSP_EXP_PIN_KNOB_BTN       3   /* in: wheel push, low while pressed */
#define BSP_EXP_PIN_SD_DET         4   /* in: SD card detect */
#define BSP_EXP_PIN_TOUCH_INT      5   /* in: touch interrupt */
#define BSP_EXP_PIN_CAM_SYNC       6   /* in: high while the Himax has a reply waiting */
#define BSP_EXP_PIN_CAM_RST        7   /* vendor SSCMA_RST; unused — see camera notes */
#define BSP_EXP_PIN_PWR_SDCARD     8   /* out: SD card rail */
#define BSP_EXP_PIN_PWR_LCD        9   /* out: display rail */
#define BSP_EXP_PIN_PWR_SYSTEM     10  /* out: holds the board on from battery */
#define BSP_EXP_PIN_PWR_AI_CHIP    11  /* out: Himax vision coprocessor rail */
#define BSP_EXP_PIN_PWR_CODEC_PA   12  /* out: audio codec + amp rail */
#define BSP_EXP_PIN_BAT_DET        13  /* in: battery present */
#define BSP_EXP_PIN_PWR_GROVE      14  /* out: Grove connector rail */
#define BSP_EXP_PIN_PWR_BAT_ADC    15  /* out: battery divider rail */

/* ---- SPI2: shared by the Himax camera and the SD card ---- */
#define BSP_PIN_SPI2_SCLK          4
#define BSP_PIN_SPI2_MOSI          5
#define BSP_PIN_SPI2_MISO          6

/* ---- Camera: Himax WiseEye2, SSCMA over SPI2 @12 MHz mode 0 ---- */
#define BSP_PIN_CAM_CS             21

/* ---- SD card (SPI2 device 2) ---- */
#define BSP_PIN_SD_CS              46

/* ---- LCD: SPD2010 412x412 QSPI on SPI3, touch over I2C1 ---- */
#define BSP_PIN_LCD_PCLK           7
#define BSP_PIN_LCD_D0             9
#define BSP_PIN_LCD_D1             1   /* also LCD D/C */
#define BSP_PIN_LCD_D2             14
#define BSP_PIN_LCD_D3             13
#define BSP_PIN_LCD_CS             45
#define BSP_PIN_LCD_BACKLIGHT      8
#define BSP_PIN_TOUCH_SDA          39  /* I2C1 */
#define BSP_PIN_TOUCH_SCL          38  /* I2C1 */

/* ---- Audio: ES8311 (DAC) + ES7243 (ADC) on I2C0, I2S0 ---- */
#define BSP_PIN_I2S_MCLK           10
#define BSP_PIN_I2S_BCLK           11
#define BSP_PIN_I2S_WS             12
#define BSP_PIN_I2S_DIN            15
#define BSP_PIN_I2S_DOUT           16

/* ---- Himax firmware flashing UART (not needed to run the camera) ---- */
#define BSP_PIN_SSCMA_UART_TX      17  /* UART1 */
#define BSP_PIN_SSCMA_UART_RX      18  /* UART1 */

/* ---- Misc ---- */
#define BSP_PIN_RGB_LED            40  /* WS2812 */
#define BSP_PIN_KNOB_A             41
#define BSP_PIN_KNOB_B             42
#define BSP_ADC_CHANNEL_BATTERY    2   /* GPIO3, divider (62k+20k)/20k */

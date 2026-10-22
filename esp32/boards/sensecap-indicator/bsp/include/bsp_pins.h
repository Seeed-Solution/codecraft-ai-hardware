/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Indicator (D1/D1S/D1Pro/D1L) pin map — the board's source of truth.
 *
 * Sources, highest priority first (see docs/vendor-sources.md):
 *  1. Vendor: Seeed-Solution/SenseCAP_Indicator_ESP32 (Apache 2.0)
 *     components/bsp/src/boards/sensecap_indicator_board.c — RGB bus pins,
 *     backlight, button, I2C, TCA9535 IO expander map;
 *     components/bsp/src/boards/lcd_panel_config.c — ST7701S init channel
 *     (9-bit 3-wire SPI: SCK=GPIO41, SDO=GPIO48, CS via expander IO4);
 *     components/bsp/Kconfig.projbuild — PCLK 18 MHz, 480x480, GX screen.
 *  2. Field-verified memory config: Meta muse-gadget-sdk
 *     esp32/devices/sdkconfig.sensecap-indicator (Apache 2.0) —
 *     8 MB QIO flash @80 MHz, 8 MB octal PSRAM @80 MHz.
 */
#pragma once

/* ---- User input ---- */
#define BSP_PIN_BUTTON_USER        38  /* top case button, active low */

/* ---- RGB LCD (ST7701S, 480x480, 16-bit bus) ---- */
#define BSP_PIN_LCD_VSYNC          17
#define BSP_PIN_LCD_HSYNC          16
#define BSP_PIN_LCD_DE             18
#define BSP_PIN_LCD_PCLK           21
#define BSP_PIN_LCD_BACKLIGHT      45  /* active high */

/* Data lines D0..D15 map to B0..B4, G0..G5, R0..R4 (blue on the low bits). */
#define BSP_PIN_LCD_D0             15  /* B0 */
#define BSP_PIN_LCD_D1             14  /* B1 */
#define BSP_PIN_LCD_D2             13  /* B2 */
#define BSP_PIN_LCD_D3             12  /* B3 */
#define BSP_PIN_LCD_D4             11  /* B4 */
#define BSP_PIN_LCD_D5             10  /* G0 */
#define BSP_PIN_LCD_D6              9  /* G1 */
#define BSP_PIN_LCD_D7              8  /* G2 */
#define BSP_PIN_LCD_D8              7  /* G3 */
#define BSP_PIN_LCD_D9              6  /* G4 */
#define BSP_PIN_LCD_D10             5  /* G5 */
#define BSP_PIN_LCD_D11             4  /* R0 */
#define BSP_PIN_LCD_D12             3  /* R1 */
#define BSP_PIN_LCD_D13             2  /* R2 */
#define BSP_PIN_LCD_D14             1  /* R3 */
#define BSP_PIN_LCD_D15             0  /* R4 */

/* ---- ST7701S register channel: bit-banged 9-bit 3-wire SPI (GX screen) ---- */
#define BSP_PIN_LCD_SPI_SCK        41
#define BSP_PIN_LCD_SPI_SDO        48  /* MOSI; the panel is write-only here */

/* ---- Board I2C (IO expander, touch, sensors) ---- */
#define BSP_PIN_I2C_SDA            39
#define BSP_PIN_I2C_SCL            40
#define BSP_I2C_FREQ_HZ        100000

/* ---- TCA9535 IO expander (vendor probes 0x20, falls back to 0x39) ---- */
#define BSP_I2C_ADDR_EXPANDER_PRIMARY   0x20
#define BSP_I2C_ADDR_EXPANDER_FALLBACK  0x39

/* Expander pin assignments (0-15). */
#define BSP_EXP_IO_LCD_CS           4  /* ST7701S SPI chip select */
#define BSP_EXP_IO_LCD_RESET        5  /* panel reset (held high by vendor) */
#define BSP_EXP_IO_TP_RESET         7  /* touch controller reset */
#define BSP_EXP_IO_RP2040_RESET     8  /* RP2040 co-processor reset */
#define BSP_EXP_IO_BMP_PWR         10  /* BMP3xx pressure sensor power */

/* ---- Touch (I2C, on the shared board bus) ----
 * Vendor probe order (indev_tp.c): FT6336U @0x48 (GX screen), @0x38,
 * then GT911 @0x5D or @0x14. Reset pulse on expander IO7: low 5 ms, high. */
#define BSP_I2C_ADDR_TOUCH_FT6x36   0x48
#define BSP_I2C_ADDR_TOUCH_FT6x36_ALT 0x38
#define BSP_I2C_ADDR_TOUCH_GT911    0x5D
#define BSP_I2C_ADDR_TOUCH_GT911_ALT 0x14

/* ---- Panel geometry / timing (GX screen, vendor Kconfig defaults) ---- */
#define BSP_LCD_H_RES             480
#define BSP_LCD_V_RES             480
#define BSP_LCD_PCLK_HZ     18000000   /* 18 MHz */
#define BSP_LCD_HSYNC_PULSE_WIDTH    8
#define BSP_LCD_HSYNC_BACK_PORCH    50
#define BSP_LCD_HSYNC_FRONT_PORCH   10
#define BSP_LCD_VSYNC_PULSE_WIDTH    8
#define BSP_LCD_VSYNC_BACK_PORCH    20
#define BSP_LCD_VSYNC_FRONT_PORCH   10
#define BSP_LCD_PCLK_ACTIVE_NEG      0

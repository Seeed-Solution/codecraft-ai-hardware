/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher display: a 412x412 round SPD2010 panel on SPI3 in QSPI
 * mode (40 MHz, 32-bit commands, 8-bit params, SPI mode 3, no D/C line), its
 * power rail on the IO expander and a PWM backlight on GPIO8.
 * Raw esp_lcd only — no LVGL; apps that want a UI toolkit layer it on top.
 */
#pragma once

#include "esp_err.h"
#include "esp_lcd_panel_ops.h"
#include "esp_lcd_panel_io.h"

#ifdef __cplusplus
extern "C" {
#endif

#define BSP_LCD_H_RES 412
#define BSP_LCD_V_RES 412

/* Powers the LCD rail, initializes the QSPI bus and the panel, turns the
 * display on and sets the backlight to 100%. Safe to call more than once;
 * later calls return the same handles. Either out handle may be NULL. */
esp_err_t bsp_display_init(esp_lcd_panel_handle_t *out_panel, esp_lcd_panel_io_handle_t *out_io);

/* 0-100. The display keeps its last frame with the backlight off. */
esp_err_t bsp_display_set_brightness(int percent);

#ifdef __cplusplus
}
#endif

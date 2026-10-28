/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * 1.47" 172x320 IPS LCD, JD9853A driven with the ST7789-compatible
 * esp_lcd driver plus Seeed's vendor register sequence (Seeed_GFX2, MIT).
 * Portrait: the panel is mirrored on X with a column gap of 34.
 */
#pragma once

#include <stdbool.h>

#include "esp_err.h"
#include "esp_lcd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Claim the SPI bus, reset the panel (the reset line is shared with the
 * touch controller and is pulsed once here), run the vendor init
 * sequence and turn the backlight on. `out_panel` receives the esp_lcd
 * panel handle for esp_lcd_panel_draw_bitmap(). */
esp_err_t bsp_display_init(esp_lcd_panel_handle_t *out_panel);

/* Backlight on/off (BSP_PIN_LCD_BL, active high). */
esp_err_t bsp_display_backlight(bool on);

#ifdef __cplusplus
}
#endif

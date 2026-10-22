/* SenseCAP Indicator display: ST7701S 480x480 RGB panel.
 * The panel's register channel is a bit-banged 9-bit 3-wire SPI (SCK/SDO on
 * GPIO, chip select on the TCA9535 expander); pixels stream over the 16-bit
 * RGB bus through esp_lcd. Framebuffer lives in octal PSRAM. */
#pragma once

#include "esp_err.h"
#include "esp_lcd_panel_ops.h"
#include "esp_lcd_panel_rgb.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Initializes the IO expander, releases the touch/RP2040 resets, creates the
 * esp_lcd RGB panel (480x480, RGB565, framebuffer in PSRAM) and runs the
 * ST7701S register init for the GX screen variant.
 * out_panel may be NULL. Safe to call once per boot. */
esp_err_t bsp_display_init(esp_lcd_panel_handle_t *out_panel);

/* Backlight on GPIO 45, active high. */
esp_err_t bsp_display_backlight_set(bool on);

bool bsp_display_backlight_get(void);

#ifdef __cplusplus
}
#endif

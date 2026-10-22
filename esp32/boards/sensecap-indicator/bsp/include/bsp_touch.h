/* SenseCAP Indicator capacitive touch (FT6336U or GT911, auto-probed).
 * Shares the board I2C bus with the TCA9535 expander; the controller
 * reset lives on expander pin IO7. All functions are idempotent-safe to
 * call after bsp_touch_init(). */
#pragma once

#include "esp_err.h"
#include "esp_lcd_touch.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Resets the touch controller via the IO expander, probes the vendor
 * address list (FT6336U @0x48/0x38, then GT911 @0x5D/0x14) and creates an
 * esp_lcd_touch handle. Returns ESP_ERR_NOT_FOUND when no known
 * controller answers. Safe to call more than once. */
esp_err_t bsp_touch_init(esp_lcd_touch_handle_t *out_handle);

/* Handle from a previous bsp_touch_init(), or NULL. */
esp_lcd_touch_handle_t bsp_touch_get_handle(void);

#ifdef __cplusplus
}
#endif

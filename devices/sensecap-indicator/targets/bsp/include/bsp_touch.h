/* SenseCAP Indicator capacitive touch (FT6336U or GT911, auto-probed).
 * Shares the board I2C bus with the TCA9535 expander; the controller
 * reset lives on expander pin IO7. */
#pragma once

#include "esp_err.h"
#include "esp_lcd_touch.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Task context only. Serialize startup before starting touch consumers. If
 * using the display, initialize it first because it also pulses touch reset.
 * Afterwards one touch-owning
 * task or application mutex must serialize native read_data/get_coordinates
 * cycles and any configuration through the borrowed handle. No ISR or
 * non-blocking callback calls; keep the separate LVGL lock for LVGL access.
 * Polling is configured; do not reconfigure shared reset/bus independently.
 *
 * Resets the touch controller via the IO expander, probes the vendor
 * address list (FT6336U @0x48/0x38, then GT911 @0x5D/0x14) and creates an
 * esp_lcd_touch handle. GX + FT6336U coordinates are rotated 180 degrees
 * about pixel 479; out-of-range samples are dropped. DX/GT911 are unchanged.
 * Source and revision limits: Device AGENTS.md, GX application observations.
 * Returns ESP_ERR_NOT_FOUND when no known controller answers. out_handle is optional and cleared on failure. A
 * successful repeat returns the same fully created handle without reset.
 * Failed constructor cleans its touch allocation; BSP releases owned IO and
 * retains the board bus/expander. Complete cleanup permits startup retry;
 * incomplete cleanup requires reset. Pinned native IO deletion can abort
 * on device-removal failure; graceful recovery from that SDK failure is not
 * promised. Reset/controller state after any error remains uncertain.
 *
 * Init includes the original reset/boot delays, up to four 50 ms probes and
 * native driver I2C calls. Native panel-IO transactions can wait indefinitely;
 * there is no hard total deadline. BSP owns touch/IO handles for the process
 * lifetime. Do not delete, independently reset or close them. Caller buffers
 * passed to native polling APIs belong to the caller through each call; honor
 * read errors instead of treating previously cached coordinates as fresh. */
esp_err_t bsp_touch_init(esp_lcd_touch_handle_t *out_handle);

/* Borrowed ready handle, or NULL; use under the same startup/owner contract. */
esp_lcd_touch_handle_t bsp_touch_get_handle(void);

#ifdef __cplusplus
}
#endif

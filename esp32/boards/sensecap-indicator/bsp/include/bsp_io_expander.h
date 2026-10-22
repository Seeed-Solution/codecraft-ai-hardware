/* SenseCAP Indicator TCA9535 IO expander.
 * Drives the ST7701S chip-select/reset, touch reset, RP2040 reset and the
 * BMP sensor power rail. All functions are idempotent-safe to call after
 * bsp_io_expander_init(). */
#pragma once

#include <stdbool.h>
#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Initializes the board I2C bus and probes the expander (0x20, then 0x39).
 * Safe to call more than once; later calls return ESP_OK immediately. */
esp_err_t bsp_io_expander_init(void);

/* pin: 0-15. direction: 1 = output, 0 = input. */
esp_err_t bsp_io_expander_set_direction(int pin, int output);

/* pin: 0-15, must be configured as output. */
esp_err_t bsp_io_expander_set_level(int pin, int level);

/* pin: 0-15. Reads the input register regardless of direction. */
esp_err_t bsp_io_expander_get_level(int pin, int *level);

#ifdef __cplusplus
}
#endif

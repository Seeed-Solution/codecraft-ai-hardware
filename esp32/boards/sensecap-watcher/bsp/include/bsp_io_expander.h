/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher PCA9535 16-bit IO expander (I2C0 @0x21, INT on GPIO2).
 * Carries the board's power rails, the camera sync/reset lines, card detect,
 * charge status, the knob's push and the touch interrupt.
 * All functions are idempotent-safe to call after bsp_io_expander_init().
 */
#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Initializes the board I2C0 bus and probes the expander at 0x21.
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

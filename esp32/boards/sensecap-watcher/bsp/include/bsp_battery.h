/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher battery: a 62k/20k divider from the battery onto ADC1
 * channel 2 (GPIO3), gated by the PWR_BAT_ADC rail on the IO expander, and a
 * battery-present detect line next to it.
 */
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Powers the divider rail and sets up ADC1 channel 2 with curve-fitting
 * calibration. Safe to call more than once. */
esp_err_t bsp_battery_init(void);

/* Battery terminal voltage in millivolts, or ESP_ERR_INVALID_STATE before
 * bsp_battery_init(). */
esp_err_t bsp_battery_get_voltage_mv(uint16_t *out_mv);

/* Expander BAT_DET line: true when a battery is present. */
bool bsp_battery_is_present(void);

#ifdef __cplusplus
}
#endif

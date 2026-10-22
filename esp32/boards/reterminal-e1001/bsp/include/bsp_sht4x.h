/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Onboard Sensirion SHT4x temperature/humidity sensor on I2C0
 * (address 0x44). Single-shot, high-precision measurements; each read
 * takes about 10 ms. The enclosure warms the sensor slightly — apply a
 * calibration offset in the application if absolute temperature matters.
 */
#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Claim I2C0 and probe for the sensor. Returns ESP_ERR_NOT_FOUND when the
 * sensor does not answer. */
esp_err_t bsp_sht4x_init(void);

/* One measurement. Blocks ~10 ms. CRC-checked; returns ESP_ERR_INVALID_CRC
 * on a corrupt frame. */
esp_err_t bsp_sht4x_read(float *temp_c, float *rh_pct);

#ifdef __cplusplus
}
#endif

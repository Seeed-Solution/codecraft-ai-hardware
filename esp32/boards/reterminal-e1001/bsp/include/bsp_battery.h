/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Battery voltage sense: divider on GPIO 1 (ADC1 channel 0), enabled by
 * driving GPIO 21 high. Divider is 10k/10k, so VBAT = 2x the ADC voltage
 * (Zephyr boards/seeed/reterminal_e1001 dts voltage-divider node,
 * Apache 2.0). Battery: 2000 mAh LiPo charged by a SY6974B on I2C1.
 */
#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

esp_err_t bsp_battery_init(void);

/* Enable the divider, settle 10 ms, sample, disable. Returns the battery
 * voltage in millivolts. */
esp_err_t bsp_battery_read_mv(int *mv);

#ifdef __cplusplus
}
#endif

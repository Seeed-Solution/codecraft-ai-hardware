/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Green status LED on GPIO 6, active low
 * (Zephyr boards/seeed/reterminal_e1001 dts, Apache 2.0).
 */
#pragma once

#include <stdbool.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

esp_err_t bsp_led_init(void);
void bsp_led_set(bool on);

#ifdef __cplusplus
}
#endif

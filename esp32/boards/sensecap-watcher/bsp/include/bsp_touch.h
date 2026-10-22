/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher touch: the SPD2010 panel's integrated capacitive
 * controller on the dedicated I2C1 bus (SDA GPIO39 / SCL GPIO38, 400 kHz,
 * addr 0x53). Its interrupt line is not a real GPIO — it is pin 5 of the
 * PCA9535 IO expander, so the driver is created without an interrupt GPIO
 * and apps either poll or watch the expander (bsp_io_expander).
 * Coordinates map 1:1 to the 412x412 display, no swap/mirror.
 */
#pragma once

#include "esp_err.h"
#include "esp_lcd_touch.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Brings up the I2C1 bus and probes the touch controller. Safe to call more
 * than once; later calls return the same handle. Also configures expander
 * pin 5 as an input so the touch interrupt reaches the expander's INT line
 * (GPIO2). Either out handle may be NULL. */
esp_err_t bsp_touch_init(esp_lcd_touch_handle_t *out_handle);

/* Returns the touch handle from a previous bsp_touch_init(), or NULL. */
esp_lcd_touch_handle_t bsp_touch_get_handle(void);

#ifdef __cplusplus
}
#endif

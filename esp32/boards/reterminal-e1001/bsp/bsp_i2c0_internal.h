/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Shared I2C0 master bus. The SHT4x sensor (0x44) and the PCF8563 RTC
 * (0x51) share I2C0 (SDA 19 / SCL 20), so their drivers must attach to
 * one bus instance instead of each claiming a controller. Component-local;
 * not installed into the public include directory.
 */
#pragma once

#include "driver/i2c_master.h"
#include "esp_err.h"

/* Returns the shared I2C0 master bus, creating it on first call. Call from
 * driver init functions before starting concurrent tasks; the bus is never
 * deleted. */
esp_err_t bsp_i2c0_bus_get(i2c_master_bus_handle_t *out_bus);

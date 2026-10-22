/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 */
#include "bsp_i2c0_internal.h"
#include "bsp_pins.h"

static i2c_master_bus_handle_t s_i2c0_bus;

esp_err_t bsp_i2c0_bus_get(i2c_master_bus_handle_t *out_bus)
{
    if (s_i2c0_bus == NULL) {
        i2c_master_bus_config_t bus_cfg = {
            .i2c_port = 0,
            .sda_io_num = BSP_PIN_I2C0_SDA,
            .scl_io_num = BSP_PIN_I2C0_SCL,
            .clk_source = I2C_CLK_SRC_DEFAULT,
            .glitch_ignore_cnt = 7,
            .flags.enable_internal_pullup = true,
        };
        esp_err_t err = i2c_new_master_bus(&bus_cfg, &s_i2c0_bus);
        if (err != ESP_OK) {
            return err;
        }
    }
    *out_bus = s_i2c0_bus;
    return ESP_OK;
}

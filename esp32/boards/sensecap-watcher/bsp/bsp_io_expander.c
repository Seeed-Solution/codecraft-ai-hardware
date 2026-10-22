/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * PCA9535 16-bit IO expander on the board I2C0 bus.
 * Register map per TI PCA9535 datasheet (SCPS204): 0/1 input, 2/3 output,
 * 4/5 polarity inversion, 6/7 configuration (1 = input) — the same layout
 * as the TCA9535. Address fact: vendor BSP sensecap-watcher.c probes
 * ESP_IO_EXPANDER_I2C_PCA9535_ADDRESS_001 = 0x21 (A2A1A0 = 001).
 */
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "driver/i2c_master.h"
#include "esp_check.h"

#include <stdbool.h>

static const char *TAG = "bsp_io_expander";

#define BSP_I2C_ADDR_EXPANDER 0x21

static i2c_master_bus_handle_t s_bus;
static i2c_master_dev_handle_t s_dev;

/* Shadow copies of the output (reg 2/3) and config (reg 6/7) registers so
 * single-pin updates never clobber the other pins on the same port. */
static uint8_t s_output[2];
static uint8_t s_config[2];
static bool s_shadow_valid;

static esp_err_t exp_write_reg(uint8_t reg, uint8_t val)
{
    uint8_t buf[2] = {reg, val};
    return i2c_master_transmit(s_dev, buf, sizeof(buf), 100);
}

static esp_err_t exp_read_reg(uint8_t reg, uint8_t *val)
{
    return i2c_master_transmit_receive(s_dev, &reg, 1, val, 1, 100);
}

esp_err_t bsp_io_expander_init(void)
{
    if (s_dev) {
        return ESP_OK;
    }
    if (!s_bus) {
        i2c_master_bus_config_t bus_cfg = {
            .i2c_port = -1,
            .sda_io_num = BSP_PIN_I2C_SDA,
            .scl_io_num = BSP_PIN_I2C_SCL,
            .clk_source = I2C_CLK_SRC_DEFAULT,
            .glitch_ignore_cnt = 7,
            .flags.enable_internal_pullup = true,
        };
        ESP_RETURN_ON_ERROR(i2c_new_master_bus(&bus_cfg, &s_bus), TAG, "bus");
    }
    i2c_device_config_t dev_cfg = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = BSP_I2C_ADDR_EXPANDER,
        .scl_speed_hz = BSP_I2C_FREQ_HZ,
    };
    ESP_RETURN_ON_ERROR(i2c_master_bus_add_device(s_bus, &dev_cfg, &s_dev), TAG,
                        "add device 0x%02x", BSP_I2C_ADDR_EXPANDER);
    ESP_RETURN_ON_ERROR(i2c_master_probe(s_bus, BSP_I2C_ADDR_EXPANDER, 100), TAG,
                        "PCA9535 not found at 0x%02x", BSP_I2C_ADDR_EXPANDER);

    /* Sync the shadows with the real chip state. */
    ESP_RETURN_ON_ERROR(exp_read_reg(2, &s_output[0]), TAG, "read output0");
    ESP_RETURN_ON_ERROR(exp_read_reg(3, &s_output[1]), TAG, "read output1");
    ESP_RETURN_ON_ERROR(exp_read_reg(6, &s_config[0]), TAG, "read config0");
    ESP_RETURN_ON_ERROR(exp_read_reg(7, &s_config[1]), TAG, "read config1");
    s_shadow_valid = true;
    return ESP_OK;
}

esp_err_t bsp_i2c0_get_bus(i2c_master_bus_handle_t *out_bus)
{
    ESP_RETURN_ON_FALSE(out_bus != NULL, ESP_ERR_INVALID_ARG, TAG, "out_bus");
    /* The audio codecs (ES8311 DAC @0x30, ES7243 ADC @0x13/0x14) share the
     * board I2C0 bus with this expander. */
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "init");
    *out_bus = s_bus;
    return ESP_OK;
}

esp_err_t bsp_io_expander_set_direction(int pin, int output)
{
    ESP_RETURN_ON_FALSE(s_dev && s_shadow_valid, ESP_ERR_INVALID_STATE, TAG,
                        "not initialized");
    ESP_RETURN_ON_FALSE(pin >= 0 && pin < 16, ESP_ERR_INVALID_ARG, TAG, "pin");
    uint8_t mask = 1u << (pin & 7);
    int port = pin >> 3;
    if (output) {
        s_config[port] &= ~mask;
    } else {
        s_config[port] |= mask;
    }
    return exp_write_reg(6 + port, s_config[port]);
}

esp_err_t bsp_io_expander_set_level(int pin, int level)
{
    ESP_RETURN_ON_FALSE(s_dev && s_shadow_valid, ESP_ERR_INVALID_STATE, TAG,
                        "not initialized");
    ESP_RETURN_ON_FALSE(pin >= 0 && pin < 16, ESP_ERR_INVALID_ARG, TAG, "pin");
    uint8_t mask = 1u << (pin & 7);
    int port = pin >> 3;
    if (level) {
        s_output[port] |= mask;
    } else {
        s_output[port] &= ~mask;
    }
    return exp_write_reg(2 + port, s_output[port]);
}

esp_err_t bsp_io_expander_get_level(int pin, int *level)
{
    ESP_RETURN_ON_FALSE(s_dev, ESP_ERR_INVALID_STATE, TAG, "not initialized");
    ESP_RETURN_ON_FALSE(pin >= 0 && pin < 16 && level, ESP_ERR_INVALID_ARG,
                        TAG, "pin");
    uint8_t val;
    ESP_RETURN_ON_ERROR(exp_read_reg(pin >> 3, &val), TAG, "read input");
    *level = (val >> (pin & 7)) & 1;
    return ESP_OK;
}

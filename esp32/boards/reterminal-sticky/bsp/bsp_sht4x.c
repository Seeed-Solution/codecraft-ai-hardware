/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * Modified 2026 by CodeCraft AI Hardware contributors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Derived from muse-gadget-sdk esp32/main/reterminal_sht4x.c (Apache 2.0):
 * protocol, CRC and decoding kept; the polling task and the cJSON command
 * plumbing are dropped in favour of a plain single-shot API. The Sticky
 * wires the SHT4x to I2C1 (SDA=1, SCL=0) instead of the E100x's I2C0.
 */
#include "bsp_sht4x.h"
#include "bsp_pins.h"

#include <stdbool.h>
#include <stdint.h>

#include "driver/i2c_master.h"
#include "esp_check.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

/* High-precision single-shot: send the measure command, wait ~10 ms,
 * read back 6 bytes (temperature MSB/LSB/CRC, humidity MSB/LSB/CRC). */
#define SHT4X_ADDR            0x44
#define SHT4X_CMD_MEASURE_HPM 0xFD
#define SHT4X_MEASURE_MS      10

static i2c_master_bus_handle_t s_bus;
static i2c_master_dev_handle_t s_dev;

/* CRC-8 of each 2-byte word: polynomial 0x31, initialized to 0xFF. */
static uint8_t sht4x_crc(const uint8_t *data, size_t len)
{
    uint8_t crc = 0xFF;
    for (size_t i = 0; i < len; i++) {
        crc ^= data[i];
        for (int bit = 0; bit < 8; bit++) {
            crc = (crc & 0x80) ? (uint8_t)((crc << 1) ^ 0x31) : (uint8_t)(crc << 1);
        }
    }
    return crc;
}

esp_err_t bsp_sht4x_init(void)
{
    const i2c_master_bus_config_t bus_cfg = {
        .i2c_port = I2C_NUM_1,
        .sda_io_num = BSP_PIN_I2C1_SDA,
        .scl_io_num = BSP_PIN_I2C1_SCL,
        .clk_source = I2C_CLK_SRC_DEFAULT,
        .glitch_ignore_cnt = 7,
        .flags.enable_internal_pullup = true,
    };
    ESP_RETURN_ON_ERROR(i2c_new_master_bus(&bus_cfg, &s_bus), "bsp_sht4x", "bus");
    ESP_RETURN_ON_ERROR(i2c_master_probe(s_bus, SHT4X_ADDR, 100), "bsp_sht4x", "probe");
    const i2c_device_config_t dev_cfg = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = SHT4X_ADDR,
        .scl_speed_hz = 100000,
    };
    return i2c_master_bus_add_device(s_bus, &dev_cfg, &s_dev);
}

esp_err_t bsp_sht4x_read(float *temp_c, float *rh_pct)
{
    if (!temp_c || !rh_pct) return ESP_ERR_INVALID_ARG;
    uint8_t cmd = SHT4X_CMD_MEASURE_HPM;
    ESP_RETURN_ON_ERROR(i2c_master_transmit(s_dev, &cmd, 1, 100), "bsp_sht4x", "cmd");
    vTaskDelay(pdMS_TO_TICKS(SHT4X_MEASURE_MS));
    uint8_t buf[6];
    ESP_RETURN_ON_ERROR(i2c_master_receive(s_dev, buf, sizeof(buf), 100), "bsp_sht4x", "read");
    if (sht4x_crc(buf, 2) != buf[2] || sht4x_crc(buf + 3, 2) != buf[5]) {
        return ESP_ERR_INVALID_CRC;
    }
    uint16_t t_raw = ((uint16_t)buf[0] << 8) | buf[1];
    uint16_t rh_raw = ((uint16_t)buf[3] << 8) | buf[4];
    *temp_c = -45.0f + 175.0f * (float)t_raw / 65535.0f;
    *rh_pct = -6.0f + 125.0f * (float)rh_raw / 65535.0f;
    if (*rh_pct < 0.0f) *rh_pct = 0.0f;
    if (*rh_pct > 100.0f) *rh_pct = 100.0f;
    return ESP_OK;
}

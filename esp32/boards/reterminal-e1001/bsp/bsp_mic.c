/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Written for this BSP. Pin facts (CLK 42 / DATA 41 / EN 38) come from the
 * reTerminal E1001 V1.2 vendor schematic (CC BY-SA 4.0, see bsp_pins.h).
 * Build-verified only; mic gain/clocking need on-device validation.
 */
#include "bsp_mic.h"
#include "bsp_pins.h"

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/gpio.h"
#include "driver/i2s_pdm.h"
#include "esp_check.h"

static i2s_chan_handle_t s_rx_chan;

esp_err_t bsp_mic_init(uint32_t sample_rate_hz)
{
    /* Power the microphone rail first so the PDM clock has a live mic. */
    gpio_config_t en_cfg = {
        .pin_bit_mask = 1ULL << BSP_PIN_PDM_EN,
        .mode = GPIO_MODE_OUTPUT,
        .pull_up_en = GPIO_PULLUP_DISABLE,
        .pull_down_en = GPIO_PULLDOWN_DISABLE,
        .intr_type = GPIO_INTR_DISABLE,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&en_cfg), "bsp_mic", "mic EN config failed");
    ESP_RETURN_ON_ERROR(gpio_set_level(BSP_PIN_PDM_EN, 1), "bsp_mic", "mic EN high failed");

    i2s_chan_config_t chan_cfg = I2S_CHANNEL_DEFAULT_CONFIG(I2S_NUM_0, I2S_ROLE_MASTER);
    ESP_RETURN_ON_ERROR(i2s_new_channel(&chan_cfg, NULL, &s_rx_chan), "bsp_mic",
                        "i2s channel alloc failed");

    i2s_pdm_rx_config_t pdm_cfg = {
        .clk_cfg = I2S_PDM_RX_CLK_DEFAULT_CONFIG(sample_rate_hz),
        .slot_cfg = I2S_PDM_RX_SLOT_DEFAULT_CONFIG(I2S_DATA_BIT_WIDTH_16BIT,
                                                   I2S_SLOT_MODE_MONO),
        .gpio_cfg = {
            .clk = BSP_PIN_PDM_CLK,
            .din = BSP_PIN_PDM_DATA,
            .invert_flags = { .clk_inv = false },
        },
    };
    ESP_RETURN_ON_ERROR(i2s_channel_init_pdm_rx_mode(s_rx_chan, &pdm_cfg), "bsp_mic",
                        "pdm rx init failed");
    return i2s_channel_enable(s_rx_chan);
}

esp_err_t bsp_mic_read(int16_t *samples, size_t sample_count, size_t *out_read,
                       uint32_t timeout_ms)
{
    size_t bytes_read = 0;
    esp_err_t err = i2s_channel_read(s_rx_chan, samples, sample_count * sizeof(int16_t),
                                     &bytes_read, pdMS_TO_TICKS(timeout_ms));
    if (err != ESP_OK) {
        return err;
    }
    *out_read = bytes_read / sizeof(int16_t);
    return ESP_OK;
}

esp_err_t bsp_mic_deinit(void)
{
    esp_err_t err = i2s_channel_disable(s_rx_chan);
    if (err != ESP_OK) {
        return err;
    }
    err = i2s_del_channel(s_rx_chan);
    if (err != ESP_OK) {
        return err;
    }
    s_rx_chan = NULL;
    return gpio_set_level(BSP_PIN_PDM_EN, 0);
}

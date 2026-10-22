/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Knob facts: vendor BSP header (Apache 2.0) — quadrature on GPIO41/42, push
 * on expander pin 3, low while pressed (muse board_sensecap_watcher.c calls
 * the same bit EXP_WHEEL, "low while pressed"). The decoder follows the
 * ESP-IDF rotary-encoder PCNT pattern: each channel watches one phase's
 * edges against the other phase's level.
 */
#include "bsp_knob.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "driver/pulse_cnt.h"
#include "esp_check.h"

static const char *TAG = "bsp_knob";

static pcnt_unit_handle_t s_unit;
static bool s_ready;

esp_err_t bsp_knob_init(void)
{
    if (s_ready) {
        return ESP_OK;
    }
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_KNOB_BTN, 0), TAG, "button dir");

    const pcnt_unit_config_t unit_cfg = {
        .low_limit = -4096,
        .high_limit = 4096,
    };
    ESP_RETURN_ON_ERROR(pcnt_new_unit(&unit_cfg, &s_unit), TAG, "pcnt unit");

    const pcnt_glitch_filter_config_t filter_cfg = {
        .max_glitch_ns = 1000,
    };
    ESP_RETURN_ON_ERROR(pcnt_unit_set_glitch_filter(s_unit, &filter_cfg), TAG, "glitch filter");

    pcnt_chan_config_t chan_cfg = {
        .edge_gpio_num = BSP_PIN_KNOB_A,
        .level_gpio_num = BSP_PIN_KNOB_B,
    };
    pcnt_channel_handle_t chan_a;
    ESP_RETURN_ON_ERROR(pcnt_new_channel(s_unit, &chan_cfg, &chan_a), TAG, "channel A");
    chan_cfg.edge_gpio_num = BSP_PIN_KNOB_B;
    chan_cfg.level_gpio_num = BSP_PIN_KNOB_A;
    pcnt_channel_handle_t chan_b;
    ESP_RETURN_ON_ERROR(pcnt_new_channel(s_unit, &chan_cfg, &chan_b), TAG, "channel B");

    ESP_RETURN_ON_ERROR(pcnt_channel_set_edge_action(chan_a, PCNT_CHANNEL_EDGE_ACTION_DECREASE,
                                                     PCNT_CHANNEL_EDGE_ACTION_INCREASE),
                        TAG, "A edges");
    ESP_RETURN_ON_ERROR(pcnt_channel_set_level_action(chan_a, PCNT_CHANNEL_LEVEL_ACTION_KEEP,
                                                      PCNT_CHANNEL_LEVEL_ACTION_INVERSE),
                        TAG, "A levels");
    ESP_RETURN_ON_ERROR(pcnt_channel_set_edge_action(chan_b, PCNT_CHANNEL_EDGE_ACTION_INCREASE,
                                                     PCNT_CHANNEL_EDGE_ACTION_DECREASE),
                        TAG, "B edges");
    ESP_RETURN_ON_ERROR(pcnt_channel_set_level_action(chan_b, PCNT_CHANNEL_LEVEL_ACTION_KEEP,
                                                      PCNT_CHANNEL_LEVEL_ACTION_INVERSE),
                        TAG, "B levels");

    ESP_RETURN_ON_ERROR(pcnt_unit_enable(s_unit), TAG, "enable");
    ESP_RETURN_ON_ERROR(pcnt_unit_clear_count(s_unit), TAG, "clear");
    ESP_RETURN_ON_ERROR(pcnt_unit_start(s_unit), TAG, "start");
    s_ready = true;
    return ESP_OK;
}

esp_err_t bsp_knob_get_delta(int *out_delta)
{
    ESP_RETURN_ON_FALSE(s_ready, ESP_ERR_INVALID_STATE, TAG, "not initialized");
    ESP_RETURN_ON_FALSE(out_delta, ESP_ERR_INVALID_ARG, TAG, "out_delta");
    int count;
    ESP_RETURN_ON_ERROR(pcnt_unit_get_count(s_unit, &count), TAG, "count");
    ESP_RETURN_ON_ERROR(pcnt_unit_clear_count(s_unit), TAG, "clear");
    *out_delta = count;
    return ESP_OK;
}

bool bsp_knob_button_pressed(void)
{
    int level = 1;
    return bsp_io_expander_get_level(BSP_EXP_PIN_KNOB_BTN, &level) == ESP_OK && !level;
}

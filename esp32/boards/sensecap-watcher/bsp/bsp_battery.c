/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Battery facts: vendor BSP sensecap-watcher.c (Apache 2.0) — ADC1 channel 2
 * (GPIO3), ADC_ATTEN_DB_2_5 (0-1100 mV at the pin), divider (62k+20k)/20k,
 * curve-fitting calibration, mV at the battery = mV at the pin * 82 / 20.
 * The divider rail (PWR_BAT_ADC) is gated so measurement doesn't drain the
 * battery; here the driver owns it instead of relying on boot state.
 */
#include "bsp_battery.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "esp_check.h"
#include "esp_adc/adc_cali.h"
#include "esp_adc/adc_cali_scheme.h"
#include "esp_adc/adc_oneshot.h"

static const char *TAG = "bsp_battery";

static adc_oneshot_unit_handle_t s_adc;
static adc_cali_handle_t s_cali;
static bool s_ready;

esp_err_t bsp_battery_init(void)
{
    if (s_ready) {
        return ESP_OK;
    }
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_PWR_BAT_ADC, 1), TAG, "rail dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_PIN_PWR_BAT_ADC, 1), TAG, "rail on");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_BAT_DET, 0), TAG, "det dir");

    const adc_oneshot_unit_init_cfg_t unit_cfg = {
        .unit_id = ADC_UNIT_1,
    };
    ESP_RETURN_ON_ERROR(adc_oneshot_new_unit(&unit_cfg, &s_adc), TAG, "adc unit");
    const adc_oneshot_chan_cfg_t chan_cfg = {
        .bitwidth = ADC_BITWIDTH_DEFAULT,
        .atten = ADC_ATTEN_DB_2_5,
    };
    ESP_RETURN_ON_ERROR(adc_oneshot_config_channel(s_adc, BSP_ADC_CHANNEL_BATTERY, &chan_cfg),
                        TAG, "adc channel");

    const adc_cali_curve_fitting_config_t cali_cfg = {
        .unit_id = ADC_UNIT_1,
        .chan = BSP_ADC_CHANNEL_BATTERY,
        .atten = ADC_ATTEN_DB_2_5,
        .bitwidth = ADC_BITWIDTH_DEFAULT,
    };
    ESP_RETURN_ON_ERROR(adc_cali_create_scheme_curve_fitting(&cali_cfg, &s_cali), TAG, "calibration");
    s_ready = true;
    return ESP_OK;
}

esp_err_t bsp_battery_get_voltage_mv(uint16_t *out_mv)
{
    ESP_RETURN_ON_FALSE(s_ready, ESP_ERR_INVALID_STATE, TAG, "not initialized");
    ESP_RETURN_ON_FALSE(out_mv, ESP_ERR_INVALID_ARG, TAG, "out_mv");
    int raw, mv;
    ESP_RETURN_ON_ERROR(adc_oneshot_read(s_adc, BSP_ADC_CHANNEL_BATTERY, &raw), TAG, "read");
    ESP_RETURN_ON_ERROR(adc_cali_raw_to_voltage(s_cali, raw, &mv), TAG, "calibrate");
    *out_mv = (uint16_t)(mv * 82 / 20);
    return ESP_OK;
}

bool bsp_battery_is_present(void)
{
    int level = 0;
    return bsp_io_expander_get_level(BSP_EXP_PIN_BAT_DET, &level) == ESP_OK && level;
}

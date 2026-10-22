/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 */
#include "bsp_battery.h"
#include "bsp_pins.h"

#include "driver/gpio.h"
#include "esp_adc/adc_oneshot.h"
#include "esp_adc/adc_cali.h"
#include "esp_adc/adc_cali_scheme.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#define VBAT_DIVIDER_NUM 2  /* VBAT = 2 x ADC voltage (10k over 10k) */
#define VBAT_SETTLE_MS   10 /* Zephyr power-on-sample-delay-us = 10000 */

static adc_oneshot_unit_handle_t s_adc;
static adc_cali_handle_t s_cali;
static bool s_cali_ok;

esp_err_t bsp_battery_init(void)
{
    const gpio_config_t en = {
        .pin_bit_mask = 1ULL << BSP_PIN_VBAT_EN,
        .mode = GPIO_MODE_OUTPUT,
    };
    esp_err_t err = gpio_config(&en);
    if (err != ESP_OK) return err;
    gpio_set_level(BSP_PIN_VBAT_EN, 0);

    adc_oneshot_unit_init_cfg_t unit = {
        .unit_id = ADC_UNIT_1,
        .ulp_mode = ADC_ULP_MODE_DISABLE,
    };
    err = adc_oneshot_new_unit(&unit, &s_adc);
    if (err != ESP_OK) return err;
    adc_oneshot_chan_cfg_t chan = {
        .bitwidth = ADC_BITWIDTH_12,
        .atten = ADC_ATTEN_DB_12,
    };
    err = adc_oneshot_config_channel(s_adc, ADC_CHANNEL_0, &chan);
    if (err != ESP_OK) return err;
    adc_cali_curve_fitting_config_t cali = {
        .unit_id = ADC_UNIT_1,
        .chan = ADC_CHANNEL_0,
        .atten = ADC_ATTEN_DB_12,
        .bitwidth = ADC_BITWIDTH_12,
    };
    s_cali_ok = adc_cali_create_scheme_curve_fitting(&cali, &s_cali) == ESP_OK;
    return ESP_OK;
}

esp_err_t bsp_battery_read_mv(int *mv)
{
    gpio_set_level(BSP_PIN_VBAT_EN, 1);
    vTaskDelay(pdMS_TO_TICKS(VBAT_SETTLE_MS));
    int raw = 0;
    esp_err_t err = adc_oneshot_read(s_adc, ADC_CHANNEL_0, &raw);
    gpio_set_level(BSP_PIN_VBAT_EN, 0);
    if (err != ESP_OK) return err;
    int sensed = raw;
    if (s_cali_ok) {
        err = adc_cali_raw_to_voltage(s_cali, raw, &sensed);
        if (err != ESP_OK) return err;
    } else {
        /* Uncalibrated fallback: 12-bit, 12 dB attenuation ~3.1 V full scale. */
        sensed = raw * 3100 / 4095;
    }
    *mv = sensed * VBAT_DIVIDER_NUM;
    return ESP_OK;
}

/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * USR1/USR2 keys, active low with internal pull-ups. Polled with debounce
 * in a dedicated task; callbacks must stay non-blocking.
 */
#include "bsp_button.h"
#include "bsp_pins.h"

#include "driver/gpio.h"
#include "esp_check.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#define BSP_BUTTON_DEBOUNCE_MS 30
#define BSP_BUTTON_POLL_MS     20

static const int s_gpios[BSP_BUTTON_COUNT] = {
    [BSP_BUTTON_USR1] = BSP_PIN_BUTTON_USR1,
    [BSP_BUTTON_USR2] = BSP_PIN_BUTTON_USR2,
};

static bsp_button_cb_t s_cb;
static void *s_cb_arg;
static volatile bool s_pressed[BSP_BUTTON_COUNT];

static void bsp_button_task(void *arg)
{
    (void)arg;
    bool stable[BSP_BUTTON_COUNT] = {false};
    bool raw_last[BSP_BUTTON_COUNT] = {false};
    uint32_t stable_ms[BSP_BUTTON_COUNT] = {0};
    for (;;) {
        uint32_t now = xTaskGetTickCount() * portTICK_PERIOD_MS;
        for (int i = 0; i < BSP_BUTTON_COUNT; i++) {
            bool raw = gpio_get_level(s_gpios[i]) == 0;
            if (raw != raw_last[i]) { raw_last[i] = raw; stable_ms[i] = now; }
            if (raw != stable[i] && now - stable_ms[i] >= BSP_BUTTON_DEBOUNCE_MS) {
                stable[i] = raw;
                s_pressed[i] = stable[i];
                if (s_cb) s_cb((bsp_button_id_t)i, stable[i], s_cb_arg);
            }
        }
        vTaskDelay(pdMS_TO_TICKS(BSP_BUTTON_POLL_MS));
    }
}

esp_err_t bsp_button_init(bsp_button_cb_t cb, void *arg)
{
    gpio_config_t cfg = {
        .pin_bit_mask = (1ULL << BSP_PIN_BUTTON_USR1) | (1ULL << BSP_PIN_BUTTON_USR2),
        .mode = GPIO_MODE_INPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&cfg), "bsp_button", "gpio");
    s_cb = cb;
    s_cb_arg = arg;
    ESP_RETURN_ON_FALSE(
        xTaskCreate(bsp_button_task, "bsp_button", 2048, NULL, 5, NULL) == pdPASS,
        ESP_ERR_NO_MEM, "bsp_button", "task");
    return ESP_OK;
}

bool bsp_button_pressed(bsp_button_id_t id)
{
    if (id < 0 || id >= BSP_BUTTON_COUNT) return false;
    return s_pressed[id];
}

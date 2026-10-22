/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Key facts: Zephyr boards/seeed/reterminal_e1001 dts (Apache 2.0) —
 * GPIO 3/4/5, pull-up, active low; muse-gadget-sdk sdkconfig.reterminal-e1001
 * (Apache 2.0) — GPIO 3 is the green key.
 */
#include "bsp_button.h"
#include "bsp_pins.h"

#include "driver/gpio.h"
#include "esp_check.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#define BSP_BUTTON_DEBOUNCE_MS 30
#define BSP_BUTTON_POLL_MS     20

static const int s_key_gpio[BSP_KEY_COUNT] = {
    [BSP_KEY_GREEN] = BSP_PIN_KEY_GREEN,
    [BSP_KEY_LEFT]  = BSP_PIN_KEY_LEFT,
    [BSP_KEY_RIGHT] = BSP_PIN_KEY_RIGHT,
};

static bsp_button_cb_t s_cb;
static void *s_cb_arg;
static volatile bool s_pressed[BSP_KEY_COUNT];

static void bsp_button_task(void *arg)
{
    bool stable[BSP_KEY_COUNT] = {false};
    bool raw_last[BSP_KEY_COUNT] = {false};
    uint32_t stable_ms[BSP_KEY_COUNT] = {0};
    for (;;) {
        uint32_t now = xTaskGetTickCount() * portTICK_PERIOD_MS;
        for (int k = 0; k < BSP_KEY_COUNT; k++) {
            bool raw = gpio_get_level(s_key_gpio[k]) == 0;
            if (raw != raw_last[k]) { raw_last[k] = raw; stable_ms[k] = now; }
            if (raw != stable[k] && now - stable_ms[k] >= BSP_BUTTON_DEBOUNCE_MS) {
                stable[k] = raw;
                s_pressed[k] = stable[k];
                if (s_cb) s_cb((bsp_key_t)k, stable[k], s_cb_arg);
            }
        }
        vTaskDelay(pdMS_TO_TICKS(BSP_BUTTON_POLL_MS));
    }
}

esp_err_t bsp_button_init(bsp_button_cb_t cb, void *arg)
{
    uint64_t mask = 0;
    for (int k = 0; k < BSP_KEY_COUNT; k++) mask |= 1ULL << s_key_gpio[k];
    gpio_config_t cfg = {
        .pin_bit_mask = mask,
        .mode = GPIO_MODE_INPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&cfg), "bsp_button", "gpio");
    s_cb = cb;
    s_cb_arg = arg;
    ESP_RETURN_ON_FALSE(
        xTaskCreate(bsp_button_task, "bsp_button", 3072, NULL, 5, NULL) == pdPASS,
        ESP_ERR_NO_MEM, "bsp_button", "task");
    return ESP_OK;
}

bool bsp_button_pressed(bsp_key_t key)
{
    if (key >= BSP_KEY_COUNT) return false;
    return s_pressed[key];
}

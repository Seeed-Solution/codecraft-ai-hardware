/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher touch example: polls the SPD2010 touch controller and
 * logs press coordinates. Touch the round screen to see output.
 */

#include "bsp_touch.h"

#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "touch_read";

void app_main(void)
{
    esp_lcd_touch_handle_t touch = NULL;
    ESP_ERROR_CHECK(bsp_touch_init(&touch));

    ESP_LOGI(TAG, "touch the screen");
    bool was_pressed = false;
    while (true) {
        esp_lcd_touch_read_data(touch);
        uint16_t x[1] = {0}, y[1] = {0}, strength[1] = {0};
        uint8_t count = 0;
        bool pressed = esp_lcd_touch_get_coordinates(touch, x, y, strength, &count, 1);
        if (pressed && count > 0) {
            ESP_LOGI(TAG, "press x=%u y=%u strength=%u points=%u",
                     x[0], y[0], strength[0], count);
        } else if (was_pressed) {
            ESP_LOGI(TAG, "released");
        }
        was_pressed = pressed && count > 0;
        vTaskDelay(pdMS_TO_TICKS(20));
    }
}

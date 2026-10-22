/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Prints knob turns (PCNT quadrature counts since the last poll) and knob
 * pushes (expander pin 3, active low) as they happen.
 */
#include "bsp_knob.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "knob_input";

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_knob_init());
    bool was_pressed = bsp_knob_button_pressed();
    for (;;) {
        int delta;
        ESP_ERROR_CHECK(bsp_knob_get_delta(&delta));
        if (delta) {
            ESP_LOGI(TAG, "knob %+d counts", delta);
        }
        bool pressed = bsp_knob_button_pressed();
        if (pressed != was_pressed) {
            ESP_LOGI(TAG, "knob %s", pressed ? "pressed" : "released");
            was_pressed = pressed;
        }
        vTaskDelay(pdMS_TO_TICKS(50));
    }
}

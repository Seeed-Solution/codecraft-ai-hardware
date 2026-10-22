/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Reports the battery every second: presence (expander BAT_DET) and terminal
 * voltage (ADC1 channel 2 through the gated 62k/20k divider).
 */
#include <inttypes.h>

#include "bsp_battery.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "battery_status";

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_battery_init());
    for (;;) {
        uint16_t mv;
        ESP_ERROR_CHECK(bsp_battery_get_voltage_mv(&mv));
        ESP_LOGI(TAG, "battery %s, %" PRIu16 " mV",
                 bsp_battery_is_present() ? "present" : "absent", mv);
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}

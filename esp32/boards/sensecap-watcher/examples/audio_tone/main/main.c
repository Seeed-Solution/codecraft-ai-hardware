/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher speaker example: plays a 1 kHz sine for two seconds,
 * then fades the volume down and back up.
 */

#include "bsp_audio.h"

#include <math.h>

#include "esp_log.h"
#include "esp_codec_dev.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "audio_tone";

#define TONE_HZ    1000
#define CHUNK      256 /* samples per write */

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_audio_init());
    esp_codec_dev_handle_t spk = bsp_audio_get_speaker();
    ESP_ERROR_CHECK(bsp_audio_set_volume(60));

    int16_t buf[CHUNK];
    const float w = 2.0f * (float)M_PI * TONE_HZ / BSP_AUDIO_SAMPLE_RATE;
    for (int i = 0; i < CHUNK; i++) {
        buf[i] = (int16_t)(sinf(w * i) * 12000);
    }

    ESP_LOGI(TAG, "1 kHz tone for 2 s");
    int chunks = BSP_AUDIO_SAMPLE_RATE * 2 / CHUNK;
    for (int c = 0; chunks < 0 || c < chunks; c++) {
        if (esp_codec_dev_write(spk, buf, sizeof(buf)) != ESP_CODEC_DEV_OK) {
            ESP_LOGE(TAG, "write failed");
            break;
        }
    }

    ESP_LOGI(TAG, "volume sweep 60 -> 5 -> 60");
    for (int v = 60; v >= 5; v -= 5) {
        ESP_ERROR_CHECK(bsp_audio_set_volume(v));
        for (int c = 0; c < BSP_AUDIO_SAMPLE_RATE / 10 / CHUNK; c++) {
            esp_codec_dev_write(spk, buf, sizeof(buf));
        }
    }
    for (int v = 5; v <= 60; v += 5) {
        ESP_ERROR_CHECK(bsp_audio_set_volume(v));
        for (int c = 0; c < BSP_AUDIO_SAMPLE_RATE / 10 / CHUNK; c++) {
            esp_codec_dev_write(spk, buf, sizeof(buf));
        }
    }

    ESP_LOGI(TAG, "done; replaying every 5 s (Ctrl+C to stop)");
    while (true) {
        vTaskDelay(pdMS_TO_TICKS(5000));
        for (int c = 0; c < chunks; c++) {
            esp_codec_dev_write(spk, buf, sizeof(buf));
        }
    }
}

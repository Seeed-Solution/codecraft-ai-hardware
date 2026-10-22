/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher microphone example: reads 100 ms windows from the
 * ES7243 ADC and logs the RMS level, with a simple bar graph.
 */

#include "bsp_audio.h"

#include <math.h>
#include <string.h>

#include "esp_log.h"
#include "esp_codec_dev.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "mic_level";

#define WINDOW_MS 100
#define SAMPLES   (BSP_AUDIO_SAMPLE_RATE * WINDOW_MS / 1000)

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_audio_init());
    esp_codec_dev_handle_t mic = bsp_audio_get_mic();

    static int16_t buf[SAMPLES];
    static char bar[49];

    ESP_LOGI(TAG, "make some noise near the mic");
    while (true) {
        if (esp_codec_dev_read(mic, buf, sizeof(buf)) != ESP_CODEC_DEV_OK) {
            ESP_LOGE(TAG, "read failed");
            vTaskDelay(pdMS_TO_TICKS(500));
            continue;
        }
        double acc = 0;
        for (int i = 0; i < SAMPLES; i++) {
            acc += (double)buf[i] * buf[i];
        }
        int rms = (int)sqrt(acc / SAMPLES); /* 0..32768 */
        int db = rms > 0 ? (int)(20.0 * log10((double)rms / 32768.0)) : -90;
        int fill = (db + 60) * 48 / 60; /* -60 dBFS .. 0 dBFS */
        if (fill < 0) fill = 0;
        if (fill > 48) fill = 48;
        memset(bar, '#', fill);
        bar[fill] = '\0';
        printf("%4d dBFS |%-48s|\n", db, bar);
    }
}

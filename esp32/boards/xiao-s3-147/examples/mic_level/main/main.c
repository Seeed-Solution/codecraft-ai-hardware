/* mic_level: capture the PDM mic at 16 kHz and log an RMS level bar twice
 * a second. Demonstrates bsp_mic. */
#include <math.h>

#include "bsp_mic.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "mic_level";

#define CHUNK_SAMPLES 512

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_mic_init(16000));
    static int16_t samples[CHUNK_SAMPLES];
    for (;;) {
        size_t got = 0;
        if (bsp_mic_read(samples, CHUNK_SAMPLES, &got, 500) != ESP_OK || got == 0) {
            vTaskDelay(pdMS_TO_TICKS(100));
            continue;
        }
        double acc = 0;
        for (size_t i = 0; i < got; i++) acc += (double)samples[i] * samples[i];
        int level = (int)(sqrt(acc / got) / 32768.0 * 40.0);
        if (level > 40) level = 40;
        char bar[41];
        for (int i = 0; i < 40; i++) bar[i] = i < level ? '#' : ' ';
        bar[40] = '\0';
        ESP_LOGI(TAG, "|%s|", bar);
    }
}

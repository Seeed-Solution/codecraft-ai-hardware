/* audio_meter: read the XMOS mic, log RMS level, mirror it on the WS2812.
 * Demonstrates bsp_audio capture + bsp_led. Requires the XMOS 16 kHz I2S
 * firmware v1.0.9 (see the board README). */
#include <math.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_audio.h"
#include "bsp_led.h"
#include "esp_log.h"

static const char *TAG = "audio_meter";
#define FRAMES 240

static int32_t s_buf[FRAMES * 2];

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_led_init());
    esp_err_t err = bsp_audio_init();
    if (err != ESP_OK) {
        /* XMOS missing or wrong firmware: blink red forever. */
        for (;;) {
            bsp_led_set_rgb(32, 0, 0);
            vTaskDelay(pdMS_TO_TICKS(200));
            bsp_led_off();
            vTaskDelay(pdMS_TO_TICKS(200));
        }
    }
    ESP_ERROR_CHECK(bsp_audio_mic_start());
    for (;;) {
        size_t frames = bsp_audio_mic_read(s_buf, FRAMES, 500);
        if (!frames) continue;
        double acc = 0;
        for (size_t i = 0; i < frames; i++) {
            double s = (double)s_buf[i * 2] / (double)INT32_MAX; /* XMOS processed channel */
            acc += s * s;
        }
        double rms = sqrt(acc / frames);
        uint8_t level = (uint8_t)(rms * 400 > 255 ? 255 : rms * 400);
        bsp_led_set_rgb(level, level / 4, 0);
        ESP_LOGI(TAG, "rms %.4f level %u", rms, level);
    }
}

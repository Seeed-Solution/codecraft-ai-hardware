/* board_health: periodic readout of every board service — RTC, charger,
 * battery, SHT4x — plus a startup beep and a live mic level meter. Use it
 * as a smoke test after assembly or as a starting point for a device
 * status page. */
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <time.h>

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_battery.h"
#include "bsp_buzzer.h"
#include "bsp_charger.h"
#include "bsp_mic.h"
#include "bsp_rtc.h"
#include "bsp_sht4x.h"
#include "esp_log.h"

static const char *TAG = "board_health";
static const char *CHARGE_STATE[] = {
    "discharging", "not charging", "pre-charge", "fast charge", "full",
};

static void report(void)
{
    struct tm t;
    esp_err_t err = bsp_rtc_get_time(&t);
    if (err == ESP_OK) {
        ESP_LOGI(TAG, "rtc %04d-%02d-%02d %02d:%02d:%02d", t.tm_year + 1900,
                 t.tm_mon + 1, t.tm_mday, t.tm_hour, t.tm_min, t.tm_sec);
    } else if (err == ESP_ERR_INVALID_STATE) {
        ESP_LOGW(TAG, "rtc time invalid (VL flag) — set it with bsp_rtc_set_time()");
    } else {
        ESP_LOGE(TAG, "rtc read failed: %s", esp_err_to_name(err));
    }

    bsp_charger_status_t st;
    bool input = false;
    uint8_t faults = 0;
    if (bsp_charger_get_status(&st) == ESP_OK &&
        bsp_charger_is_input_present(&input) == ESP_OK) {
        bsp_charger_get_faults(&faults);
        ESP_LOGI(TAG, "charger %s, usb %s, faults 0x%02x", CHARGE_STATE[st],
                 input ? "present" : "absent", faults);
    }

    int mv = 0;
    if (bsp_battery_read_mv(&mv) == ESP_OK) {
        ESP_LOGI(TAG, "battery %d mV", mv);
    }

    float temp = 0, rh = 0;
    if (bsp_sht4x_read(&temp, &rh) == ESP_OK) {
        ESP_LOGI(TAG, "ambient %.1f C, %.1f %%rH", (double)temp, (double)rh);
    }
}

static void mic_level(void)
{
    int16_t buf[1600]; /* 100 ms at 16 kHz */
    size_t got = 0;
    if (bsp_mic_read(buf, 1600, &got, 500) != ESP_OK || got == 0) {
        ESP_LOGW(TAG, "mic read failed");
        return;
    }
    int64_t acc = 0;
    for (size_t i = 0; i < got; i++) {
        acc += (int32_t)buf[i] * buf[i];
    }
    int rms = (int)sqrtf((float)(acc / (int64_t)got));
    ESP_LOGI(TAG, "mic rms %d (%zu samples)", rms, got);
}

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_battery_init());
    ESP_ERROR_CHECK(bsp_buzzer_init());
    ESP_ERROR_CHECK(bsp_mic_init(16000));

    if (bsp_rtc_init() != ESP_OK) {
        ESP_LOGW(TAG, "rtc not found");
    }
    if (bsp_charger_init() != ESP_OK) {
        ESP_LOGW(TAG, "charger not found");
    }
    if (bsp_sht4x_init() != ESP_OK) {
        ESP_LOGW(TAG, "sht4x not found");
    }

    bsp_buzzer_beep(2000, 80);

    while (true) {
        report();
        mic_level();
        vTaskDelay(pdMS_TO_TICKS(10000));
    }
}

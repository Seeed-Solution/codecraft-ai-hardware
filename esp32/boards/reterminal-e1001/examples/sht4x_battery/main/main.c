/* sht4x_battery: log temperature/humidity and battery voltage every 10 s.
 * The green LED blinks on each sample; any key press logs immediately. */
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_battery.h"
#include "bsp_button.h"
#include "bsp_led.h"
#include "bsp_sht4x.h"
#include "esp_log.h"

static const char *TAG = "sht4x_battery";
static const char *KEY_NAMES[] = {"GREEN", "LEFT", "RIGHT"};

static void sample(const char *why)
{
    float t = 0, rh = 0;
    int mv = 0;
    esp_err_t se = bsp_sht4x_read(&t, &rh);
    esp_err_t be = bsp_battery_read_mv(&mv);
    ESP_LOGI(TAG, "[%s] temp %.1f C, rh %.1f %% (%s), battery %d mV (%s)",
             why,
             se == ESP_OK ? (double)t : 0.0, se == ESP_OK ? (double)rh : 0.0,
             esp_err_to_name(se), be == ESP_OK ? mv : 0, esp_err_to_name(be));
    bsp_led_set(true);
    vTaskDelay(pdMS_TO_TICKS(50));
    bsp_led_set(false);
}

static void on_key(bsp_key_t key, bool pressed, void *arg)
{
    if (pressed) sample(KEY_NAMES[key]);
}

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_led_init());
    ESP_ERROR_CHECK(bsp_button_init(on_key, NULL));
    ESP_ERROR_CHECK(bsp_battery_init());
    esp_err_t err = bsp_sht4x_init();
    if (err != ESP_OK) {
        ESP_LOGW(TAG, "SHT4x not found: %s (battery-only mode)", esp_err_to_name(err));
    }
    for (;;) {
        sample("tick");
        vTaskDelay(pdMS_TO_TICKS(10000));
    }
}

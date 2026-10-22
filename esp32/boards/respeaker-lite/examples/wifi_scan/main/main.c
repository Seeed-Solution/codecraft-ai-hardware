/* wifi_scan: scan 2.4 GHz APs and blink the WS2812 green on each round.
 * reSpeaker Lite RF note: attach the XIAO external antenna; 2.4 GHz only.
 * Demonstrates Wi-Fi init with no credentials and no cloud. */
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_led.h"
#include "esp_log.h"
#include "esp_wifi.h"
#include "nvs_flash.h"

static const char *TAG = "wifi_scan";

void app_main(void)
{
    ESP_ERROR_CHECK(nvs_flash_init());
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());
    esp_netif_create_default_wifi_sta();
    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&cfg));
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
    ESP_ERROR_CHECK(esp_wifi_start());
    ESP_ERROR_CHECK(bsp_led_init());

    for (;;) {
        wifi_scan_config_t scan = {.show_hidden = true};
        ESP_ERROR_CHECK(esp_wifi_scan_start(&scan, true));
        uint16_t n = 0;
        esp_wifi_scan_get_ap_num(&n);
        ESP_LOGI(TAG, "found %u APs", n);
        bsp_led_set_rgb(0, 32, 0);
        vTaskDelay(pdMS_TO_TICKS(100));
        bsp_led_off();
        vTaskDelay(pdMS_TO_TICKS(4900));
    }
}

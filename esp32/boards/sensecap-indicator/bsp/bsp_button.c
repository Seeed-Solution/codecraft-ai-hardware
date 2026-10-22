/* SenseCAP Indicator button: top case button on GPIO 38, active low. Polled
 * with debounce in a dedicated task; callbacks must stay non-blocking.
 * Pin fact: Seeed-Solution/SenseCAP_Indicator_ESP32 (Apache 2.0)
 * sensecap_indicator_board.c g_btns[] and Meta muse-gadget-sdk
 * esp32/devices/sdkconfig.sensecap-indicator (Apache 2.0). */
#include "bsp_button.h"
#include "bsp_pins.h"

#include "driver/gpio.h"
#include "esp_check.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#define BSP_BUTTON_DEBOUNCE_MS 30
#define BSP_BUTTON_POLL_MS     20

static bsp_button_cb_t s_cb;
static volatile bool s_pressed;

static void bsp_button_task(void *arg)
{
    bool stable = false, raw_last = false;
    uint32_t stable_ms = 0;
    for (;;) {
        bool raw = gpio_get_level(BSP_PIN_BUTTON_USER) == 0;
        uint32_t now = xTaskGetTickCount() * portTICK_PERIOD_MS;
        if (raw != raw_last) { raw_last = raw; stable_ms = now; }
        if (raw != stable && now - stable_ms >= BSP_BUTTON_DEBOUNCE_MS) {
            stable = raw;
            s_pressed = stable;
            if (s_cb) s_cb(stable, NULL);
        }
        vTaskDelay(pdMS_TO_TICKS(BSP_BUTTON_POLL_MS));
    }
}

esp_err_t bsp_button_init(bsp_button_cb_t cb)
{
    gpio_config_t cfg = {
        .pin_bit_mask = 1ULL << BSP_PIN_BUTTON_USER,
        .mode = GPIO_MODE_INPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&cfg), "bsp_button", "gpio");
    s_cb = cb;
    ESP_RETURN_ON_FALSE(
        xTaskCreate(bsp_button_task, "bsp_button", 2048, NULL, 5, NULL) == pdPASS,
        ESP_ERR_NO_MEM, "bsp_button", "task");
    return ESP_OK;
}

bool bsp_button_pressed(void)
{
    return s_pressed;
}

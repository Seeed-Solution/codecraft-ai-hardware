/* button_led: hold XIAO BOOT to cycle the WS2812 through red/green/blue.
 * Demonstrates bsp_button + bsp_led. Button callbacks run in the BSP task
 * and stay non-blocking — the LED write is a single RMT transaction. */
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_button.h"
#include "bsp_led.h"
#include "esp_log.h"

static const char *TAG = "button_led";
static int s_color;

static void on_button(bool pressed, void *arg)
{
    if (!pressed) return;
    s_color = (s_color + 1) % 3;
    ESP_LOGI(TAG, "color %d", s_color);
}

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_led_init());
    ESP_ERROR_CHECK(bsp_button_init(on_button));
    for (;;) {
        static const uint8_t colors[3][3] = {{32, 0, 0}, {0, 32, 0}, {0, 0, 32}};
        if (bsp_button_pressed()) bsp_led_set_rgb(colors[s_color][0], colors[s_color][1], colors[s_color][2]);
        else bsp_led_off();
        vTaskDelay(pdMS_TO_TICKS(50));
    }
}

/* button_hello: log presses of the AI/Up/Down keys. Demonstrates
 * bsp_button; the callback stays non-blocking (log only). */
#include "bsp_button.h"
#include "esp_log.h"

static const char *TAG = "button_hello";
static const char *const s_names[] = {"AI", "Up", "Down"};

static void on_button(bsp_button_id_t id, bool pressed, void *arg)
{
    (void)arg;
    if (pressed && id < BSP_BUTTON_COUNT) ESP_LOGI(TAG, "%s pressed", s_names[id]);
}

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_button_init(on_button, NULL));
}

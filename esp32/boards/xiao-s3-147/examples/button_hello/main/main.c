/* button_hello: USR1 logs a press; USR2 toggles the backlight.
 * Demonstrates bsp_button + bsp_display_backlight. Callbacks stay
 * non-blocking. */
#include "bsp_button.h"
#include "bsp_display.h"
#include "esp_log.h"

static const char *TAG = "button_hello";
static bool s_backlight = true;

static void on_button(bsp_button_id_t id, bool pressed, void *arg)
{
    (void)arg;
    if (!pressed) return;
    if (id == BSP_BUTTON_USR1) {
        ESP_LOGI(TAG, "USR1 pressed");
    } else if (id == BSP_BUTTON_USR2) {
        s_backlight = !s_backlight;
        bsp_display_backlight(s_backlight);
        ESP_LOGI(TAG, "backlight %s", s_backlight ? "on" : "off");
    }
}

void app_main(void)
{
    esp_lcd_panel_handle_t panel;
    ESP_ERROR_CHECK(bsp_display_init(&panel));
    ESP_ERROR_CHECK(bsp_button_init(on_button, NULL));
}

/* SenseCAP Indicator: white screen; each press of the top case button
 * (GPIO 38, active low) toggles the backlight. Expected result: screen
 * lights up white; pressing the button turns the backlight off/on. */
#include <stdlib.h>

#include "freertos/FreeRTOS.h"

#include "bsp_button.h"
#include "bsp_display.h"
#include "bsp_pins.h"
#include "esp_log.h"

static const char *TAG = "button_backlight";

static void on_button(bool pressed, void *arg)
{
    if (pressed) {
        bool now = !bsp_display_backlight_get();
        /* Backlight GPIO write is short; safe inside the BSP button task. */
        bsp_display_backlight_set(now);
        ESP_LOGI(TAG, "button pressed -> backlight %s", now ? "ON" : "OFF");
    }
}

void app_main(void)
{
    esp_lcd_panel_handle_t panel = NULL;
    ESP_ERROR_CHECK(bsp_display_init(&panel));
    ESP_ERROR_CHECK(bsp_display_backlight_set(true));

    uint16_t *line = malloc(BSP_LCD_H_RES * sizeof(uint16_t));
    configASSERT(line);
    for (int x = 0; x < BSP_LCD_H_RES; x++) {
        line[x] = 0xFFFF;
    }
    for (int y = 0; y < BSP_LCD_V_RES; y++) {
        ESP_ERROR_CHECK(esp_lcd_panel_draw_bitmap(panel, 0, y, BSP_LCD_H_RES,
                                                  y + 1, line));
    }
    free(line);

    ESP_ERROR_CHECK(bsp_button_init(on_button));
    ESP_LOGI(TAG, "white screen — press the top button to toggle backlight");
}

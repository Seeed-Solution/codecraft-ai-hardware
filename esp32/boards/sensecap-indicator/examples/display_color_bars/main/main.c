/* SenseCAP Indicator: fill the 480x480 panel with vertical color bars.
 * Expected result: 8 vertical bars (white, yellow, cyan, green, magenta,
 * red, blue, black), backlight on. */
#include <stdlib.h>

#include "freertos/FreeRTOS.h"

#include "bsp_display.h"
#include "bsp_pins.h"
#include "esp_log.h"

static const char *TAG = "color_bars";

void app_main(void)
{
    esp_lcd_panel_handle_t panel = NULL;
    ESP_ERROR_CHECK(bsp_display_init(&panel));
    ESP_ERROR_CHECK(bsp_display_backlight_set(true));

    /* RGB565 bars; byte order matches the panel's B0..R4 line mapping
     * (esp_lcd handles the swap for RGB565 on this bus). */
    static const uint16_t colors[8] = {
        0xFFFF, 0xFFE0, 0x07FF, 0x07E0, 0xF81F, 0xF800, 0x001F, 0x0000,
    };

    uint16_t *line = malloc(BSP_LCD_H_RES * sizeof(uint16_t));
    configASSERT(line);

    const int bar_w = BSP_LCD_H_RES / 8;
    for (int y = 0; y < BSP_LCD_V_RES; y++) {
        for (int x = 0; x < BSP_LCD_H_RES; x++) {
            line[x] = colors[x / bar_w];
        }
        ESP_ERROR_CHECK(esp_lcd_panel_draw_bitmap(panel, 0, y, BSP_LCD_H_RES,
                                                  y + 1, line));
    }
    free(line);
    ESP_LOGI(TAG, "color bars drawn — check the screen");
}

/* display_hello: fill the 172x320 panel with red/green/blue bands.
 * Demonstrates bsp_display_init + esp_lcd_panel_draw_bitmap. */
#include <string.h>

#include "bsp_display.h"
#include "bsp_pins.h"
#include "esp_log.h"
#include "esp_lcd_panel_ops.h"

static const char *TAG = "display_hello";

void app_main(void)
{
    esp_lcd_panel_handle_t panel;
    ESP_ERROR_CHECK(bsp_display_init(&panel));

    /* One band at a time, BSP_LCD_HRES x (VRES/3) RGB565 pixels. */
    static uint16_t band[BSP_LCD_HRES * (BSP_LCD_VRES / 3 + 1)];
    static const uint16_t colors[3] = {0xF800, 0x07E0, 0x001F};  /* R, G, B */
    const int band_h = BSP_LCD_VRES / 3;
    for (int i = 0; i < 3; i++) {
        for (int p = 0; p < BSP_LCD_HRES * band_h; p++) band[p] = colors[i];
        ESP_ERROR_CHECK(esp_lcd_panel_draw_bitmap(panel, 0, i * band_h,
                        BSP_LCD_HRES, (i + 1) * band_h, band));
    }
    ESP_LOGI(TAG, "bands drawn");
}

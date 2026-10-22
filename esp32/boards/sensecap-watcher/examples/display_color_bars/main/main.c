/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Fills the round panel with vertical color bands, one row of pixels at a
 * time, then fades the backlight in and out. No LVGL: straight esp_lcd.
 */
#include <string.h>

#include "bsp_display.h"
#include "esp_heap_caps.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "display_color_bars";

/* RGB565, in the panel's RGB element order. */
static const uint16_t BANDS[] = {0xF800, 0x07E0, 0x001F, 0xFFE0, 0xF81F, 0x07FF, 0xFFFF};
#define BAND_COUNT (sizeof(BANDS) / sizeof(BANDS[0]))

void app_main(void)
{
    esp_lcd_panel_handle_t panel;
    ESP_ERROR_CHECK(bsp_display_init(&panel, NULL));

    /* One row of pixels, held in DMA-capable internal RAM: the QSPI bus
     * driver hands it to the SPI hardware directly. */
    uint16_t *row = heap_caps_malloc(BSP_LCD_H_RES * 2, MALLOC_CAP_DMA);
    ESP_ERROR_CHECK(row ? ESP_OK : ESP_ERR_NO_MEM);

    const int band_w = BSP_LCD_H_RES / BAND_COUNT;
    for (int y = 0; y < BSP_LCD_V_RES; y++) {
        for (int x = 0; x < BSP_LCD_H_RES; x++) {
            int band = x / band_w;
            if (band >= (int)BAND_COUNT) {
                band = BAND_COUNT - 1;
            }
            row[x] = BANDS[band];
        }
        ESP_ERROR_CHECK(esp_lcd_panel_draw_bitmap(panel, 0, y, BSP_LCD_H_RES, y + 1, row));
    }
    free(row);
    ESP_LOGI(TAG, "bands drawn; fading the backlight");

    for (;;) {
        for (int p = 100; p >= 0; p -= 5) {
            bsp_display_set_brightness(p);
            vTaskDelay(pdMS_TO_TICKS(30));
        }
        for (int p = 0; p <= 100; p += 5) {
            bsp_display_set_brightness(p);
            vTaskDelay(pdMS_TO_TICKS(30));
        }
    }
}

/* epd_hello: draw a test pattern on the SSD1677 (portrait 480x800), then
 * invert it with a fast refresh two seconds later. Demonstrates
 * bsp_epd_update with full and fast refreshes. */
#include <string.h>

#include "bsp_epd.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "epd_hello";

static void draw_pattern(uint8_t *frame, bool inverted)
{
    /* 1 = white, 0 = black, MSB = leftmost pixel of the row. */
    memset(frame, inverted ? 0x00 : 0xFF, BSP_EPD_FRAME_BYTES);
    for (int y = 0; y < BSP_EPD_HEIGHT; y++) {
        for (int x = 0; x < BSP_EPD_WIDTH; x++) {
            bool border = x < 4 || y < 4 || x >= BSP_EPD_WIDTH - 4 || y >= BSP_EPD_HEIGHT - 4;
            bool diagonal = (x - y % BSP_EPD_WIDTH + BSP_EPD_WIDTH) % BSP_EPD_WIDTH < 4;
            bool checker = ((x / 40) + (y / 40)) % 2 == 0 && y > BSP_EPD_HEIGHT / 2;
            bool black = border || diagonal || checker;
            if (inverted) black = !black;
            if (black) frame[y * BSP_EPD_ROW_BYTES + x / 8] &= (uint8_t)~(0x80 >> (x & 7));
        }
    }
}

void app_main(void)
{
    static uint8_t frame[BSP_EPD_FRAME_BYTES];
    static uint8_t prev[BSP_EPD_FRAME_BYTES];

    ESP_ERROR_CHECK(bsp_epd_init());

    draw_pattern(frame, false);
    ESP_ERROR_CHECK(bsp_epd_update(frame, NULL, true));  /* first update: full */
    ESP_LOGI(TAG, "full refresh done");

    vTaskDelay(pdMS_TO_TICKS(2000));
    memcpy(prev, frame, sizeof(prev));
    draw_pattern(frame, true);
    ESP_ERROR_CHECK(bsp_epd_update(frame, prev, false));  /* fast refresh */
    ESP_LOGI(TAG, "fast refresh done");
}

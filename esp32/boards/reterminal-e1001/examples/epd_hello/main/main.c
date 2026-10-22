/* epd_hello: fill the framebuffer with a checkerboard and gray bars,
 * push one full refresh. The panel sleeps afterwards; the image stays.
 * Framebuffer: 1 bpp, MSB leftmost, 1 = white. */
#include <string.h>

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_epd.h"
#include "bsp_led.h"
#include "esp_heap_caps.h"
#include "esp_log.h"

static const char *TAG = "epd_hello";

static void set_pixel(uint8_t *fb, int x, int y, bool white)
{
    uint8_t *byte = fb + (size_t)y * BSP_EPD_ROW_BYTES + x / 8;
    uint8_t mask = (uint8_t)(0x80 >> (x & 7));
    if (white) *byte |= mask;
    else *byte &= (uint8_t)~mask;
}

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_led_init());
    bsp_led_set(true);

    uint8_t *fb = heap_caps_malloc(BSP_EPD_FRAME_BYTES, MALLOC_CAP_SPIRAM);
    if (!fb) fb = malloc(BSP_EPD_FRAME_BYTES);
    if (!fb) {
        ESP_LOGE(TAG, "no %u bytes for the framebuffer", BSP_EPD_FRAME_BYTES);
        return;
    }
    memset(fb, 0xFF, BSP_EPD_FRAME_BYTES); /* all white */

    /* Top half: 40 px checkerboard. Bottom half: 8 vertical bars from
     * white to black. Border frame around everything. */
    for (int y = 0; y < BSP_EPD_HEIGHT / 2; y++) {
        for (int x = 0; x < BSP_EPD_WIDTH; x++) {
            set_pixel(fb, x, y, ((x / 40) + (y / 40)) % 2 == 0);
        }
    }
    for (int y = BSP_EPD_HEIGHT / 2; y < BSP_EPD_HEIGHT; y++) {
        for (int x = 0; x < BSP_EPD_WIDTH; x++) {
            set_pixel(fb, x, y, (x / 100) % 2 == 0);
        }
    }
    for (int x = 0; x < BSP_EPD_WIDTH; x++) {
        set_pixel(fb, x, 0, false);
        set_pixel(fb, x, BSP_EPD_HEIGHT - 1, false);
    }
    for (int y = 0; y < BSP_EPD_HEIGHT; y++) {
        set_pixel(fb, 0, y, false);
        set_pixel(fb, BSP_EPD_WIDTH - 1, y, false);
    }

    ESP_ERROR_CHECK(bsp_epd_init());
    ESP_LOGI(TAG, "refreshing (%u bytes)...", BSP_EPD_FRAME_BYTES);
    ESP_ERROR_CHECK(bsp_epd_update(fb, NULL, true));
    free(fb);
    bsp_led_set(false);
    ESP_LOGI(TAG, "done; the image persists with no power");
}

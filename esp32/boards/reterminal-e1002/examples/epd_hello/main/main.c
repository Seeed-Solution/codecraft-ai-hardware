/* epd_hello: six vertical color bars, one full refresh (~30 s).
 * The panel sleeps afterwards; the image stays.
 * Framebuffer: 4 bpp, left pixel in the high nibble. */
#include <string.h>

#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "bsp_epd.h"
#include "bsp_led.h"
#include "esp_heap_caps.h"
#include "esp_log.h"

static const char *TAG = "epd_hello";

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
    memset(fb, 0x11, BSP_EPD_FRAME_BYTES); /* all white */

    static const bsp_epd_ink_t bars[] = {
        BSP_EPD_INK_BLACK, BSP_EPD_INK_WHITE, BSP_EPD_INK_YELLOW,
        BSP_EPD_INK_RED, BSP_EPD_INK_BLUE, BSP_EPD_INK_GREEN,
    };
    const int bar_w = BSP_EPD_WIDTH / 6;
    for (int y = 0; y < BSP_EPD_HEIGHT; y++) {
        for (int x = 0; x < BSP_EPD_WIDTH; x++) {
            bsp_epd_set_pixel(fb, x, y, bars[x / bar_w]);
        }
    }
    /* White separator lines between bars. */
    for (int b = 1; b < 6; b++) {
        for (int y = 0; y < BSP_EPD_HEIGHT; y++) {
            bsp_epd_set_pixel(fb, b * bar_w, y, BSP_EPD_INK_WHITE);
        }
    }

    ESP_ERROR_CHECK(bsp_epd_init());
    ESP_LOGI(TAG, "refreshing (%u bytes, ~30 s)...", BSP_EPD_FRAME_BYTES);
    ESP_ERROR_CHECK(bsp_epd_update(fb, NULL, true));
    free(fb);
    bsp_led_set(false);
    ESP_LOGI(TAG, "done; the image persists with no power");
}

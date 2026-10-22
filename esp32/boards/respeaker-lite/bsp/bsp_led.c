/* reSpeaker Lite status LED: one WS2812 on GPIO 1 (XIAO D0), no power
 * switch. Pin source: Meta muse-gadget-sdk main/led_status.c (Apache 2.0),
 * cross-checked against the Seeed XIAO D-pin mapping. */
#include "bsp_led.h"
#include "bsp_pins.h"

#include "led_strip.h"

static led_strip_handle_t s_strip;

esp_err_t bsp_led_init(void)
{
    led_strip_config_t strip = {
        .strip_gpio_num = BSP_PIN_WS2812_DATA,
        .max_leds = 1,
    };
    led_strip_rmt_config_t rmt = {
        .resolution_hz = 10 * 1000 * 1000,
    };
    return led_strip_new_rmt_device(&strip, &rmt, &s_strip);
}

esp_err_t bsp_led_set_rgb(uint8_t r, uint8_t g, uint8_t b)
{
    if (!s_strip) return ESP_ERR_INVALID_STATE;
    esp_err_t err = led_strip_set_pixel(s_strip, 0, r, g, b);
    if (err == ESP_OK) err = led_strip_refresh(s_strip);
    return err;
}

esp_err_t bsp_led_off(void)
{
    if (!s_strip) return ESP_ERR_INVALID_STATE;
    return led_strip_clear(s_strip);
}

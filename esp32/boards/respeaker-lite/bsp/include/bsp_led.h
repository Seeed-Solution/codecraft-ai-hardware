/* reSpeaker Lite status LED: one WS2812 on GPIO 1 (XIAO D0). */
#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

esp_err_t bsp_led_init(void);
esp_err_t bsp_led_set_rgb(uint8_t r, uint8_t g, uint8_t b);
esp_err_t bsp_led_off(void);

#ifdef __cplusplus
}
#endif

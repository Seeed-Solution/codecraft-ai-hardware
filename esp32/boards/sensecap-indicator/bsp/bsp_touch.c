/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Capacitive touch on the shared board I2C bus. Address list, probe order
 * and the IO7 reset pulse come from the vendor BSP
 * (SenseCAP_Indicator_ESP32 components/bsp/src/indev/indev_tp.c and
 * boards/sensecap_indicator_board.c, Apache 2.0); reimplemented on the
 * IDF v6 i2c_master driver with the esp_lcd_touch FT5x06/GT911 drivers.
 */

#include "bsp_touch.h"

#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "driver/i2c_master.h"
#include "esp_check.h"
#include "esp_lcd_panel_io.h"
#include "esp_lcd_touch_ft5x06.h"
#include "esp_lcd_touch_gt911.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "bsp_touch";

static esp_lcd_touch_handle_t s_touch;

typedef struct {
    uint8_t addr;
    bool goodix; /* false = FocalTech FT5x06 family */
} tp_candidate_t;

/* Vendor probe order: FT6336U first (GX screen ships with it), then GT911. */
static const tp_candidate_t s_candidates[] = {
    { BSP_I2C_ADDR_TOUCH_FT6x36,     false },
    { BSP_I2C_ADDR_TOUCH_FT6x36_ALT, false },
    { BSP_I2C_ADDR_TOUCH_GT911,      true  },
    { BSP_I2C_ADDR_TOUCH_GT911_ALT,  true  },
};

esp_err_t bsp_touch_init(esp_lcd_touch_handle_t *out_handle)
{
    if (s_touch) {
        if (out_handle) {
            *out_handle = s_touch;
        }
        return ESP_OK;
    }

    /* Vendor reset sequence: IO7 low for 5 ms, then high (output mode). */
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(
        bsp_io_expander_set_direction(BSP_EXP_IO_TP_RESET, 1), TAG, "tp rst dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_IO_TP_RESET, 0),
                        TAG, "tp rst low");
    vTaskDelay(pdMS_TO_TICKS(5));
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_IO_TP_RESET, 1),
                        TAG, "tp rst high");
    vTaskDelay(pdMS_TO_TICKS(100)); /* let the controller boot */

    i2c_master_bus_handle_t bus;
    ESP_RETURN_ON_ERROR(bsp_i2c_get_bus(&bus), TAG, "i2c bus");

    const tp_candidate_t *found = NULL;
    for (size_t i = 0; i < sizeof(s_candidates) / sizeof(s_candidates[0]); i++) {
        if (i2c_master_probe(bus, s_candidates[i].addr, 50) == ESP_OK) {
            found = &s_candidates[i];
            break;
        }
    }
    ESP_RETURN_ON_FALSE(found, ESP_ERR_NOT_FOUND, TAG,
                        "no touch controller at 0x%02x/0x%02x/0x%02x/0x%02x",
                        BSP_I2C_ADDR_TOUCH_FT6x36, BSP_I2C_ADDR_TOUCH_FT6x36_ALT,
                        BSP_I2C_ADDR_TOUCH_GT911, BSP_I2C_ADDR_TOUCH_GT911_ALT);

    esp_lcd_panel_io_handle_t io;
    esp_err_t ret;
    const esp_lcd_touch_config_t tp_cfg = {
        .x_max = BSP_LCD_H_RES,
        .y_max = BSP_LCD_V_RES,
        .rst_gpio_num = GPIO_NUM_NC, /* lives on the IO expander */
        .int_gpio_num = GPIO_NUM_NC, /* not wired; poll instead */
        .levels = { .reset = 0, .interrupt = 0 },
        .flags = { .swap_xy = 0, .mirror_x = 0, .mirror_y = 0 },
    };

    if (!found->goodix) {
        esp_lcd_panel_io_i2c_config_t io_cfg = ESP_LCD_TOUCH_IO_I2C_FT5x06_CONFIG();
        io_cfg.dev_addr = found->addr;
        ESP_RETURN_ON_ERROR(esp_lcd_new_panel_io_i2c(bus, &io_cfg, &io), TAG,
                            "touch io");
        ret = esp_lcd_touch_new_i2c_ft5x06(io, &tp_cfg, &s_touch);
    } else {
        esp_lcd_panel_io_i2c_config_t io_cfg = ESP_LCD_TOUCH_IO_I2C_GT911_CONFIG();
        io_cfg.dev_addr = found->addr;
        ESP_RETURN_ON_ERROR(esp_lcd_new_panel_io_i2c(bus, &io_cfg, &io), TAG,
                            "touch io");
        ret = esp_lcd_touch_new_i2c_gt911(io, &tp_cfg, &s_touch);
    }
    ESP_RETURN_ON_ERROR(ret, TAG, "touch driver @0x%02x", found->addr);

    if (out_handle) {
        *out_handle = s_touch;
    }
    ESP_LOGI(TAG, "touch ready: %s @0x%02x (%dx%d)",
             found->goodix ? "GT911" : "FT6336U", found->addr,
             BSP_LCD_H_RES, BSP_LCD_V_RES);
    return ESP_OK;
}

esp_lcd_touch_handle_t bsp_touch_get_handle(void)
{
    return s_touch;
}

/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SPD2010 capacitive touch on I2C1. Bus/pin facts from the vendor BSP
 * (SenseCAP-Watcher-Firmware components/sensecap-watcher, Apache 2.0);
 * reimplemented on the IDF v6 i2c_master driver.
 */

#include "bsp_touch.h"

#include "bsp_display.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "driver/i2c_master.h"
#include "esp_check.h"
#include "esp_lcd_panel_io.h"
#include "esp_lcd_touch_spd2010.h"
#include "esp_log.h"

static const char *TAG = "bsp_touch";

static i2c_master_bus_handle_t s_bus;
static esp_lcd_panel_io_handle_t s_io;
static esp_lcd_touch_handle_t s_touch;

esp_err_t bsp_touch_init(esp_lcd_touch_handle_t *out_handle)
{
    if (s_touch) {
        if (out_handle) {
            *out_handle = s_touch;
        }
        return ESP_OK;
    }

    /* The touch interrupt is pin 5 of the IO expander. */
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_TOUCH_INT, 0),
                        TAG, "touch int input");

    i2c_master_bus_config_t bus_cfg = {
        .i2c_port = 1,
        .sda_io_num = BSP_PIN_TOUCH_SDA,
        .scl_io_num = BSP_PIN_TOUCH_SCL,
        .clk_source = I2C_CLK_SRC_DEFAULT,
        .glitch_ignore_cnt = 7,
        .flags.enable_internal_pullup = true,
    };
    ESP_RETURN_ON_ERROR(i2c_new_master_bus(&bus_cfg, &s_bus), TAG, "i2c1 bus");

    const esp_lcd_panel_io_i2c_config_t io_cfg = ESP_LCD_TOUCH_IO_I2C_SPD2010_CONFIG();
    ESP_RETURN_ON_ERROR(esp_lcd_new_panel_io_i2c(s_bus, &io_cfg, &s_io), TAG,
                        "touch io");

    const esp_lcd_touch_config_t tp_cfg = {
        .x_max = BSP_LCD_H_RES,
        .y_max = BSP_LCD_V_RES,
        .rst_gpio_num = GPIO_NUM_NC, /* shared with the LCD reset */
        .int_gpio_num = GPIO_NUM_NC, /* lives on the IO expander instead */
        .levels = { .reset = 0, .interrupt = 0 },
        .flags = { .swap_xy = 0, .mirror_x = 0, .mirror_y = 0 },
    };
    ESP_RETURN_ON_ERROR(esp_lcd_touch_new_i2c_spd2010(s_io, &tp_cfg, &s_touch),
                        TAG, "spd2010 touch");

    /* One read wakes the controller up (vendor BSP note). */
    esp_lcd_touch_read_data(s_touch);

    if (out_handle) {
        *out_handle = s_touch;
    }
    ESP_LOGI(TAG, "touch ready (%dx%d)", BSP_LCD_H_RES, BSP_LCD_V_RES);
    return ESP_OK;
}

esp_lcd_touch_handle_t bsp_touch_get_handle(void)
{
    return s_touch;
}

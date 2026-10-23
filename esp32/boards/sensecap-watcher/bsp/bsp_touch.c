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

#include <stdlib.h>

#include "driver/i2c_master.h"
#include "esp_check.h"
#include "esp_lcd_panel_io.h"
#include "esp_lcd_panel_io_interface.h"
#include "esp_lcd_touch_spd2010.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "bsp_touch";

static i2c_master_bus_handle_t s_bus;
static esp_lcd_panel_io_handle_t s_io;
static esp_lcd_touch_handle_t s_touch;

/* Minimal panel IO talking to i2c_master directly.
 *
 * The spd2010 touch component (pinned at 0.0.1 by the vendor BSP) drives
 * its IO with lcd_cmd_bits=0 and no control phase. That was fine for the
 * legacy-I2C esp_lcd IO it was written for, but IDF v6's esp_lcd panel IO
 * is built on i2c_master, whose i2c_master_transmit_receive() rejects a
 * zero-length write -- so every rx_param in the touch driver fails with
 * ESP_ERR_INVALID_ARG ("Rx failed") before a single byte goes on the wire.
 *
 * The SPD2010 touch protocol needs no command framing anyway: tx_param
 * writes a raw payload (usually a 2-byte register address) and rx_param is
 * a plain read, so this shim maps them straight onto i2c_master. */

#define TOUCH_I2C_TIMEOUT  100  /* ms; bounded so a stuck bus cannot hang us */
/* Largest frame the touch driver has a buffer for: 4-byte header plus
 * 10 fingers * 6 bytes. The read length comes from the device and is
 * garbage during its boot phase, so never trust it past this. */
#define TOUCH_MAX_READ     64

typedef struct {
    esp_lcd_panel_io_t base;
    i2c_master_bus_handle_t bus; /* for bus reset on NACK */
    i2c_master_dev_handle_t dev;
} touch_panel_io_t;

static esp_err_t touch_io_tx_param(esp_lcd_panel_io_t *io, int lcd_cmd, const void *param, size_t param_size)
{
    (void)lcd_cmd; /* the touch protocol carries its register address in the payload */
    touch_panel_io_t *tio = __containerof(io, touch_panel_io_t, base);
    esp_err_t ret = i2c_master_transmit(tio->dev, param, param_size, TOUCH_I2C_TIMEOUT);
    if (ret != ESP_OK) {
        ESP_LOGW(TAG, "tx %d bytes -> %s", param_size, esp_err_to_name(ret));
    }
    return ret;
}

static esp_err_t touch_io_rx_param(esp_lcd_panel_io_t *io, int lcd_cmd, void *param, size_t param_size)
{
    (void)lcd_cmd;
    touch_panel_io_t *tio = __containerof(io, touch_panel_io_t, base);
    if (param_size > TOUCH_MAX_READ) {
        ESP_LOGW(TAG, "rx clamped: %d -> %d bytes", param_size, TOUCH_MAX_READ);
        param_size = TOUCH_MAX_READ;
    }
    esp_err_t ret = i2c_master_receive(tio->dev, param, param_size, TOUCH_I2C_TIMEOUT);
    if (ret != ESP_OK) {
        ESP_LOGW(TAG, "rx %d bytes -> %s", param_size, esp_err_to_name(ret));
    }
    return ret;
}

static esp_err_t touch_io_tx_color(esp_lcd_panel_io_t *io, int lcd_cmd, const void *color, size_t color_size)
{
    return ESP_ERR_NOT_SUPPORTED; /* touch controllers have no framebuffer */
}

static esp_err_t touch_io_register_cbs(esp_lcd_panel_io_t *io, const esp_lcd_panel_io_callbacks_t *cbs, void *user_ctx)
{
    return ESP_ERR_NOT_SUPPORTED; /* polled device, no transfer-done callback */
}

static esp_err_t touch_io_del(esp_lcd_panel_io_t *io)
{
    touch_panel_io_t *tio = __containerof(io, touch_panel_io_t, base);
    esp_err_t ret = i2c_master_bus_rm_device(tio->dev);
    free(tio);
    return ret;
}

static esp_err_t touch_panel_io_new(i2c_master_bus_handle_t bus, esp_lcd_panel_io_handle_t *out_io)
{
    touch_panel_io_t *tio = calloc(1, sizeof(touch_panel_io_t));
    ESP_RETURN_ON_FALSE(tio, ESP_ERR_NO_MEM, TAG, "no mem for touch io");

    const i2c_device_config_t dev_cfg = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = ESP_LCD_TOUCH_IO_I2C_SPD2010_ADDRESS,
        .scl_speed_hz = BSP_I2C_FREQ_HZ,
    };
    tio->bus = bus;
    ESP_RETURN_ON_ERROR(i2c_master_bus_add_device(bus, &dev_cfg, &tio->dev),
                        TAG, "touch i2c dev");

    tio->base.rx_param = touch_io_rx_param;
    tio->base.tx_param = touch_io_tx_param;
    tio->base.tx_color = touch_io_tx_color;
    tio->base.del = touch_io_del;
    tio->base.register_event_callbacks = touch_io_register_cbs;
    *out_io = &tio->base;
    return ESP_OK;
}

esp_err_t bsp_touch_init(esp_lcd_touch_handle_t *out_handle)
{
    if (s_touch) {
        if (out_handle) {
            *out_handle = s_touch;
        }
        return ESP_OK;
    }

    /* The touch controller sits in the display assembly and is fed by the
     * LCD rail, so it answers no I2C until that rail is on (bsp_display does
     * the same; both may be up already). */
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_PWR_LCD, 1), TAG, "lcd rail dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_PIN_PWR_LCD, 1), TAG, "lcd rail on");
    vTaskDelay(pdMS_TO_TICKS(100));   /* let the controller boot, as the vendor BSP does */
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

    ESP_RETURN_ON_ERROR(touch_panel_io_new(s_bus, &s_io), TAG, "touch io");

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

    /* One read wakes the controller up (vendor BSP note). It then NACKs
     * I2C while its touch firmware boots, so poll until a read succeeds
     * instead of returning a handle that errors for the first ~100 ms. */
    vTaskDelay(pdMS_TO_TICKS(50));
    for (int attempt = 0; attempt < 25; attempt++) {
        if (esp_lcd_touch_read_data(s_touch) == ESP_OK) {
            break;
        }
        vTaskDelay(pdMS_TO_TICKS(20));
    }

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

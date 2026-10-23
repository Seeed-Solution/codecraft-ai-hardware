/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Board glue between the PCA9535 expander and the shared SSCMA camera
 * driver. Wiring and the power-up sequence (system rail, 100 ms, camera
 * rail; no reset line — powering the rail up resets the chip) are
 * field-verified in muse-gadget-sdk
 * esp32/components/muse/boards/board_sensecap_watcher.c (Apache 2.0); the
 * pin numbers cross-check against the vendor BSP header.
 *
 * The Himax reset on expander pin 7 follows the vendor BSP sequence
 * (SenseCAP-Watcher-Firmware, Apache-2.0): the rail comes up, the chip
 * boots (its SYNC pin asserts), and only then is reset pulsed low for
 * 100 ms with a 200 ms start delay. Pulsing it before the boot completed
 * broke captures on the field unit; this order is what works.
 */
#include "bsp_camera.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "camera.h"
#include "camera_sscma.h"
#include "driver/gpio.h"
#include "driver/spi_master.h"
#include "esp_check.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "bsp_camera";

/* Called by the driver from its own task during capture/stream; the expander
 * driver's shadow registers make this a single-pin I2C write. */
static esp_err_t camera_power(bool on)
{
    return bsp_io_expander_set_level(BSP_EXP_PIN_PWR_AI_CHIP, on);
}

static bool camera_has_data(void)
{
    int level = 0;
    return bsp_io_expander_get_level(BSP_EXP_PIN_CAM_SYNC, &level) == ESP_OK && level;
}

/* Active low, as in the vendor client (reset_active_high = false). The driver
 * pulses this only after the chip has booted, never while the rail is off. */
static esp_err_t camera_reset(bool hold)
{
    return bsp_io_expander_set_level(BSP_EXP_PIN_CAM_RST, hold ? 0 : 1);
}

esp_err_t bsp_camera_init(void)
{
    static bool done;
    if (done) {
        return ESP_OK;
    }
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");

    /* System rail first (it holds the board up on battery), then the camera
     * rail is left to the driver's power callback. */
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_PWR_SYSTEM, 1), TAG, "system dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_PIN_PWR_SYSTEM, 1), TAG, "system on");
    vTaskDelay(pdMS_TO_TICKS(100));
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_PWR_AI_CHIP, 1), TAG, "camera dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_CAM_SYNC, 0), TAG, "sync dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_CAM_RST, 1), TAG, "reset dir");
    ESP_RETURN_ON_ERROR(camera_reset(false), TAG, "reset idle");

    /* The microSD slot shares this SPI bus and its CS idles low (external
     * pulldown R162), so an inserted card would answer on MISO together with
     * the Himax. Deassert it before any camera traffic, as the vendor BSP
     * does in bsp_sscma_client_init. */
    const gpio_config_t sd_cs = {
        .pin_bit_mask = 1ULL << BSP_PIN_SD_CS,
        .mode = GPIO_MODE_OUTPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&sd_cs), TAG, "sd cs");
    gpio_set_level(BSP_PIN_SD_CS, 1);

    const camera_sscma_config_t cfg = {
        .name = "Himax WiseEye2 (SenseCAP Watcher)",
        .host = SPI2_HOST,
        .sclk = BSP_PIN_SPI2_SCLK,
        .mosi = BSP_PIN_SPI2_MOSI,
        .miso = BSP_PIN_SPI2_MISO,
        .cs = BSP_PIN_CAM_CS,
        .power = camera_power,
        .has_data = camera_has_data,
        .reset = camera_reset,
        .resolution = 3,           /* 640x480 stills */
        .stream_resolution = 1,    /* 416x416 preview frames: they fit the round screen */
    };
    camera_register(camera_sscma(&cfg));
    done = true;
    return ESP_OK;
}

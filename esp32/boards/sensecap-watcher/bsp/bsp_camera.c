/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Board glue between the PCA9535 expander and the shared SSCMA camera
 * driver. Wiring and the power-up sequence (system rail, 100 ms, camera
 * rail; no reset line — power-up resets the chip) are field-verified in
 * muse-gadget-sdk esp32/components/muse/boards/board_sensecap_watcher.c
 * (Apache 2.0); the pin numbers cross-check against the vendor BSP header.
 */
#include "bsp_camera.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "camera.h"
#include "camera_sscma.h"
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

    const camera_sscma_config_t cfg = {
        .name = "Himax WiseEye2 (SenseCAP Watcher)",
        .host = SPI2_HOST,
        .sclk = BSP_PIN_SPI2_SCLK,
        .mosi = BSP_PIN_SPI2_MOSI,
        .miso = BSP_PIN_SPI2_MISO,
        .cs = BSP_PIN_CAM_CS,
        .power = camera_power,
        .reset = NULL,             /* power-up resets the chip (muse-verified) */
        .has_data = camera_has_data,
        .resolution = 3,           /* 640x480 stills */
        .stream_resolution = 1,    /* 416x416 preview frames: they fit the round screen */
    };
    camera_register(camera_sscma(&cfg));
    done = true;
    return ESP_OK;
}

/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * microSD over SDSPI on the shared SPI2 bus. Pin/detect/rail facts from
 * the vendor BSP (SenseCAP-Watcher-Firmware components/sensecap-watcher,
 * Apache 2.0).
 */

#include "bsp_sdcard.h"

#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "driver/sdspi_host.h"
#include "driver/spi_common.h"
#include "esp_check.h"
#include "esp_log.h"
#include "esp_vfs_fat.h"

static const char *TAG = "bsp_sdcard";

static sdmmc_card_t *s_card;

static esp_err_t ensure_spi2(void)
{
    const spi_bus_config_t bus = {
        .sclk_io_num = BSP_PIN_SPI2_SCLK,
        .mosi_io_num = BSP_PIN_SPI2_MOSI,
        .miso_io_num = BSP_PIN_SPI2_MISO,
        .quadwp_io_num = -1,
        .quadhd_io_num = -1,
        /* Keep >= the camera's SSCMA_MAX_READ+1 so bus init order with the
         * Himax camera does not matter. */
        .max_transfer_sz = 4096,
    };
    esp_err_t err = spi_bus_initialize(SPI2_HOST, &bus, SPI_DMA_CH_AUTO);
    ESP_RETURN_ON_FALSE(err == ESP_OK || err == ESP_ERR_INVALID_STATE, err, TAG,
                        "spi2 bus");
    return ESP_OK;
}

esp_err_t bsp_sdcard_is_inserted(bool *inserted)
{
    ESP_RETURN_ON_FALSE(inserted != NULL, ESP_ERR_INVALID_ARG, TAG, "inserted");
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    int level = 1;
    ESP_RETURN_ON_ERROR(bsp_io_expander_get_level(BSP_EXP_PIN_SD_DET, &level), TAG,
                        "sd detect");
    *inserted = level == 0;
    return ESP_OK;
}

esp_err_t bsp_sdcard_init(const char *mount_point, sdmmc_card_t **out_card)
{
    ESP_RETURN_ON_FALSE(mount_point != NULL, ESP_ERR_INVALID_ARG, TAG, "mount_point");
    if (s_card) {
        if (out_card) {
            *out_card = s_card;
        }
        return ESP_OK;
    }

    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_PWR_SDCARD, 1),
                        TAG, "sd rail dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_PIN_PWR_SDCARD, 1), TAG,
                        "sd rail on");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_SD_DET, 0), TAG,
                        "sd detect input");

    bool inserted = false;
    ESP_RETURN_ON_ERROR(bsp_sdcard_is_inserted(&inserted), TAG, "detect");
    if (!inserted) {
        ESP_LOGW(TAG, "no SD card inserted");
        return ESP_ERR_NOT_FOUND;
    }

    ESP_RETURN_ON_ERROR(ensure_spi2(), TAG, "spi2");

    sdmmc_host_t host = SDSPI_HOST_DEFAULT();
    sdspi_device_config_t slot_config = SDSPI_DEVICE_CONFIG_DEFAULT();
    slot_config.gpio_cs = BSP_PIN_SD_CS;
    slot_config.host_id = SPI2_HOST;

    esp_vfs_fat_sdmmc_mount_config_t mount_config = {
        .format_if_mount_failed = false,
        .max_files = 5,
        .allocation_unit_size = 16 * 1024,
    };
    ESP_RETURN_ON_ERROR(
        esp_vfs_fat_sdspi_mount(mount_point, &host, &slot_config, &mount_config, &s_card),
        TAG, "mount");
    ESP_LOGI(TAG, "SD card mounted at %s", mount_point);
    if (out_card) {
        *out_card = s_card;
    }
    return ESP_OK;
}

esp_err_t bsp_sdcard_deinit(const char *mount_point)
{
    ESP_RETURN_ON_FALSE(mount_point != NULL, ESP_ERR_INVALID_ARG, TAG, "mount_point");
    if (!s_card) {
        return ESP_OK;
    }
    ESP_RETURN_ON_ERROR(esp_vfs_fat_sdcard_unmount(mount_point, s_card), TAG, "unmount");
    s_card = NULL;
    return ESP_OK;
}

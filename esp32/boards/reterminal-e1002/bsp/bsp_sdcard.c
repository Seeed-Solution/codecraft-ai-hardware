/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * microSD over SDSPI on the shared SPI2 bus (panel is the other device).
 * Pin/rail/detect facts from the vendor schematic E1001 V1.2, sheet
 * 06 Peripherals (CC BY-SA 4.0); the E1002 shares the E1001 mainboard
 * (see docs/vendor-sources.md).
 */

#include "bsp_sdcard.h"

#include "bsp_pins.h"

#include "driver/gpio.h"
#include "driver/sdspi_host.h"
#include "driver/spi_common.h"
#include "esp_check.h"
#include "esp_log.h"
#include "esp_vfs_fat.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "bsp_sdcard";

static sdmmc_card_t *s_card;

static esp_err_t ensure_spi2(void)
{
    const spi_bus_config_t bus = {
        .sclk_io_num = BSP_PIN_SD_SCLK,
        .mosi_io_num = BSP_PIN_SD_MOSI,
        .miso_io_num = BSP_PIN_SD_MISO,
        .quadwp_io_num = -1,
        .quadhd_io_num = -1,
        /* Matches bsp_epd's EPD_CHUNK_BYTES so bus init order between the
         * panel and the SD slot does not matter. */
        .max_transfer_sz = 4000,
    };
    esp_err_t err = spi_bus_initialize(SPI2_HOST, &bus, SPI_DMA_CH_AUTO);
    ESP_RETURN_ON_FALSE(err == ESP_OK || err == ESP_ERR_INVALID_STATE, err, TAG,
                        "spi2 bus");
    return ESP_OK;
}

esp_err_t bsp_sdcard_is_inserted(bool *inserted)
{
    ESP_RETURN_ON_FALSE(inserted != NULL, ESP_ERR_INVALID_ARG, TAG, "inserted");
    const gpio_config_t det = {
        .pin_bit_mask = 1ULL << BSP_PIN_SD_DET,
        .mode = GPIO_MODE_INPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&det), TAG, "detect pin");
    *inserted = gpio_get_level(BSP_PIN_SD_DET) == 0;
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

    /* TF_CARD_PWR rail on (TPS22916, active high), then let it ramp. */
    const gpio_config_t pwr = {
        .pin_bit_mask = 1ULL << BSP_PIN_SD_EN,
        .mode = GPIO_MODE_OUTPUT,
    };
    ESP_RETURN_ON_ERROR(gpio_config(&pwr), TAG, "rail pin");
    ESP_RETURN_ON_ERROR(gpio_set_level(BSP_PIN_SD_EN, 1), TAG, "rail on");
    vTaskDelay(pdMS_TO_TICKS(20));

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

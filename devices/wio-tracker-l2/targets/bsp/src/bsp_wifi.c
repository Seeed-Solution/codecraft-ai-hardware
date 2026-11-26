// SPDX-License-Identifier: Apache-2.0
#include "bsp_wifi.h"
#include "esp_wifi.h"

esp_err_t bsp_wifi_scan_count(uint16_t *count)
{
    if (!count) {
        return ESP_ERR_INVALID_ARG;
    }
    wifi_scan_config_t scan = {.scan_type = WIFI_SCAN_TYPE_PASSIVE};
    esp_err_t err = esp_wifi_scan_start(&scan, true);
    if (err != ESP_OK) {
        return err;
    }
    uint16_t found = 0;
    err = esp_wifi_scan_get_ap_num(&found);
    esp_err_t cleanup = esp_wifi_clear_ap_list();
    if (err == ESP_OK && cleanup == ESP_OK) {
        *count = found;
    }
    return err == ESP_OK ? cleanup : err;
}

/* sht4x: read the onboard SHT4x once a second and log temperature and
 * humidity. Demonstrates bsp_sht4x on the I2C1 sensor bus. */
#include "bsp_sht4x.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static const char *TAG = "sht4x";

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_sht4x_init());
    for (;;) {
        float temp_c, rh_pct;
        esp_err_t err = bsp_sht4x_read(&temp_c, &rh_pct);
        if (err == ESP_OK) {
            ESP_LOGI(TAG, "%.1f C, %.0f %%RH", temp_c, rh_pct);
        } else {
            ESP_LOGW(TAG, "read failed: %s", esp_err_to_name(err));
        }
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}

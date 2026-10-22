idf_component_register(
    SRCS "bsp_epd.c" "bsp_button.c" "bsp_led.c" "bsp_sht4x.c" "bsp_battery.c"
         "bsp_i2c0_internal.c" "bsp_rtc.c" "bsp_charger.c" "bsp_buzzer.c" "bsp_mic.c"
         "bsp_sdcard.c"
    INCLUDE_DIRS "include"
    PRIV_INCLUDE_DIRS "."
    REQUIRES freertos log esp_timer esp_driver_gpio esp_driver_spi esp_driver_i2c
             esp_driver_ledc esp_driver_i2s esp_adc fatfs sdmmc
)

idf_component_register(
    SRCS "bsp_io_expander.c" "bsp_camera.c" "bsp_display.c" "bsp_battery.c" "bsp_knob.c" "bsp_touch.c" "bsp_audio.c" "bsp_sdcard.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_adc esp_driver_gpio esp_driver_i2c esp_driver_ledc
             esp_driver_pcnt esp_driver_spi esp_driver_i2s esp_lcd espressif__esp_lcd_touch esp_lcd_touch_spd2010 sscma_client esp_codec_dev fatfs sdmmc
)

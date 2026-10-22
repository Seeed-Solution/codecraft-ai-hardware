idf_component_register(
    SRCS "bsp_io_expander.c" "bsp_camera.c" "bsp_display.c" "bsp_battery.c" "bsp_knob.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_adc esp_driver_gpio esp_driver_i2c esp_driver_ledc
             esp_driver_pcnt esp_driver_spi esp_lcd sscma_client
)

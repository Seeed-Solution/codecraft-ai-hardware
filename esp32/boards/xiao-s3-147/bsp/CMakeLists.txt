idf_component_register(
    SRCS "bsp_display.c" "bsp_button.c" "bsp_mic.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_lcd esp_driver_gpio esp_driver_spi esp_driver_i2s
)

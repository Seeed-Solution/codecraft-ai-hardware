idf_component_register(
    SRCS "bsp_epd.c" "bsp_button.c" "bsp_sht4x.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_timer esp_driver_gpio esp_driver_spi esp_driver_i2c
)

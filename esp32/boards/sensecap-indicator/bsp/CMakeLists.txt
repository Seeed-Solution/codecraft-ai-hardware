idf_component_register(
    SRCS "bsp_button.c" "bsp_display.c" "bsp_io_expander.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_driver_gpio esp_driver_i2c esp_lcd
)

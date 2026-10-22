idf_component_register(
    SRCS "bsp_io_expander.c" "bsp_camera.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_driver_i2c esp_driver_spi sscma_client
)

idf_component_register(
    SRCS "bsp_audio.c" "bsp_button.c" "bsp_led.c"
    INCLUDE_DIRS "include"
    REQUIRES freertos log esp_driver_gpio esp_driver_i2c esp_driver_i2s led_strip
)

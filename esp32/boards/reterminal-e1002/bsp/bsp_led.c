/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 */
#include "bsp_led.h"
#include "bsp_pins.h"

#include "driver/gpio.h"

esp_err_t bsp_led_init(void)
{
    const gpio_config_t cfg = {
        .pin_bit_mask = 1ULL << BSP_PIN_LED,
        .mode = GPIO_MODE_OUTPUT,
    };
    esp_err_t err = gpio_config(&cfg);
    bsp_led_set(false);
    return err;
}

void bsp_led_set(bool on)
{
    gpio_set_level(BSP_PIN_LED, on ? 0 : 1); /* active low */
}

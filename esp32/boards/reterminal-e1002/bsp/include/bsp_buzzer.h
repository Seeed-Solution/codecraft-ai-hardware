/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Onboard buzzer driven by LEDC PWM on GPIO 45 (Zephyr board dts:
 * pwm-buzzer on ledc0 channel 0). Useful for key-press feedback and
 * attention alerts on a device with a slow e-paper display.
 */
#pragma once

#include <stdint.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Configure the LEDC timer/channel. Silent after init. */
esp_err_t bsp_buzzer_init(void);

/* Start a continuous tone at freq_hz (audible range roughly 100-8000 Hz),
 * 50% duty. Call bsp_buzzer_stop() to silence. */
esp_err_t bsp_buzzer_tone(uint32_t freq_hz);

/* Silence the buzzer (duty 0). */
esp_err_t bsp_buzzer_stop(void);

/* Blocking convenience: tone for duration_ms, then stop. */
esp_err_t bsp_buzzer_beep(uint32_t freq_hz, uint32_t duration_ms);

#ifdef __cplusplus
}
#endif

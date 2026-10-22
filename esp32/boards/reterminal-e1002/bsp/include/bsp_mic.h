/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Onboard PDM microphone (I2S PDM RX: CLK GPIO 42, DATA GPIO 41, power
 * enable GPIO 38 per the vendor schematic). Intended for voice-agent
 * capture: mono 16-bit PCM at the requested sample rate.
 */
#pragma once

#include <stddef.h>
#include <stdint.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Power the mic rail and start an I2S PDM RX channel at sample_rate_hz
 * (typical: 16000 for speech). */
esp_err_t bsp_mic_init(uint32_t sample_rate_hz);

/* Read up to sample_count 16-bit mono samples. Blocks up to timeout_ms;
 * out_read receives the number of samples actually read. */
esp_err_t bsp_mic_read(int16_t *samples, size_t sample_count, size_t *out_read,
                       uint32_t timeout_ms);

/* Stop capture and power the mic rail down. */
esp_err_t bsp_mic_deinit(void);

#ifdef __cplusplus
}
#endif

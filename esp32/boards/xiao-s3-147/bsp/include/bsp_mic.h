/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Onboard PDM microphone (I2S0 PDM RX: CLK GPIO 1, DATA GPIO 2 — the only
 * PDM receiver on the ESP32-S3). Mono 16-bit PCM at the requested sample
 * rate; the board removes the PDM filter's DC offset in hardware.
 */
#pragma once

#include <stddef.h>
#include <stdint.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Start an I2S PDM RX channel at sample_rate_hz (typical: 16000). */
esp_err_t bsp_mic_init(uint32_t sample_rate_hz);

/* Read up to sample_count 16-bit mono samples. Blocks up to timeout_ms;
 * out_read receives the number of samples actually read. */
esp_err_t bsp_mic_read(int16_t *samples, size_t sample_count, size_t *out_read,
                       uint32_t timeout_ms);

/* Stop capture and release the channel. */
esp_err_t bsp_mic_deinit(void);

#ifdef __cplusplus
}
#endif

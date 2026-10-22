/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher audio: an ES8311 DAC (I2C0 @0x30) driving the speaker
 * and an ES7243/ES7243E ADC (I2C0 @0x13/@0x14, variant auto-detected) for
 * the microphone, both on I2S0 (16 kHz / 16-bit / mono, MCLK=GPIO10,
 * BCLK=GPIO11, WS=GPIO12, DOUT=GPIO16, DIN=GPIO15). The codec+amplifier
 * power rail is IO expander pin 12.
 * Uses esp_codec_dev; the returned handles are opened and ready for
 * esp_codec_dev_read()/esp_codec_dev_write().
 */
#pragma once

#include "esp_err.h"
#include "esp_codec_dev.h"

#ifdef __cplusplus
extern "C" {
#endif

#define BSP_AUDIO_SAMPLE_RATE 16000
#define BSP_AUDIO_SAMPLE_BITS 16
#define BSP_AUDIO_CHANNELS    1
#define BSP_AUDIO_MIC_GAIN_DB 27.0f /* vendor DRV_AUDIO_MIC_GAIN */

/* Powers the codec rail, brings up I2S0 duplex and opens both codec
 * devices (speaker out, mic in, gain BSP_AUDIO_MIC_GAIN_DB).
 * Safe to call more than once. */
esp_err_t bsp_audio_init(void);

/* Handles from a previous bsp_audio_init(), NULL before it. */
esp_codec_dev_handle_t bsp_audio_get_speaker(void);
esp_codec_dev_handle_t bsp_audio_get_mic(void);

/* Speaker volume 0-100. */
esp_err_t bsp_audio_set_volume(int percent);
esp_err_t bsp_audio_set_mute(bool mute);

#ifdef __cplusplus
}
#endif

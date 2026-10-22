/* reSpeaker Lite audio: XMOS mic processor + AIC3204 codec over I2C,
 * 16 kHz I2S slave. Derived from Meta muse-gadget-sdk (Apache 2.0),
 * see NOTICE. */
#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Initialize I2C (XMOS + codec), pulse the XMOS reset line, verify the XMOS
 * answers with its firmware version, then start the I2S slave channels.
 * Returns ESP_ERR_* if the XMOS does not answer — check that the XMOS runs
 * Seeed's 16 kHz I2S firmware v1.0.9 (ESP32 flashing does not update it). */
esp_err_t bsp_audio_init(void);

/* Codec digital gain, -63.5 dB..0 dB. Never boosts above unity. */
esp_err_t bsp_audio_set_volume(int percent);

/* Hardware mute state reported by the XMOS (firmware >= 1.0.9).
 * Returns true (fail closed) when the state cannot be read. */
bool bsp_audio_muted(void);

/* Microphone capture. Stereo 32-bit samples at 16 kHz; the XMOS processed
 * channel is what you want. Returns frames read (0 on timeout). */
esp_err_t bsp_audio_mic_start(void);
void      bsp_audio_mic_stop(void);
size_t    bsp_audio_mic_read(int32_t *stereo_out, size_t frames, uint32_t timeout_ms);

/* Speaker playback: interleaved stereo 32-bit samples at 16 kHz. */
size_t    bsp_audio_write(const int32_t *stereo_in, size_t frames, uint32_t timeout_ms);

#ifdef __cplusplus
}
#endif

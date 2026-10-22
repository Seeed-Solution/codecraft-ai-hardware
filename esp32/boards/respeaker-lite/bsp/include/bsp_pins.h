/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reSpeaker Lite (XIAO ESP32-S3) pin map — the board's source of truth.
 *
 * Sources, highest priority first (see docs/vendor-sources.md):
 *  1. Vendor: respeaker/reSpeaker_Lite — README (I2C 0x42; D4=SDA, D5=SCL)
 *     and xiao_i2c_control_volume example (AIC3204 at 0x18; I2S slave,
 *     bck=8, ws=7, dout=43, din=44; Wire.begin(5, 6)).
 *  2. Field-verified: Meta muse-gadget-sdk main/voice_board_respeaker_lite.c
 *     and main/led_status.c (Apache 2.0).
 *  3. XIAO ESP32-S3 D-pin mapping: Seeed wiki
 *     (D0=GPIO1, D1=GPIO2, D4=GPIO5, D5=GPIO6, D6=GPIO43, D7=GPIO44,
 *      D8=GPIO7, D9=GPIO8).
 */
#pragma once

#define BSP_PIN_WS2812_DATA   1   /* XIAO D0; single WS2812, no power switch */
#define BSP_PIN_XMOS_RESET    2   /* XIAO D1; pulse 20 ms high, then 3 s boot */
#define BSP_PIN_I2C_SDA       5   /* XIAO D4 */
#define BSP_PIN_I2C_SCL       6   /* XIAO D5 */
#define BSP_PIN_I2S_WS        7   /* XIAO D8 */
#define BSP_PIN_I2S_BCLK      8   /* XIAO D9 */
#define BSP_PIN_I2S_DOUT      43  /* XIAO D6; ESP32 -> AIC3204 DAC */
#define BSP_PIN_I2S_DIN       44  /* XIAO D7; XMOS mic -> ESP32 */
#define BSP_PIN_BUTTON_BOOT   0   /* XIAO BOOT button, active low */

#define BSP_I2C_ADDR_XMOS     0x42
#define BSP_I2C_ADDR_CODEC    0x18  /* AIC3204 */
#define BSP_I2C_FREQ_HZ       100000

#define BSP_I2S_PORT          0
#define BSP_I2S_SAMPLE_RATE   16000 /* XMOS 16 kHz I2S firmware v1.0.9 */

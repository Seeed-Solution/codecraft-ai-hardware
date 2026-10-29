/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal Sticky e-paper panel: 3.97" SSD1677, native 800x480, used in
 * portrait at 480x800.
 *
 * Mono framebuffer: 1 bit per pixel, 8 pixels per byte, MSB is the
 * leftmost pixel of the logical row, 1 = white, 0 = black.
 * Gray4 framebuffer: 2 bits per pixel packed MSB first (four pixels per
 * byte), 0 = black ... 3 = white.
 *
 * The panel power rail (BSP_PIN_EPD_EN) stays on between updates; the
 * driver holds refresh state so fast updates can follow full ones.
 */
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "esp_err.h"

#define BSP_EPD_WIDTH            480
#define BSP_EPD_HEIGHT           800
#define BSP_EPD_ROW_BYTES        (BSP_EPD_WIDTH / 8)
#define BSP_EPD_FRAME_BYTES      (BSP_EPD_ROW_BYTES * BSP_EPD_HEIGHT)
#define BSP_EPD_GRAY4_ROW_BYTES  (BSP_EPD_WIDTH / 4)
#define BSP_EPD_GRAY4_FRAME_BYTES (BSP_EPD_GRAY4_ROW_BYTES * BSP_EPD_HEIGHT)

#ifdef __cplusplus
extern "C" {
#endif

/* Claim GPIOs and the SPI2 bus. Call once. */
esp_err_t bsp_epd_init(void);

/* Show `frame`. `prev` is what the panel currently shows (pass NULL on the
 * first update after boot or after a failed update; it forces a full
 * refresh). `full` selects a full (slow, clean) refresh over a fast one.
 * Blocks for the refresh: a few seconds for full. */
esp_err_t bsp_epd_update(const uint8_t *frame, const uint8_t *prev, bool full);

/* Show a 4-gray image (2 bpp packed frame). Always a full grayscale
 * refresh; the next mono update afterwards must be full. */
esp_err_t bsp_epd_update_gray4(const uint8_t *frame);

#ifdef __cplusplus
}
#endif

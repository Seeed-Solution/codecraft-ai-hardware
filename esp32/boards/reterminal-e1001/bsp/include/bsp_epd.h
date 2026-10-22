/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal E1001 e-paper panel: 7.5" 800x480 monochrome, GDEY075T7
 * with a UC8179 controller.
 *
 * Framebuffer format: 1 bit per pixel, 8 pixels per byte, MSB is the
 * leftmost pixel, 1 = white, 0 = black (the controller's order).
 *
 * The panel is put into deep sleep at the end of every update, so the
 * driver holds no state between refreshes.
 */
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "esp_err.h"

#define BSP_EPD_WIDTH       800
#define BSP_EPD_HEIGHT      480
#define BSP_EPD_ROW_BYTES   (BSP_EPD_WIDTH / 8)
#define BSP_EPD_FRAME_BYTES (BSP_EPD_ROW_BYTES * BSP_EPD_HEIGHT)

#ifdef __cplusplus
extern "C" {
#endif

/* Claim GPIOs and the SPI bus. Call once. */
esp_err_t bsp_epd_init(void);

/* Show `frame`. `prev` is what the panel currently shows (pass NULL on the
 * first update after boot); `full` selects a full (slow, clean) refresh over
 * a fast one. Blocks for the refresh: a few seconds. On success the panel
 * is back in deep sleep. */
esp_err_t bsp_epd_update(const uint8_t *frame, const uint8_t *prev, bool full);

#ifdef __cplusplus
}
#endif

/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal E1002 e-paper panel: 7.3" 800x480 E Ink Spectra 6
 * (GDEP073E01), six inks, full refreshes only (~30 s).
 *
 * Framebuffer format: 4 bits per pixel, 2 pixels per byte, the left pixel
 * in the high nibble (the controller's order).
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
#define BSP_EPD_ROW_BYTES   (BSP_EPD_WIDTH / 2)
#define BSP_EPD_FRAME_BYTES (BSP_EPD_ROW_BYTES * BSP_EPD_HEIGHT)

/* Spectra 6 ink codes (muse epaper_status.c; Zephyr color-palette agrees). */
typedef enum {
    BSP_EPD_INK_BLACK  = 0,
    BSP_EPD_INK_WHITE  = 1,
    BSP_EPD_INK_YELLOW = 2,
    BSP_EPD_INK_RED    = 3,
    BSP_EPD_INK_BLUE   = 4,
    BSP_EPD_INK_GREEN  = 5,
} bsp_epd_ink_t;

#ifdef __cplusplus
extern "C" {
#endif

/* Claim GPIOs and the SPI bus. Call once. */
esp_err_t bsp_epd_init(void);

/* Show `frame`. The panel supports full refreshes only; `prev` and `full`
 * are accepted for interface parity with the mono panel and ignored.
 * Blocks for the refresh: tens of seconds. On success the panel is back
 * in deep sleep. */
esp_err_t bsp_epd_update(const uint8_t *frame, const uint8_t *prev, bool full);

/* Pack one row of ink codes into the framebuffer. */
static inline void bsp_epd_set_pixel(uint8_t *fb, int x, int y, bsp_epd_ink_t ink)
{
    uint8_t *byte = fb + (size_t)y * BSP_EPD_ROW_BYTES + x / 2;
    if (x & 1) *byte = (uint8_t)((*byte & 0xF0) | (uint8_t)ink);
    else       *byte = (uint8_t)((*byte & 0x0F) | ((uint8_t)ink << 4));
}

#ifdef __cplusplus
}
#endif

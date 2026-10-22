/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher camera: a Himax WiseEye2 vision coprocessor that speaks
 * Seeed's SSCMA protocol over SPI2. Its power rail, sync line and reset all
 * sit on the PCA9535 expander, so bsp_camera_init() brings up I2C0 and the
 * expander first. The driver itself is the repository-shared sscma_client
 * component; this file only wires the board's lines into it.
 */
#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Powers the system rail, points the expander's camera lines the right way
 * and registers the Himax as the board's camera (camera_get()). The chip
 * itself stays powered off until the first camera_capture(): it is reset by
 * power-cycling its rail, as muse's shipping Watcher firmware does, so the
 * expander's reset pin (7) stays an input.
 * Safe to call more than once. */
esp_err_t bsp_camera_init(void);

#ifdef __cplusplus
}
#endif

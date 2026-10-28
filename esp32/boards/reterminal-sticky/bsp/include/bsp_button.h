/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal Sticky keys: AI (GPIO 4), Up (GPIO 5), Down (GPIO 6), all
 * active low with internal pull-ups. Callbacks run in the BSP button task
 * and must stay non-blocking.
 */
#pragma once

#include <stdbool.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    BSP_BUTTON_AI = 0,
    BSP_BUTTON_UP,
    BSP_BUTTON_DOWN,
    BSP_BUTTON_COUNT,
} bsp_button_id_t;

typedef void (*bsp_button_cb_t)(bsp_button_id_t id, bool pressed, void *arg);

/* Poll the keys with debounce in a dedicated task. */
esp_err_t bsp_button_init(bsp_button_cb_t cb, void *arg);

/* Level without a callback, true while held. */
bool bsp_button_pressed(bsp_button_id_t id);

#ifdef __cplusplus
}
#endif

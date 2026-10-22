/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal E1001 top keys: green ("Refresh"), Left, Right. All active
 * low with internal pull-ups. Polled with debounce in a dedicated task;
 * callbacks run in that task's context and must stay non-blocking.
 */
#pragma once

#include <stdbool.h>

#include "esp_err.h"

typedef enum {
    BSP_KEY_GREEN = 0,
    BSP_KEY_LEFT,
    BSP_KEY_RIGHT,
    BSP_KEY_COUNT,
} bsp_key_t;

typedef void (*bsp_button_cb_t)(bsp_key_t key, bool pressed, void *arg);

#ifdef __cplusplus
extern "C" {
#endif

esp_err_t bsp_button_init(bsp_button_cb_t cb, void *arg);
bool bsp_button_pressed(bsp_key_t key);

#ifdef __cplusplus
}
#endif

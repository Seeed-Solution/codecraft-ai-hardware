/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher knob: a quadrature wheel on GPIO41/42 decoded by the
 * pulse counter, with its push button on IO expander pin 3 (active low).
 */
#pragma once

#include <stdbool.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Sets up the PCNT decoder with a 1 us glitch filter and the button line.
 * Safe to call more than once. */
esp_err_t bsp_knob_init(void);

/* Encoder counts since the last call, signed (positive = one way, negative =
 * the other; which is which is TBD on hardware). The raw count advances
 * several steps per detent — divide by the encoder's steps-per-detent if you
 * want detents. */
esp_err_t bsp_knob_get_delta(int *out_delta);

/* True while the knob is pushed. */
bool bsp_knob_button_pressed(void);

#ifdef __cplusplus
}
#endif

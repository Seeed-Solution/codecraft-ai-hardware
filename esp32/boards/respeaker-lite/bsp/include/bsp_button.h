/* reSpeaker Lite button: XIAO BOOT on GPIO 0, active low.
 * Callbacks run in the BSP button task and must stay non-blocking. */
#pragma once

#include <stdbool.h>
#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*bsp_button_cb_t)(bool pressed, void *arg);

/* Polls the button with debounce in a dedicated task. */
esp_err_t bsp_button_init(bsp_button_cb_t cb);

/* Level without a callback, true while held. */
bool bsp_button_pressed(void);

#ifdef __cplusplus
}
#endif

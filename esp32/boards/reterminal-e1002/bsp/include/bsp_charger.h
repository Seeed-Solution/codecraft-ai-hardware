/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Onboard Silergy SY6974B single-cell Li-Po charger on I2C1 (address 0x6B).
 * Charging itself is autonomous (hardware CC/CV); this driver exists to
 * report charge state to the UI and to tune the charge current/voltage.
 */
#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    BSP_CHARGER_STATUS_DISCHARGING = 0, /* no valid input power */
    BSP_CHARGER_STATUS_NOT_CHARGING,    /* powered, but not charging */
    BSP_CHARGER_STATUS_PRECHARGE,       /* deeply discharged battery */
    BSP_CHARGER_STATUS_FAST_CHARGE,
    BSP_CHARGER_STATUS_FULL,
} bsp_charger_status_t;

/* Probe the part ID, reset to defaults, disable the I2C watchdog (so the
 * configuration persists without periodic host traffic) and apply the
 * vendor board settings: 1000 mA charge current (0.5C of the 2000 mAh
 * battery), 4208 mV charge voltage. Returns ESP_ERR_NOT_FOUND when the
 * chip does not answer or reports a wrong part ID. */
esp_err_t bsp_charger_init(void);

/* Aggregate charge state for UI display. */
esp_err_t bsp_charger_get_status(bsp_charger_status_t *out_status);

/* True when a valid input source is present (REG0A BUS_GD). */
esp_err_t bsp_charger_is_input_present(bool *out_present);

/* Enable/disable charging (REG01 CHG_CONFIG). */
esp_err_t bsp_charger_set_charge_enabled(bool enabled);

/* Constant-charge current in mA. Set clamps to 0-3000 mA in 60 mA steps. */
esp_err_t bsp_charger_get_charge_current_ma(uint16_t *out_ma);
esp_err_t bsp_charger_set_charge_current_ma(uint16_t ma);

/* Raw fault register (REG09) for diagnostics; 0 means no faults. Bit
 * layout: [7] watchdog, [6] boost, [5:4] charge fault, [3] battery OVP,
 * [2:0] NTC fault. */
esp_err_t bsp_charger_get_faults(uint8_t *out_fault);

#ifdef __cplusplus
}
#endif

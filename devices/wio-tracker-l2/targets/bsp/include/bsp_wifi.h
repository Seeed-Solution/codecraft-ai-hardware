// SPDX-License-Identifier: Apache-2.0
#pragma once
#include <stdint.h>
#include "esp_err.h"

// Blocking passive scan on an already started station. The caller owns the
// Wi-Fi lifecycle and serializes scan/connect operations on that station.
// Writes the AP count only on success. Retains no AP identifiers.
esp_err_t bsp_wifi_scan_count(uint16_t *count);

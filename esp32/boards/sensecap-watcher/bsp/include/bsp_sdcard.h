/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SenseCAP Watcher microSD slot: second device on the shared SPI2 bus
 * (CS=GPIO46, the Himax camera is the other device), card-detect on IO
 * expander pin 4 (low = inserted), power rail on expander pin 8.
 * The SSCMA camera driver tolerates an already-initialized SPI2 bus and
 * vice versa; max_transfer_sz is kept >= 4096 so either may init first.
 */
#pragma once

#include <stdbool.h>

#include "esp_err.h"
#include "sdmmc_cmd.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Powers the SD rail, ensures the shared SPI2 bus exists and mounts the
 * FAT partition at mount_point (e.g. "/sdcard"). Safe to call more than
 * once. The card handle is returned via out_card (may be NULL). */
esp_err_t bsp_sdcard_init(const char *mount_point, sdmmc_card_t **out_card);

/* true when a card is physically inserted (expander pin 4, active low). */
esp_err_t bsp_sdcard_is_inserted(bool *inserted);

/* Unmounts and releases the slot; the rail is left powered. */
esp_err_t bsp_sdcard_deinit(const char *mount_point);

#ifdef __cplusplus
}
#endif

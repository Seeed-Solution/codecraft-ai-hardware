/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal E1002 microSD slot (J5, ST-TF-003J), wired for SPI mode:
 *   CLOCK = GPIO 7, CMD/MOSI = GPIO 9, DAT0/MISO = GPIO 8, CD/DAT3/CS = GPIO 14.
 * The bus is shared with the e-paper panel (CS = GPIO 10); bsp_epd also
 * routes MISO so either device may initialise the bus first.
 * Card power rail TF_CARD_PWR is switched by a TPS22916 load switch (U13),
 * enable GPIO 16 active high. Card-detect switch on GPIO 15 shorts to GND
 * when a card is inserted (active low, internal pull-up).
 * Facts from the vendor schematic E1001 V1.2, sheet 06 Peripherals
 * (CC BY-SA 4.0); the E1002 shares the E1001 mainboard
 * (see docs/vendor-sources.md).
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

/* true when a card is physically inserted (GPIO 15, active low). */
esp_err_t bsp_sdcard_is_inserted(bool *inserted);

/* Unmounts and releases the slot; the rail is left powered. */
esp_err_t bsp_sdcard_deinit(const char *mount_point);

#ifdef __cplusplus
}
#endif

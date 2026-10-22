/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal E1002 SD card example: powers the card rail, mounts the slot
 * on SPI2 (shared with the e-paper panel), writes and reads back a file,
 * prints card info, unmounts. Insert a FAT-formatted microSD card before
 * running.
 */

#include "bsp_sdcard.h"

#include <stdio.h>
#include <string.h>

#include "esp_log.h"
#include "sdmmc_cmd.h"

static const char *TAG = "sdcard_test";
#define MOUNT "/sdcard"

void app_main(void)
{
    bool inserted = false;
    ESP_ERROR_CHECK(bsp_sdcard_is_inserted(&inserted));
    if (!inserted) {
        ESP_LOGW(TAG, "no card inserted; halting");
        return;
    }

    sdmmc_card_t *card = NULL;
    ESP_ERROR_CHECK(bsp_sdcard_init(MOUNT, &card));
    sdmmc_card_print_info(stdout, card);

    const char *path = MOUNT "/hello.txt";
    const char *msg = "codecraft-ai-hardware: hello from reTerminal E1002\n";
    FILE *f = fopen(path, "w");
    if (!f) {
        ESP_LOGE(TAG, "open for write failed");
        return;
    }
    fwrite(msg, 1, strlen(msg), f);
    fclose(f);

    char buf[128] = {0};
    f = fopen(path, "r");
    if (f) {
        size_t n = fread(buf, 1, sizeof(buf) - 1, f);
        fclose(f);
        ESP_LOGI(TAG, "read back %d bytes: %s", (int)n, buf);
    }

    ESP_ERROR_CHECK(bsp_sdcard_deinit(MOUNT));
    ESP_LOGI(TAG, "unmounted; done");
}

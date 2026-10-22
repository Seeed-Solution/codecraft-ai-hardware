/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Takes one JPEG with the Watcher's Himax camera and logs what came back.
 * The camera is powered only for the capture; between runs the AI-chip rail
 * on the IO expander is off.
 */
#include <stdio.h>

#include "bsp_camera.h"
#include "camera.h"
#include "esp_log.h"

static const char *TAG = "camera_capture";

void app_main(void)
{
    ESP_ERROR_CHECK(bsp_camera_init());

    const camera_driver_t *cam = camera_get();
    ESP_LOGI(TAG, "camera: %s (up to %dx%d)", cam->name, cam->max_width, cam->max_height);

    camera_frame_t frame;
    esp_err_t err = camera_capture(&frame);
    if (err != ESP_OK) {
        ESP_LOGE(TAG, "capture failed: %s", esp_err_to_name(err));
        return;
    }
    ESP_LOGI(TAG, "JPEG %ux%u, %u bytes, SOI %02x%02x", frame.width, frame.height,
             (unsigned)frame.len, frame.jpeg[0], frame.jpeg[1]);
    camera_release(&frame);
}

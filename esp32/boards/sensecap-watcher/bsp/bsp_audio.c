/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * ES8311 DAC + ES7243/ES7243E ADC over esp_codec_dev on I2S0.
 * Pin/address/gain facts from the vendor BSP (SenseCAP-Watcher-Firmware
 * components/sensecap-watcher, Apache 2.0); reimplemented on the IDF v6
 * i2c_master/i2s_std drivers with esp_codec_dev 1.6.x bus_handle support.
 */

#include "bsp_audio.h"

#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include <stdbool.h>

#include "driver/i2c_master.h"
#include "driver/i2s_std.h"
#include "esp_check.h"
#include "esp_codec_dev_defaults.h"
#include "esp_log.h"

static const char *TAG = "bsp_audio";

#define BSP_I2C_ADDR_ES8311  0x30 /* speaker DAC */
#define BSP_I2C_ADDR_ES7243  0x13 /* mic ADC */
#define BSP_I2C_ADDR_ES7243E 0x14 /* mic ADC, E variant */

static i2s_chan_handle_t s_tx_chan;
static i2s_chan_handle_t s_rx_chan;
static const audio_codec_data_if_t *s_data_if;
static esp_codec_dev_handle_t s_speaker;
static esp_codec_dev_handle_t s_mic;

static esp_err_t audio_i2s_init(void)
{
    if (s_tx_chan && s_rx_chan) {
        return ESP_OK;
    }
    i2s_chan_config_t chan_cfg = I2S_CHANNEL_DEFAULT_CONFIG(I2S_NUM_0, I2S_ROLE_MASTER);
    ESP_RETURN_ON_ERROR(i2s_new_channel(&chan_cfg, &s_tx_chan, &s_rx_chan), TAG,
                        "i2s channel");

    i2s_std_config_t std_cfg = {
        .clk_cfg = I2S_STD_CLK_DEFAULT_CONFIG(BSP_AUDIO_SAMPLE_RATE),
        .slot_cfg = I2S_STD_PHILIPS_SLOT_DEFAULT_CONFIG(I2S_DATA_BIT_WIDTH_16BIT,
                                                        I2S_SLOT_MODE_MONO),
        .gpio_cfg = {
            .mclk = BSP_PIN_I2S_MCLK,
            .bclk = BSP_PIN_I2S_BCLK,
            .ws = BSP_PIN_I2S_WS,
            .dout = BSP_PIN_I2S_DOUT,
            .din = BSP_PIN_I2S_DIN,
            .invert_flags = { .mclk_inv = false, .bclk_inv = false, .ws_inv = false },
        },
    };
    ESP_RETURN_ON_ERROR(i2s_channel_init_std_mode(s_tx_chan, &std_cfg), TAG, "tx init");
    ESP_RETURN_ON_ERROR(i2s_channel_enable(s_tx_chan), TAG, "tx enable");

    /* The mic only answers on the right slot (vendor BSP). */
    std_cfg.slot_cfg.slot_mask = I2S_STD_SLOT_RIGHT;
    ESP_RETURN_ON_ERROR(i2s_channel_init_std_mode(s_rx_chan, &std_cfg), TAG, "rx init");
    ESP_RETURN_ON_ERROR(i2s_channel_enable(s_rx_chan), TAG, "rx enable");

    audio_codec_i2s_cfg_t i2s_cfg = {
        .port = 0,
        .rx_handle = s_rx_chan,
        .tx_handle = s_tx_chan,
    };
    s_data_if = audio_codec_new_i2s_data(&i2s_cfg);
    ESP_RETURN_ON_FALSE(s_data_if != NULL, ESP_FAIL, TAG, "i2s data if");
    return ESP_OK;
}

static esp_err_t audio_speaker_init(i2c_master_bus_handle_t bus)
{
    const audio_codec_gpio_if_t *gpio_if = audio_codec_new_gpio();
    ESP_RETURN_ON_FALSE(gpio_if != NULL, ESP_FAIL, TAG, "gpio if");

    audio_codec_i2c_cfg_t i2c_cfg = {
        .addr = BSP_I2C_ADDR_ES8311,
        .bus_handle = bus,
    };
    const audio_codec_ctrl_if_t *ctrl_if = audio_codec_new_i2c_ctrl(&i2c_cfg);
    ESP_RETURN_ON_FALSE(ctrl_if != NULL, ESP_FAIL, TAG, "es8311 ctrl if");

    esp_codec_dev_hw_gain_t gain = {
        .pa_voltage = 5.0f,        /* vendor BSP: PA rail is 5 V */
        .codec_dac_voltage = 3.3f,
    };
    es8311_codec_cfg_t es8311_cfg = {
        .ctrl_if = ctrl_if,
        .gpio_if = gpio_if,
        .codec_mode = ESP_CODEC_DEV_WORK_MODE_DAC,
        .pa_pin = GPIO_NUM_NC, /* PA power is the expander rail, not a GPIO */
        .pa_reverted = false,
        .master_mode = false,
        .use_mclk = true,
        .digital_mic = false,
        .invert_mclk = false,
        .invert_sclk = false,
        .hw_gain = gain,
    };
    const audio_codec_if_t *codec_if = es8311_codec_new(&es8311_cfg);
    ESP_RETURN_ON_FALSE(codec_if != NULL, ESP_FAIL, TAG, "es8311 codec");

    esp_codec_dev_cfg_t dev_cfg = {
        .dev_type = ESP_CODEC_DEV_TYPE_OUT,
        .codec_if = codec_if,
        .data_if = s_data_if,
    };
    s_speaker = esp_codec_dev_new(&dev_cfg);
    ESP_RETURN_ON_FALSE(s_speaker != NULL, ESP_FAIL, TAG, "speaker dev");
    return ESP_OK;
}

static esp_err_t audio_mic_init(i2c_master_bus_handle_t bus)
{
    /* Two board revisions exist: ES7243 @0x13 or ES7243E @0x14 (vendor BSP
     * probes in this order). */
    const audio_codec_if_t *codec_if = NULL;
    if (i2c_master_probe(bus, BSP_I2C_ADDR_ES7243, 100) == ESP_OK) {
        audio_codec_i2c_cfg_t i2c_cfg = {
            .addr = BSP_I2C_ADDR_ES7243,
            .bus_handle = bus,
        };
        const audio_codec_ctrl_if_t *ctrl_if = audio_codec_new_i2c_ctrl(&i2c_cfg);
        ESP_RETURN_ON_FALSE(ctrl_if != NULL, ESP_FAIL, TAG, "es7243 ctrl if");
        es7243_codec_cfg_t cfg = { .ctrl_if = ctrl_if };
        codec_if = es7243_codec_new(&cfg);
        ESP_LOGI(TAG, "mic ADC: ES7243 @0x%02x", BSP_I2C_ADDR_ES7243);
    } else {
        audio_codec_i2c_cfg_t i2c_cfg = {
            .addr = BSP_I2C_ADDR_ES7243E,
            .bus_handle = bus,
        };
        const audio_codec_ctrl_if_t *ctrl_if = audio_codec_new_i2c_ctrl(&i2c_cfg);
        ESP_RETURN_ON_FALSE(ctrl_if != NULL, ESP_FAIL, TAG, "es7243e ctrl if");
        es7243e_codec_cfg_t cfg = { .ctrl_if = ctrl_if };
        codec_if = es7243e_codec_new(&cfg);
        ESP_LOGI(TAG, "mic ADC: ES7243E @0x%02x", BSP_I2C_ADDR_ES7243E);
    }
    ESP_RETURN_ON_FALSE(codec_if != NULL, ESP_FAIL, TAG, "mic codec");

    esp_codec_dev_cfg_t dev_cfg = {
        .dev_type = ESP_CODEC_DEV_TYPE_IN,
        .codec_if = codec_if,
        .data_if = s_data_if,
    };
    s_mic = esp_codec_dev_new(&dev_cfg);
    ESP_RETURN_ON_FALSE(s_mic != NULL, ESP_FAIL, TAG, "mic dev");
    return ESP_OK;
}

esp_err_t bsp_audio_init(void)
{
    if (s_speaker && s_mic) {
        return ESP_OK;
    }

    /* Codec + amplifier rail. */
    ESP_RETURN_ON_ERROR(bsp_io_expander_init(), TAG, "expander");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_direction(BSP_EXP_PIN_PWR_CODEC_PA, 1),
                        TAG, "codec rail dir");
    ESP_RETURN_ON_ERROR(bsp_io_expander_set_level(BSP_EXP_PIN_PWR_CODEC_PA, 1),
                        TAG, "codec rail on");

    i2c_master_bus_handle_t bus = NULL;
    ESP_RETURN_ON_ERROR(bsp_i2c0_get_bus(&bus), TAG, "i2c0 bus");

    ESP_RETURN_ON_ERROR(audio_i2s_init(), TAG, "i2s");
    ESP_RETURN_ON_ERROR(audio_speaker_init(bus), TAG, "speaker");
    ESP_RETURN_ON_ERROR(audio_mic_init(bus), TAG, "mic");

    esp_codec_dev_sample_info_t fs = {
        .bits_per_sample = BSP_AUDIO_SAMPLE_BITS,
        .channel = BSP_AUDIO_CHANNELS,
        .sample_rate = BSP_AUDIO_SAMPLE_RATE,
    };
    ESP_RETURN_ON_FALSE(esp_codec_dev_open(s_speaker, &fs) == ESP_CODEC_DEV_OK,
                        ESP_FAIL, TAG, "open speaker");
    ESP_RETURN_ON_FALSE(esp_codec_dev_open(s_mic, &fs) == ESP_CODEC_DEV_OK,
                        ESP_FAIL, TAG, "open mic");
    ESP_RETURN_ON_FALSE(esp_codec_dev_set_in_gain(s_mic, BSP_AUDIO_MIC_GAIN_DB) ==
                            ESP_CODEC_DEV_OK,
                        ESP_FAIL, TAG, "mic gain");

    ESP_LOGI(TAG, "audio ready: %d Hz / %d bit / %d ch", BSP_AUDIO_SAMPLE_RATE,
             BSP_AUDIO_SAMPLE_BITS, BSP_AUDIO_CHANNELS);
    return ESP_OK;
}

esp_codec_dev_handle_t bsp_audio_get_speaker(void)
{
    return s_speaker;
}

esp_codec_dev_handle_t bsp_audio_get_mic(void)
{
    return s_mic;
}

esp_err_t bsp_audio_set_volume(int percent)
{
    ESP_RETURN_ON_FALSE(s_speaker != NULL, ESP_ERR_INVALID_STATE, TAG, "init first");
    return esp_codec_dev_set_out_vol(s_speaker, percent) == ESP_CODEC_DEV_OK
               ? ESP_OK
               : ESP_FAIL;
}

esp_err_t bsp_audio_set_mute(bool mute)
{
    ESP_RETURN_ON_FALSE(s_speaker != NULL, ESP_ERR_INVALID_STATE, TAG, "init first");
    return esp_codec_dev_set_out_mute(s_speaker, mute) == ESP_CODEC_DEV_OK
               ? ESP_OK
               : ESP_FAIL;
}

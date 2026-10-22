/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * PCF8563 driver written for this BSP. Register map (time/date block at
 * 0x02, BCD encoding, VL integrity flag) cross-checked against:
 *  - Zephyr drivers/rtc/rtc_pcf8563.c (Apache-2.0)
 *  - Zephyr board dts boards/seeed/reterminal_e1001 (chip at I2C0 0x51)
 *  - NXP PCF8563 datasheet (public register specification)
 */
#include "bsp_rtc.h"
#include "bsp_i2c0_internal.h"
#include "bsp_pins.h"

#include <stdint.h>

#include "driver/i2c_master.h"

/* Register addresses. */
#define PCF8563_REG_TIME_DATE 0x02 /* sec, min, hour, day, wday, month, year */

/* Field masks (unused bits read back as garbage — always mask). */
#define PCF8563_VL_FLAG      0x80 /* seconds reg bit 7: clock integrity low */
#define PCF8563_SECONDS_MASK 0x7F
#define PCF8563_MINUTES_MASK 0x7F
#define PCF8563_HOURS_MASK   0x3F /* 24-hour mode */
#define PCF8563_DAYS_MASK    0x3F
#define PCF8563_WEEKDAYS_MASK 0x07
#define PCF8563_MONTHS_MASK  0x1F

static i2c_master_dev_handle_t s_dev;

static uint8_t bin2bcd(uint8_t v)
{
    return (uint8_t)(((v / 10) << 4) | (v % 10));
}

static uint8_t bcd2bin(uint8_t v)
{
    return (uint8_t)((v >> 4) * 10 + (v & 0x0F));
}

esp_err_t bsp_rtc_init(void)
{
    i2c_master_bus_handle_t bus;
    esp_err_t err = bsp_i2c0_bus_get(&bus);
    if (err != ESP_OK) {
        return err;
    }
    if (i2c_master_probe(bus, BSP_I2C_ADDR_RTC, 100) != ESP_OK) {
        return ESP_ERR_NOT_FOUND;
    }
    i2c_device_config_t dev_cfg = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = BSP_I2C_ADDR_RTC,
        .scl_speed_hz = BSP_I2C0_FREQ_HZ,
    };
    return i2c_master_bus_add_device(bus, &dev_cfg, &s_dev);
}

esp_err_t bsp_rtc_get_time(struct tm *out_time)
{
    uint8_t reg = PCF8563_REG_TIME_DATE;
    uint8_t raw[7];
    esp_err_t err = i2c_master_transmit_receive(s_dev, &reg, 1, raw, sizeof(raw), 100);
    if (err != ESP_OK) {
        return err;
    }
    if (raw[0] & PCF8563_VL_FLAG) {
        return ESP_ERR_INVALID_STATE; /* oscillator stopped; time is garbage */
    }
    out_time->tm_sec = bcd2bin(raw[0] & PCF8563_SECONDS_MASK);
    out_time->tm_min = bcd2bin(raw[1] & PCF8563_MINUTES_MASK);
    out_time->tm_hour = bcd2bin(raw[2] & PCF8563_HOURS_MASK);
    out_time->tm_mday = bcd2bin(raw[3] & PCF8563_DAYS_MASK);
    out_time->tm_wday = raw[4] & PCF8563_WEEKDAYS_MASK;
    out_time->tm_mon = bcd2bin(raw[5] & PCF8563_MONTHS_MASK) - 1;
    out_time->tm_year = bcd2bin(raw[6]) + 100; /* century bit unused: 20xx */
    out_time->tm_yday = -1;
    out_time->tm_isdst = -1;
    return ESP_OK;
}

esp_err_t bsp_rtc_set_time(const struct tm *time)
{
    int year = time->tm_year + 1900;
    if (year < 2000 || year > 2099) {
        return ESP_ERR_INVALID_ARG;
    }
    uint8_t buf[8];
    buf[0] = PCF8563_REG_TIME_DATE;
    buf[1] = bin2bcd((uint8_t)time->tm_sec);
    buf[2] = bin2bcd((uint8_t)time->tm_min);
    buf[3] = bin2bcd((uint8_t)time->tm_hour);
    buf[4] = bin2bcd((uint8_t)time->tm_mday);
    buf[5] = (uint8_t)time->tm_wday;
    buf[6] = bin2bcd((uint8_t)(time->tm_mon + 1)); /* century bit clear */
    buf[7] = bin2bcd((uint8_t)(year - 2000));
    return i2c_master_transmit(s_dev, buf, sizeof(buf), 100);
}

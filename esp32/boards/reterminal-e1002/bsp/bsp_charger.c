/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * SY6974B driver written for this BSP. Register map, part-ID check and the
 * reset/disable-watchdog init sequence follow Zephyr
 * drivers/charger/charger_sy6974b.c (Apache-2.0); board defaults (1000 mA,
 * 4208 mV) come from the vendor-contributed Zephyr board definition
 * boards/seeed/reterminal_e1001/reterminal_e1001_procpu.dts (Apache-2.0).
 * Note: the community ESPHome SY6974 driver places CHRG_STAT at
 * REG08[1:0]; we follow the Zephyr upstream ([4:3]).
 */
#include "bsp_charger.h"
#include "bsp_pins.h"

#include "driver/i2c_master.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

/* Register map (Zephyr charger_sy6974b.c). */
#define REG01_POC      0x01 /* power-on configuration */
#define REG02_CCC      0x02 /* charge current control */
#define REG04_CVC      0x04 /* charge voltage control */
#define REG05_COC      0x05 /* charge operation control */
#define REG08_STATUS   0x08 /* system status (RO) */
#define REG09_FAULT    0x09 /* fault status (RO) */
#define REG0A_VIC      0x0A /* VINDPM/IINDPM + interrupt mask */
#define REG0B_ID       0x0B /* part/revision ID + reset */

#define REG01_CHG_CONFIG      (1u << 4)
#define REG02_ICHG_MASK       0x3F
#define REG04_VREG_MASK       0xF8
#define REG04_VREG_SHIFT      3
#define REG05_WATCHDOG_MASK   0x30
#define REG08_CHRG_STAT_MASK  0x18
#define REG08_CHRG_STAT_SHIFT 3
#define REG08_PG_STAT         (1 << 2)
#define REG0A_BUS_GD          (1 << 7)
#define REG0B_REG_RST         (1 << 7)
#define REG0B_PN_MASK         0x78
#define REG0B_PN_VAL          (0x8u << 3)

#define ICHG_STEP_MA   60u
#define ICHG_MAX_MA    3000u
#define VREG_BASE_MV   3856u
#define VREG_STEP_MV   32u
#define VREG_MAX_MV    4624u

/* Vendor board settings (Zephyr dts, see header). */
#define BOARD_CHG_CURRENT_MA 1000u /* 0.5C of the 2000 mAh battery */
#define BOARD_CHG_VOLTAGE_MV 4208u

static i2c_master_bus_handle_t s_bus;
static i2c_master_dev_handle_t s_dev;

static esp_err_t read_reg(uint8_t reg, uint8_t *out)
{
    return i2c_master_transmit_receive(s_dev, &reg, 1, out, 1, 100);
}

static esp_err_t update_reg(uint8_t reg, uint8_t mask, uint8_t value)
{
    uint8_t cur;
    esp_err_t err = read_reg(reg, &cur);
    if (err != ESP_OK) {
        return err;
    }
    uint8_t next = (cur & (uint8_t)~mask) | (value & mask);
    uint8_t buf[2] = {reg, next};
    return i2c_master_transmit(s_dev, buf, sizeof(buf), 100);
}

esp_err_t bsp_charger_init(void)
{
    i2c_master_bus_config_t bus_cfg = {
        .i2c_port = 1,
        .sda_io_num = BSP_PIN_I2C1_SDA,
        .scl_io_num = BSP_PIN_I2C1_SCL,
        .clk_source = I2C_CLK_SRC_DEFAULT,
        .glitch_ignore_cnt = 7,
        .flags.enable_internal_pullup = true,
    };
    esp_err_t err = i2c_new_master_bus(&bus_cfg, &s_bus);
    if (err != ESP_OK) {
        return err;
    }
    if (i2c_master_probe(s_bus, BSP_I2C_ADDR_CHARGER, 100) != ESP_OK) {
        return ESP_ERR_NOT_FOUND;
    }
    i2c_device_config_t dev_cfg = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = BSP_I2C_ADDR_CHARGER,
        .scl_speed_hz = 100000,
    };
    err = i2c_master_bus_add_device(s_bus, &dev_cfg, &s_dev);
    if (err != ESP_OK) {
        return err;
    }

    uint8_t id;
    err = read_reg(REG0B_ID, &id);
    if (err != ESP_OK) {
        return err;
    }
    if ((id & REG0B_PN_MASK) != REG0B_PN_VAL) {
        return ESP_ERR_NOT_FOUND;
    }

    /* Reset to known defaults, let the chip settle, then disable the I2C
     * watchdog so the configuration sticks without periodic host writes. */
    err = update_reg(REG0B_ID, REG0B_REG_RST, REG0B_REG_RST);
    if (err != ESP_OK) {
        return err;
    }
    vTaskDelay(pdMS_TO_TICKS(10));
    err = update_reg(REG05_COC, REG05_WATCHDOG_MASK, 0);
    if (err != ESP_OK) {
        return err;
    }

    err = bsp_charger_set_charge_current_ma(BOARD_CHG_CURRENT_MA);
    if (err != ESP_OK) {
        return err;
    }
    uint32_t vreg = ((BOARD_CHG_VOLTAGE_MV - VREG_BASE_MV) / VREG_STEP_MV) << REG04_VREG_SHIFT;
    return update_reg(REG04_CVC, REG04_VREG_MASK, (uint8_t)vreg);
}

esp_err_t bsp_charger_get_status(bsp_charger_status_t *out_status)
{
    uint8_t st;
    esp_err_t err = read_reg(REG08_STATUS, &st);
    if (err != ESP_OK) {
        return err;
    }
    if ((st & REG08_PG_STAT) == 0) {
        *out_status = BSP_CHARGER_STATUS_DISCHARGING;
        return ESP_OK;
    }
    switch ((st & REG08_CHRG_STAT_MASK) >> REG08_CHRG_STAT_SHIFT) {
    case 0:
        *out_status = BSP_CHARGER_STATUS_NOT_CHARGING;
        break;
    case 1:
        *out_status = BSP_CHARGER_STATUS_PRECHARGE;
        break;
    case 2:
        *out_status = BSP_CHARGER_STATUS_FAST_CHARGE;
        break;
    default:
        *out_status = BSP_CHARGER_STATUS_FULL;
        break;
    }
    return ESP_OK;
}

esp_err_t bsp_charger_is_input_present(bool *out_present)
{
    uint8_t v;
    esp_err_t err = read_reg(REG0A_VIC, &v);
    if (err != ESP_OK) {
        return err;
    }
    *out_present = (v & REG0A_BUS_GD) != 0;
    return ESP_OK;
}

esp_err_t bsp_charger_set_charge_enabled(bool enabled)
{
    return update_reg(REG01_POC, REG01_CHG_CONFIG, enabled ? REG01_CHG_CONFIG : 0);
}

esp_err_t bsp_charger_get_charge_current_ma(uint16_t *out_ma)
{
    uint8_t v;
    esp_err_t err = read_reg(REG02_CCC, &v);
    if (err != ESP_OK) {
        return err;
    }
    *out_ma = (uint16_t)((v & REG02_ICHG_MASK) * ICHG_STEP_MA);
    return ESP_OK;
}

esp_err_t bsp_charger_set_charge_current_ma(uint16_t ma)
{
    if (ma > ICHG_MAX_MA) {
        ma = ICHG_MAX_MA;
    }
    return update_reg(REG02_CCC, REG02_ICHG_MASK, (uint8_t)(ma / ICHG_STEP_MA));
}

esp_err_t bsp_charger_get_faults(uint8_t *out_fault)
{
    return read_reg(REG09_FAULT, out_fault);
}

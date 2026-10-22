/*
 * Copyright (c) 2026 CodeCraft AI Hardware contributors.
 * SPDX-License-Identifier: Apache-2.0
 *
 * reTerminal E1002 pin map — the board's source of truth.
 *
 * Sources, highest priority first (see docs/vendor-sources.md):
 *  1. Vendor schematic: reTerminal E1001 V1.2 SCH (CC BY-SA 4.0; the E1002 shares the mainboard layout, panel excepted),
 *     files.seeedstudio.com/wiki/reterminal_e10xx/res/
 *     202004307_reTerminal_E1001_V1_2_SCH_251120.pdf
 *  2. Zephyr board definition (Apache 2.0, vendor-contributed):
 *     zephyrproject-rtos/zephyr boards/seeed/reterminal_e1002/
 *     reterminal_e1001_procpu.dts + pinctrl dtsi — key/LED active levels,
 *     battery divider, I2C device addresses, buzzer PWM, 32 MB flash.
 *  3. Meta muse-gadget-sdk (Apache 2.0): esp32/main/epd_spectra6.c (panel SPI
 *     pins and clock), main/reterminal_sht4x.c (SHT4x on I2C0),
 *     devices/sdkconfig.reterminal-e1001 (green key = GPIO 3).
 */
#pragma once

/* ---- E-paper panel: 7.3" E Ink Spectra 6 (GDEP073E01), 800x480, on SPI2 ---- */
#define BSP_PIN_EPD_SCLK        7
#define BSP_PIN_EPD_MOSI        9
#define BSP_PIN_EPD_CS          10
#define BSP_PIN_EPD_DC          11  /* high = data */
#define BSP_PIN_EPD_RST         12  /* active low */
#define BSP_PIN_EPD_BUSY        13  /* low = busy */
#define BSP_EPD_SPI_HZ          (10 * 1000 * 1000)

/* ---- Keys (all active low, internal pull-up) ---- */
#define BSP_PIN_KEY_GREEN       3   /* "Refresh" in Zephyr, green case key */
#define BSP_PIN_KEY_LEFT        4
#define BSP_PIN_KEY_RIGHT       5

/* ---- Status LED ---- */
#define BSP_PIN_LED             6   /* green, active low */

/* ---- I2C0: SHT4x temperature/humidity + PCF8563 RTC ---- */
#define BSP_PIN_I2C0_SDA        19
#define BSP_PIN_I2C0_SCL        20
#define BSP_I2C0_FREQ_HZ        100000
#define BSP_I2C_ADDR_SHT4X      0x44
#define BSP_I2C_ADDR_RTC        0x51  /* PCF8563 */

/* ---- I2C1: SY6974B battery charger ---- */
#define BSP_PIN_I2C1_SDA        39
#define BSP_PIN_I2C1_SCL        40
#define BSP_I2C_ADDR_CHARGER    0x6B

/* ---- Battery voltage sense ---- */
#define BSP_PIN_VBAT_ADC        1   /* ADC1 channel 0 */
#define BSP_PIN_VBAT_EN         21  /* active high; divider 10k/10k => VBAT = 2x */

/* ---- PDM microphone ---- */
#define BSP_PIN_PDM_CLK         42
#define BSP_PIN_PDM_DATA        41
#define BSP_PIN_PDM_EN          38

/* ---- Buzzer (LEDC PWM) ---- */
#define BSP_PIN_BUZZER          45

/* ---- microSD card ---- */
#define BSP_PIN_SD_CS           14
#define BSP_PIN_SD_DET          15
#define BSP_PIN_SD_EN           16

/* ---- Expansion UART1 ---- */
#define BSP_PIN_UART1_TX        17
#define BSP_PIN_UART1_RX        18

/* ---- Touch (routed; populated on touch variants) ---- */
#define BSP_PIN_TOUCH_INT       47
#define BSP_PIN_TOUCH_RST       48

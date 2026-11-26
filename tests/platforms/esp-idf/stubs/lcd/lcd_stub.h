/* SPDX-License-Identifier: Apache-2.0
 * Narrow SDK-boundary doubles for SPI/RGB display and expander contracts. */
#pragma once
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
typedef int esp_err_t;
enum {ESP_OK=0,ESP_FAIL=-1,ESP_ERR_NO_MEM=0x101,ESP_ERR_INVALID_ARG=0x102,
      ESP_ERR_INVALID_STATE=0x103,ESP_ERR_NOT_FOUND=0x105,ESP_ERR_TIMEOUT=0x107};
typedef void *esp_lcd_panel_handle_t;
typedef void *esp_lcd_panel_io_handle_t;
typedef int esp_lcd_spi_bus_handle_t;
typedef void *i2c_master_bus_handle_t;
typedef void *i2c_master_dev_handle_t;
typedef void *SemaphoreHandle_t;
typedef unsigned TickType_t;
enum {GPIO_MODE_OUTPUT=1,GPIO_PULLUP_DISABLE=0,GPIO_PULLDOWN_DISABLE=0,
      GPIO_INTR_DISABLE=0,GPIO_NUM_NC=-1,SPI2_HOST=1,SPI_DMA_CH_AUTO=2,
      LCD_RGB_ELEMENT_ORDER_RGB=0,LCD_CLK_SRC_DEFAULT=0,I2C_CLK_SRC_DEFAULT=0,
      I2C_ADDR_BIT_LEN_7=7,pdTRUE=1};
typedef struct {uint64_t pin_bit_mask;int mode,pull_up_en,pull_down_en,intr_type;} gpio_config_t;
typedef struct {int sclk_io_num,mosi_io_num,miso_io_num,quadwp_io_num,quadhd_io_num,max_transfer_sz;} spi_bus_config_t;
typedef struct {int cs_gpio_num,dc_gpio_num,spi_mode,pclk_hz,trans_queue_depth,lcd_cmd_bits,lcd_param_bits;} esp_lcd_panel_io_spi_config_t;
typedef struct {int reset_gpio_num,rgb_ele_order,bits_per_pixel;} esp_lcd_panel_dev_config_t;
typedef struct {
    int clk_src,data_width,disp_gpio_num,pclk_gpio_num,vsync_gpio_num,hsync_gpio_num,de_gpio_num,data_gpio_nums[16];
    struct {int pclk_hz,h_res,v_res,hsync_pulse_width,hsync_back_porch,hsync_front_porch,
                vsync_pulse_width,vsync_back_porch,vsync_front_porch;
            struct {unsigned pclk_active_neg:1;} flags;} timings;
    size_t bounce_buffer_size_px;
    struct {unsigned fb_in_psram:1;} flags;
} esp_lcd_rgb_panel_config_t;
typedef struct {int i2c_port,sda_io_num,scl_io_num,clk_source,glitch_ignore_cnt;
                struct {unsigned enable_internal_pullup:1;} flags;} i2c_master_bus_config_t;
typedef struct {int dev_addr_length,device_address,scl_speed_hz;} i2c_device_config_t;
esp_err_t gpio_config(const gpio_config_t *cfg);
esp_err_t gpio_set_level(int pin,int level);
esp_err_t spi_bus_initialize(int host,const spi_bus_config_t *cfg,int dma);
esp_err_t esp_lcd_new_panel_io_spi(esp_lcd_spi_bus_handle_t bus,const esp_lcd_panel_io_spi_config_t *cfg,esp_lcd_panel_io_handle_t *out);
esp_err_t esp_lcd_new_panel_st7789(esp_lcd_panel_io_handle_t io,const esp_lcd_panel_dev_config_t *cfg,esp_lcd_panel_handle_t *out);
esp_err_t esp_lcd_new_rgb_panel(const esp_lcd_rgb_panel_config_t *cfg,esp_lcd_panel_handle_t *out);
esp_err_t esp_lcd_panel_reset(esp_lcd_panel_handle_t panel);
esp_err_t esp_lcd_panel_init(esp_lcd_panel_handle_t panel);
esp_err_t esp_lcd_panel_mirror(esp_lcd_panel_handle_t panel,bool x,bool y);
esp_err_t esp_lcd_panel_set_gap(esp_lcd_panel_handle_t panel,int x,int y);
esp_err_t esp_lcd_panel_invert_color(esp_lcd_panel_handle_t panel,bool invert);
esp_err_t esp_lcd_panel_disp_on_off(esp_lcd_panel_handle_t panel,bool on);
esp_err_t esp_lcd_panel_io_tx_param(esp_lcd_panel_io_handle_t io,int command,const void *data,size_t size);
esp_err_t esp_lcd_panel_draw_bitmap(esp_lcd_panel_handle_t panel,int x0,int y0,int x1,int y1,const void *data);
esp_err_t esp_lcd_panel_del(esp_lcd_panel_handle_t panel);
esp_err_t esp_lcd_panel_io_del(esp_lcd_panel_io_handle_t io);
void vTaskDelay(unsigned ticks);
void esp_rom_delay_us(unsigned us);
#define pdMS_TO_TICKS(ms) ((ms)/10)
SemaphoreHandle_t xSemaphoreCreateMutex(void);
int xSemaphoreTake(SemaphoreHandle_t mutex,TickType_t wait);
void xSemaphoreGive(SemaphoreHandle_t mutex);
void vSemaphoreDelete(SemaphoreHandle_t mutex);
esp_err_t i2c_new_master_bus(const i2c_master_bus_config_t *cfg,i2c_master_bus_handle_t *out);
esp_err_t i2c_master_bus_add_device(i2c_master_bus_handle_t bus,const i2c_device_config_t *cfg,i2c_master_dev_handle_t *out);
esp_err_t i2c_master_probe(i2c_master_bus_handle_t bus,int addr,int timeout);
esp_err_t i2c_master_bus_rm_device(i2c_master_dev_handle_t dev);
esp_err_t i2c_del_master_bus(i2c_master_bus_handle_t bus);
esp_err_t i2c_master_transmit(i2c_master_dev_handle_t dev,const uint8_t *data,size_t size,int timeout);
esp_err_t i2c_master_transmit_receive(i2c_master_dev_handle_t dev,const uint8_t *tx,size_t tx_size,uint8_t *rx,size_t rx_size,int timeout);
static inline void stub_log(const char *tag,const char *fmt,...){(void)tag;(void)fmt;}
#define ESP_LOGI stub_log
#define ESP_LOGE stub_log
#define ESP_RETURN_ON_ERROR(call,tag,...) do {esp_err_t e_=(call);if(e_!=ESP_OK){stub_log(tag,__VA_ARGS__);return e_;}} while(0)
#define ESP_RETURN_ON_FALSE(cond,err,tag,...) do {if(!(cond)){stub_log(tag,__VA_ARGS__);return (err);}} while(0)
#define ESP_GOTO_ON_ERROR(call,label,tag,...) do {ret=(call);if(ret!=ESP_OK){stub_log(tag,__VA_ARGS__);goto label;}} while(0)
#define ESP_ERROR_CHECK(call) do {if((call)!=ESP_OK) __builtin_trap();} while(0)

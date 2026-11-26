/* SPDX-License-Identifier: Apache-2.0
 * Narrow SDK boundaries used by the remaining BSP contract scenarios. */
#pragma once
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <assert.h>
typedef int esp_err_t;
enum { ESP_OK=0, ESP_FAIL=-1, ESP_ERR_NO_MEM=0x101, ESP_ERR_INVALID_ARG=0x102,
       ESP_ERR_INVALID_STATE=0x103, ESP_ERR_NOT_FOUND=0x105,
       ESP_ERR_NOT_SUPPORTED=0x106, ESP_ERR_TIMEOUT=0x107 };
enum { GPIO_MODE_INPUT=1, GPIO_MODE_OUTPUT=2, GPIO_PULLUP_ENABLE=1,
       GPIO_NUM_NC=-1, pdPASS=1, pdTRUE=1, pdFALSE=0, SPI2_HOST=1,
       SPI_DMA_CH_AUTO=2, I2C_CLK_SRC_DEFAULT=0, I2C_ADDR_BIT_LEN_7=7 };
typedef uint32_t TickType_t;
#ifndef TEST_TICK_MS
#define TEST_TICK_MS 10
#endif
#define portTICK_PERIOD_MS TEST_TICK_MS
#define pdMS_TO_TICKS(ms) ((ms)/TEST_TICK_MS)
typedef void (*TaskFunction_t)(void *);
typedef void *TaskHandle_t;
typedef void *QueueHandle_t;
#define portMAX_DELAY UINT32_MAX
#define configASSERT assert
TaskHandle_t xTaskGetCurrentTaskHandle(void);
int xTaskNotifyGive(TaskHandle_t task);
uint32_t ulTaskNotifyTake(int clear,TickType_t wait);
QueueHandle_t xQueueCreate(unsigned length,unsigned size);
int xQueueSend(QueueHandle_t queue,const void *item,TickType_t wait);
int xQueueReceive(QueueHandle_t queue,void *item,TickType_t wait);
int xTaskCreate(TaskFunction_t task,const char *name,unsigned stack,void *arg,unsigned priority,void *handle);
TickType_t xTaskGetTickCount(void);
void vTaskDelay(TickType_t ticks);
typedef struct {uint64_t pin_bit_mask;int mode,pull_up_en,pull_down_en,intr_type;} gpio_config_t;
esp_err_t gpio_config(const gpio_config_t *cfg);
esp_err_t gpio_set_level(int pin,int level);
int gpio_get_level(int pin);
typedef void *SemaphoreHandle_t;
SemaphoreHandle_t xSemaphoreCreateMutex(void);
int xSemaphoreTake(SemaphoreHandle_t mutex,TickType_t wait);
void xSemaphoreGive(SemaphoreHandle_t mutex);
void vSemaphoreDelete(SemaphoreHandle_t mutex);
typedef void *adc_oneshot_unit_handle_t;
typedef void *adc_cali_handle_t;
enum {ADC_UNIT_1=1,ADC_ULP_MODE_DISABLE=0,ADC_BITWIDTH_12=12,ADC_ATTEN_DB_12=12,ADC_CHANNEL_0=0};
typedef struct {int unit_id,ulp_mode;} adc_oneshot_unit_init_cfg_t;
typedef struct {int bitwidth,atten;} adc_oneshot_chan_cfg_t;
typedef struct {int unit_id,chan,atten,bitwidth;} adc_cali_curve_fitting_config_t;
esp_err_t adc_oneshot_new_unit(const adc_oneshot_unit_init_cfg_t *cfg,adc_oneshot_unit_handle_t *out);
esp_err_t adc_oneshot_config_channel(adc_oneshot_unit_handle_t unit,int channel,const adc_oneshot_chan_cfg_t *cfg);
esp_err_t adc_oneshot_del_unit(adc_oneshot_unit_handle_t unit);
esp_err_t adc_oneshot_read(adc_oneshot_unit_handle_t unit,int channel,int *out);
esp_err_t adc_cali_create_scheme_curve_fitting(const adc_cali_curve_fitting_config_t *cfg,adc_cali_handle_t *out);
esp_err_t adc_cali_raw_to_voltage(adc_cali_handle_t cali,int raw,int *out);
typedef void *i2c_master_bus_handle_t;
typedef void *i2c_master_dev_handle_t;
typedef struct {int i2c_port,sda_io_num,scl_io_num,clk_source,glitch_ignore_cnt;
    struct {unsigned enable_internal_pullup:1;} flags;} i2c_master_bus_config_t;
typedef struct {int dev_addr_length,device_address,scl_speed_hz;} i2c_device_config_t;
esp_err_t i2c_new_master_bus(const i2c_master_bus_config_t *cfg,i2c_master_bus_handle_t *out);
esp_err_t i2c_master_bus_add_device(i2c_master_bus_handle_t bus,const i2c_device_config_t *cfg,i2c_master_dev_handle_t *out);
esp_err_t i2c_master_probe(i2c_master_bus_handle_t bus,int addr,int timeout);
esp_err_t i2c_master_bus_rm_device(i2c_master_dev_handle_t dev);
esp_err_t i2c_del_master_bus(i2c_master_bus_handle_t bus);
esp_err_t i2c_master_transmit(i2c_master_dev_handle_t dev,const uint8_t *data,size_t size,int timeout);
esp_err_t i2c_master_transmit_receive(i2c_master_dev_handle_t dev,const uint8_t *tx,size_t tx_size,uint8_t *rx,size_t rx_size,int timeout);
typedef struct {int sclk_io_num,mosi_io_num,miso_io_num,quadwp_io_num,quadhd_io_num,max_transfer_sz;} spi_bus_config_t;
esp_err_t spi_bus_initialize(int host,const spi_bus_config_t *cfg,int dma);
typedef struct {int slot;} sdmmc_host_t;
typedef struct {int host_id,gpio_cs;} sdspi_device_config_t;
typedef struct {int alive;} sdmmc_card_t;
typedef struct {bool format_if_mount_failed;int max_files;size_t allocation_unit_size;} esp_vfs_fat_sdmmc_mount_config_t;
#define SDSPI_HOST_DEFAULT() ((sdmmc_host_t){.slot=SPI2_HOST})
#define SDSPI_DEVICE_CONFIG_DEFAULT() ((sdspi_device_config_t){.host_id=SPI2_HOST})
esp_err_t esp_vfs_fat_sdspi_mount(const char *path,const sdmmc_host_t *host,const sdspi_device_config_t *slot,const esp_vfs_fat_sdmmc_mount_config_t *cfg,sdmmc_card_t **out);
esp_err_t esp_vfs_fat_sdcard_unmount(const char *path,sdmmc_card_t *card);
enum {LEDC_TIMER_0=0,LEDC_CHANNEL_0=0,LEDC_TIMER_10_BIT=10,LEDC_LOW_SPEED_MODE=0,LEDC_AUTO_CLK=0};
typedef struct {int speed_mode,duty_resolution,timer_num,freq_hz,clk_cfg;} ledc_timer_config_t;
typedef struct {int gpio_num,speed_mode,channel,timer_sel;uint32_t duty;int hpoint;} ledc_channel_config_t;
esp_err_t ledc_timer_config(const ledc_timer_config_t *cfg);
esp_err_t ledc_channel_config(const ledc_channel_config_t *cfg);
esp_err_t ledc_set_freq(int mode,int timer,uint32_t frequency);
esp_err_t ledc_set_duty(int mode,int channel,uint32_t duty);
esp_err_t ledc_update_duty(int mode,int channel);
typedef void *led_strip_handle_t;
typedef struct {int strip_gpio_num,max_leds;} led_strip_config_t;
typedef struct {int resolution_hz;} led_strip_rmt_config_t;
esp_err_t led_strip_new_rmt_device(const led_strip_config_t *cfg,const led_strip_rmt_config_t *rmt,led_strip_handle_t *out);
esp_err_t led_strip_set_pixel(led_strip_handle_t strip,uint32_t index,uint32_t r,uint32_t g,uint32_t b);
esp_err_t led_strip_refresh(led_strip_handle_t strip);
esp_err_t led_strip_clear(led_strip_handle_t strip);
typedef void *esp_lcd_touch_handle_t;
typedef void *esp_lcd_panel_io_handle_t;
typedef void *esp_lcd_panel_handle_t;
typedef struct {uint16_t x,y,strength;} esp_lcd_touch_point_data_t;
esp_err_t esp_lcd_touch_read_data(esp_lcd_touch_handle_t touch);
esp_err_t esp_lcd_touch_get_data(esp_lcd_touch_handle_t touch,esp_lcd_touch_point_data_t *point,uint8_t *count,uint8_t maximum);
esp_err_t esp_lcd_panel_draw_bitmap(esp_lcd_panel_handle_t panel,int x0,int y0,int x1,int y1,const void *data);
const char *esp_err_to_name(esp_err_t err);
typedef struct {int dev_addr;} esp_lcd_panel_io_i2c_config_t;
typedef struct {int x_max,y_max,rst_gpio_num,int_gpio_num;
    struct {int reset,interrupt;} levels;struct {int swap_xy,mirror_x,mirror_y;} flags;
    void (*process_coordinates)(esp_lcd_touch_handle_t, uint16_t *, uint16_t *, uint16_t *, uint8_t *, uint8_t);
} esp_lcd_touch_config_t;
#define ESP_LCD_TOUCH_IO_I2C_FT5x06_CONFIG() ((esp_lcd_panel_io_i2c_config_t){0})
#define ESP_LCD_TOUCH_IO_I2C_GT911_CONFIG() ((esp_lcd_panel_io_i2c_config_t){0})
esp_err_t esp_lcd_new_panel_io_i2c(i2c_master_bus_handle_t bus,const esp_lcd_panel_io_i2c_config_t *cfg,esp_lcd_panel_io_handle_t *out);
esp_err_t esp_lcd_panel_io_del(esp_lcd_panel_io_handle_t io);
esp_err_t esp_lcd_touch_new_i2c_ft5x06(esp_lcd_panel_io_handle_t io,const esp_lcd_touch_config_t *cfg,esp_lcd_touch_handle_t *out);
esp_err_t esp_lcd_touch_new_i2c_gt911(esp_lcd_panel_io_handle_t io,const esp_lcd_touch_config_t *cfg,esp_lcd_touch_handle_t *out);
#ifdef TEST_CAPTURE_LOG
void stub_log(const char *tag,const char *fmt,...);
#else
static inline void stub_log(const char *tag,const char *fmt,...){(void)tag;(void)fmt;}
#endif
#define ESP_LOGI stub_log
#define ESP_LOGW stub_log
#define ESP_LOGE stub_log
#define ESP_ERROR_CHECK(call) do {assert((call)==ESP_OK);} while(0)
#define ESP_RETURN_ON_ERROR(call,tag,...) do {esp_err_t e_=(call);if(e_!=ESP_OK){stub_log(tag,__VA_ARGS__);return e_;}} while(0)
#define ESP_RETURN_ON_FALSE(cond,err,tag,...) do {if(!(cond)){stub_log(tag,__VA_ARGS__);return (err);}} while(0)

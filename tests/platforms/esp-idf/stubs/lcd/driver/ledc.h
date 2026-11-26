#pragma once
#include "lcd_stub.h"
enum {LEDC_LOW_SPEED_MODE=0,LEDC_TIMER_0=0,LEDC_CHANNEL_0=0,LEDC_TIMER_8_BIT=8,LEDC_AUTO_CLK=0};
typedef struct {int speed_mode,duty_resolution,timer_num;uint32_t freq_hz;int clk_cfg;} ledc_timer_config_t;
typedef struct {int gpio_num,speed_mode,channel,timer_sel;uint32_t duty;int hpoint;} ledc_channel_config_t;
esp_err_t ledc_timer_config(const ledc_timer_config_t *cfg);
esp_err_t ledc_channel_config(const ledc_channel_config_t *cfg);
esp_err_t ledc_set_duty(int mode,int channel,uint32_t duty);
esp_err_t ledc_update_duty(int mode,int channel);

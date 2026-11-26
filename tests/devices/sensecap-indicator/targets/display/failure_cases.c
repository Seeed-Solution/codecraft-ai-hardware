/* SPDX-License-Identifier: Apache-2.0
 * RGB allocation, complete bit-bang error propagation and readiness cases. */
#include "bsp_display.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"
#include "lcd_stub.h"
#include "driver/ledc.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int failure,cleanup_failure,occurrence=1,seen,calls,setup,deleted;
static int events[50],panel_object;
static bool panel_live,spi_started,custom;
static uint32_t duty;
static int rp_level=-1,bmp_level=-1;
static esp_err_t step(int stage);
esp_err_t ledc_timer_config(const ledc_timer_config_t *c){assert(c->freq_hz==5000&&c->duty_resolution==8&&c->timer_num==0);return step(22);}

esp_err_t ledc_channel_config(const ledc_channel_config_t *c){assert(c->gpio_num==BSP_PIN_LCD_BACKLIGHT&&c->duty==0&&c->channel==0);return step(23);}
esp_err_t ledc_set_duty(int mode,int channel,uint32_t value){assert(mode==0&&channel==0&&value<=255);esp_err_t e=step(24);if(!e)duty=value;return e;}
esp_err_t ledc_update_duty(int mode,int channel){assert(mode==0&&channel==0);return step(25);}

static bool touch_reset_low;
static unsigned reset_delay_us;
static uint64_t wire=1469598103934665603ULL;
#if !CODECRAFT_DISPLAY_GX
static int dx_pins[12],dx_kinds[12],dx_values[12];
#endif
static void hash(unsigned value){wire=(wire^value)*1099511628211ULL;}
static esp_err_t step(int stage)
{
    ++calls;++events[stage];
    if(failure==stage&&++seen==occurrence){failure=0;return ESP_FAIL;}
    if(cleanup_failure==stage){cleanup_failure=0;return ESP_FAIL;}
    return ESP_OK;
}
esp_err_t bsp_io_expander_init(void){setup=0;spi_started=false;return step(1);}
esp_err_t bsp_io_expander_set_level(int pin,int level)
{
    if(pin==BSP_EXP_IO_RP2040_RESET)rp_level=level;
    if(pin==BSP_EXP_IO_BMP_PWR)bmp_level=level;
    hash(1);hash(pin);hash(level);
    if(spi_started){assert(pin==BSP_EXP_IO_LCD_CS);return step(17);}
#if !CODECRAFT_DISPLAY_GX
    dx_pins[setup]=pin;dx_kinds[setup]=1;dx_values[setup]=level;
#endif
    ++setup;esp_err_t ret=step(setup<=7?setup+1:setup+4);
    if(ret==ESP_OK&&pin==BSP_EXP_IO_TP_RESET){
        if(level){assert(touch_reset_low&&reset_delay_us>=5000);touch_reset_low=false;}
        else {touch_reset_low=true;reset_delay_us=0;}
    }
    return ret;
}
esp_err_t bsp_io_expander_set_direction(int pin,int output)
{
    if(custom&&pin==BSP_EXP_IO_RP2040_RESET)assert(rp_level==0);
    if(custom&&pin==BSP_EXP_IO_BMP_PWR)assert(bmp_level==0);
    hash(2);hash(pin);hash(output);assert(!spi_started&&output==1);
#if !CODECRAFT_DISPLAY_GX
    dx_pins[setup]=pin;dx_kinds[setup]=2;dx_values[setup]=output;
#endif
    ++setup;return step(setup<=7?setup+1:setup+4);
}
esp_err_t gpio_config(const gpio_config_t *c)
{
    hash(3);hash((unsigned)c->pin_bit_mask);hash((unsigned)(c->pin_bit_mask>>32));
    assert(c->mode==GPIO_MODE_OUTPUT);
    if(c->pin_bit_mask==(1ULL<<BSP_PIN_LCD_BACKLIGHT))return step(20);
    assert(c->pin_bit_mask==((1ULL<<BSP_PIN_LCD_SPI_SCK)|(1ULL<<BSP_PIN_LCD_SPI_SDO)));
    assert(setup==11);esp_err_t ret=step(16);if(ret==ESP_OK)spi_started=true;return ret;
}
esp_err_t gpio_set_level(int pin,int level)
{
    hash(4);hash(pin);hash(level);
    if(pin==BSP_PIN_LCD_BACKLIGHT)return step(21);
    assert(spi_started);
    if(pin==BSP_PIN_LCD_SPI_SCK)return step(18);
    assert(pin==BSP_PIN_LCD_SPI_SDO);return step(19);
}
void esp_rom_delay_us(unsigned us){hash(5);hash(us);assert(us==10||us==5000);if(touch_reset_low)reset_delay_us+=us;}
void vTaskDelay(unsigned ticks){hash(6);hash(ticks);if(touch_reset_low&&ticks)reset_delay_us+=(ticks-1)*10000;}
esp_err_t esp_lcd_new_rgb_panel(const esp_lcd_rgb_panel_config_t *c,esp_lcd_panel_handle_t *out)
{
    assert(!panel_live&&setup==7);
    assert(c->bounce_buffer_size_px==(custom?4800:0));
    assert(rp_level==!custom&&bmp_level==!custom);
    assert(c->data_width==16&&c->disp_gpio_num==GPIO_NUM_NC&&c->flags.fb_in_psram);
    assert(c->pclk_gpio_num==BSP_PIN_LCD_PCLK&&c->vsync_gpio_num==BSP_PIN_LCD_VSYNC&&c->hsync_gpio_num==BSP_PIN_LCD_HSYNC&&c->de_gpio_num==BSP_PIN_LCD_DE);
    const int pins[]={BSP_PIN_LCD_D0,BSP_PIN_LCD_D1,BSP_PIN_LCD_D2,BSP_PIN_LCD_D3,BSP_PIN_LCD_D4,BSP_PIN_LCD_D5,BSP_PIN_LCD_D6,BSP_PIN_LCD_D7,BSP_PIN_LCD_D8,BSP_PIN_LCD_D9,BSP_PIN_LCD_D10,BSP_PIN_LCD_D11,BSP_PIN_LCD_D12,BSP_PIN_LCD_D13,BSP_PIN_LCD_D14,BSP_PIN_LCD_D15};
    assert(memcmp(pins,c->data_gpio_nums,sizeof(pins))==0);
    assert(c->timings.pclk_hz==(custom?16000000:BSP_LCD_PCLK_HZ)&&c->timings.h_res==BSP_LCD_H_RES&&c->timings.v_res==BSP_LCD_V_RES);
    assert(c->timings.hsync_pulse_width==BSP_LCD_HSYNC_PULSE_WIDTH&&c->timings.hsync_back_porch==BSP_LCD_HSYNC_BACK_PORCH&&c->timings.hsync_front_porch==BSP_LCD_HSYNC_FRONT_PORCH);
    assert(c->timings.vsync_pulse_width==BSP_LCD_VSYNC_PULSE_WIDTH&&c->timings.vsync_back_porch==BSP_LCD_VSYNC_BACK_PORCH&&c->timings.vsync_front_porch==BSP_LCD_VSYNC_FRONT_PORCH&&c->timings.flags.pclk_active_neg==BSP_LCD_PCLK_ACTIVE_NEG);
    esp_err_t ret=step(9);if(ret!=ESP_OK)return ret;
    panel_live=true;*out=&panel_object;return ESP_OK;
}
esp_err_t esp_lcd_panel_reset(esp_lcd_panel_handle_t panel){assert(panel==&panel_object&&panel_live);return step(10);}
esp_err_t esp_lcd_panel_init(esp_lcd_panel_handle_t panel){assert(panel==&panel_object&&panel_live);return step(11);}
esp_err_t esp_lcd_panel_del(esp_lcd_panel_handle_t panel)
{
    assert(panel==&panel_object&&panel_live);++deleted;
    esp_err_t ret=step(40);if(ret==ESP_OK)panel_live=false;return ret;
}
int main(int argc,char **argv)
{
    assert(argc>=2);const char *mode=argv[1];
    esp_lcd_panel_handle_t panel=(void *)1;
    if(!strcmp(mode,"config")){
        bsp_display_config_t cfg=BSP_DISPLAY_CONFIG_DEFAULT();
        assert(bsp_display_init_with_config(NULL,&panel)==ESP_ERR_INVALID_ARG&&!panel&&calls==0);
        cfg.pclk_hz=0;assert(bsp_display_init_with_config(&cfg,&panel)==ESP_ERR_INVALID_ARG&&calls==0);
        cfg.pclk_hz=16000000;cfg.bounce_buffer_lines=7;
        assert(bsp_display_init_with_config(&cfg,&panel)==ESP_ERR_INVALID_ARG&&calls==0);
        cfg.bounce_buffer_lines=10;cfg.hold_rp2040_reset=true;cfg.backlight_pwm_hz=5000;custom=true;
        failure=argc>2?atoi(argv[2]):0;
        if(failure){assert(bsp_display_init_with_config(&cfg,&panel)==ESP_FAIL&&!panel&&!panel_live);}
        assert(bsp_display_init_with_config(&cfg,&panel)==ESP_OK);
        int before=calls;assert(bsp_display_init_with_config(&cfg,&panel)==ESP_OK&&calls==before);
        assert(bsp_display_init(&panel)==ESP_ERR_INVALID_STATE&&!panel&&calls==before);
        assert(bsp_display_brightness_set(101)==ESP_ERR_INVALID_ARG&&calls==before);
        assert(bsp_display_brightness_set(70)==ESP_OK&&duty==179&&bsp_display_brightness_get()==70);
        for(int stage=24;stage<=25;stage++){
            failure=stage;seen=0;assert(bsp_display_brightness_set(20)==ESP_FAIL&&bsp_display_brightness_get()==70);
        }
        assert(bsp_display_backlight_set(true)==ESP_OK&&duty==255&&bsp_display_backlight_get());
        assert(bsp_display_backlight_set(false)==ESP_OK&&duty==0&&!bsp_display_backlight_get());return 0;
    }
    if(!strcmp(mode,"wire")){
        assert(bsp_display_init(&panel)==ESP_OK);
        printf("%llu %d %d %d\n",(unsigned long long)wire,events[17],events[18],events[19]);return 0;
    }
    if(!strcmp(mode,"unready")){
        assert(bsp_display_brightness_set(50)==ESP_ERR_INVALID_STATE);
        assert(bsp_display_backlight_set(true)==ESP_ERR_INVALID_STATE);
        assert(!bsp_display_backlight_get()&&calls==0);return 0;
    }
    failure=atoi(mode);cleanup_failure=argc>2?atoi(argv[2]):0;
    occurrence=argc>3?atoi(argv[3]):1;
    int expected=failure,cleanup=cleanup_failure;
    esp_err_t ret=bsp_display_init(!strcmp(mode,"null")?NULL:&panel);
    if(expected){
        assert(ret==ESP_FAIL&&failure==0&&panel==NULL);
        assert(bsp_display_brightness_set(50)==ESP_ERR_INVALID_STATE);
        assert(bsp_display_backlight_set(true)==ESP_ERR_INVALID_STATE);
        assert(!bsp_display_backlight_get());
        if(cleanup){
            assert(cleanup_failure==0&&panel_live&&deleted==1);
            int prior=calls;
            assert(bsp_display_init(&panel)==ESP_ERR_INVALID_STATE&&panel==NULL&&calls==prior);
            return 0;
        }
        assert(!panel_live);
        assert(bsp_display_init(&panel)==ESP_OK);
    }else assert(ret==ESP_OK);
#if !CODECRAFT_DISPLAY_GX
    /* Vendor DX contract: no register-channel GPIO setup/transfers. The
     * four final expander actions release CS/reset then enable outputs. */
    assert(setup==11&&!spi_started&&events[16]==0&&events[17]==0&&events[18]==0&&events[19]==0);
    const int expected_pins[]={BSP_EXP_IO_LCD_CS,BSP_EXP_IO_LCD_RESET,BSP_EXP_IO_LCD_CS,BSP_EXP_IO_LCD_RESET};
    for(int i=0;i<4;i++)assert(dx_pins[7+i]==expected_pins[i]&&dx_values[7+i]==1&&dx_kinds[7+i]==(i<2?1:2));
#endif
    int prior=calls;uint64_t prior_wire=wire;
    assert(bsp_display_init(&panel)==ESP_OK&&panel==&panel_object&&calls==prior&&wire==prior_wire);
    assert(bsp_display_init(NULL)==ESP_OK&&calls==prior);
    assert(bsp_display_backlight_set(true)==ESP_OK&&bsp_display_backlight_get());
    failure=21;seen=0;occurrence=1;
    assert(bsp_display_backlight_set(false)==ESP_FAIL&&bsp_display_backlight_get());
    assert(bsp_display_backlight_set(false)==ESP_OK&&!bsp_display_backlight_get());
    return 0;
}

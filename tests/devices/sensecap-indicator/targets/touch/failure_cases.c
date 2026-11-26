/* SPDX-License-Identifier: Apache-2.0 */
#include "peripheral_stub.h"
#include "bsp_touch.h"
#include "bsp_io_expander.h"
#include "bsp_pins.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>
static int calls,fail_at,candidate,probes,io,controller,deletions;
static bool cleanup_fail,missing;
static bool reset_low;
static unsigned reset_delay_us;
static const int addresses[]={BSP_I2C_ADDR_TOUCH_FT6x36,BSP_I2C_ADDR_TOUCH_FT6x36_ALT,BSP_I2C_ADDR_TOUCH_GT911,BSP_I2C_ADDR_TOUCH_GT911_ALT};
static esp_err_t step(void){return ++calls==fail_at?ESP_FAIL:ESP_OK;}
esp_err_t bsp_io_expander_init(void){return step();}
esp_err_t bsp_io_expander_set_direction(int pin,int output){assert(pin==BSP_EXP_IO_TP_RESET&&output==1);return step();}
esp_err_t bsp_io_expander_set_level(int pin,int level){
    assert(pin==BSP_EXP_IO_TP_RESET&&level<=1);esp_err_t e=step();
    if(!e){if(level){assert(reset_low&&reset_delay_us>=5000);reset_low=false;}
           else {reset_low=true;reset_delay_us=0;}}
    return e;
}
esp_err_t bsp_i2c_get_bus(i2c_master_bus_handle_t *out){esp_err_t e=step();if(!e)*out=&calls;return e;}
void vTaskDelay(TickType_t ticks){if(reset_low&&ticks)reset_delay_us+=(ticks-1)*TEST_TICK_MS*1000;}
void esp_rom_delay_us(unsigned us){if(reset_low)reset_delay_us+=us;}
esp_err_t i2c_master_probe(i2c_master_bus_handle_t bus,int addr,int timeout){assert(bus==&calls&&timeout==50&&addr==addresses[probes++]);return !missing&&addr==addresses[candidate]?ESP_OK:ESP_ERR_NOT_FOUND;}
esp_err_t esp_lcd_new_panel_io_i2c(i2c_master_bus_handle_t bus,const esp_lcd_panel_io_i2c_config_t *c,esp_lcd_panel_io_handle_t *out){assert(bus==&calls&&!io&&c->dev_addr==addresses[candidate]);esp_err_t e=step();if(!e){io=1;*out=&io;}return e;}
esp_err_t esp_lcd_panel_io_del(esp_lcd_panel_io_handle_t handle){assert(handle==&io&&io&&!controller);deletions++;if(cleanup_fail)return ESP_FAIL;io=0;return ESP_OK;}
static esp_err_t construct(esp_lcd_panel_io_handle_t handle,const esp_lcd_touch_config_t *c,esp_lcd_touch_handle_t *out,bool goodix){
    assert(handle==&io&&io&&!controller&&goodix==(candidate>=2));assert(c->x_max==480&&c->y_max==480&&c->rst_gpio_num==GPIO_NUM_NC&&c->int_gpio_num==GPIO_NUM_NC);
    if (CODECRAFT_DISPLAY_GX && !goodix) {
        assert(c->process_coordinates);
        uint16_t x[]={0,479,0,479,480,12,65535},y[]={0,0,479,479,1,34,0};
        uint16_t strength[]={1,2,3,4,5,6,7};uint8_t count=7;
        c->process_coordinates(NULL,x,y,strength,&count,7);
        assert(count==5&&x[0]==479&&y[0]==479&&x[1]==0&&y[1]==479);
        assert(x[2]==479&&y[2]==0&&x[3]==0&&y[3]==0);
        assert(x[4]==467&&y[4]==445&&strength[4]==6);
        count=0;c->process_coordinates(NULL,x,y,NULL,&count,7);assert(count==0);
        x[0]=0;y[0]=0;count=7;c->process_coordinates(NULL,x,y,NULL,&count,1);assert(count==1);
    } else assert(!c->process_coordinates);
    assert(!c->flags.swap_xy&&!c->flags.mirror_x&&!c->flags.mirror_y);esp_err_t e=step();if(!e){controller=1;*out=&controller;}return e;}
esp_err_t esp_lcd_touch_new_i2c_ft5x06(esp_lcd_panel_io_handle_t h,const esp_lcd_touch_config_t *c,esp_lcd_touch_handle_t *out){return construct(h,c,out,false);}
esp_err_t esp_lcd_touch_new_i2c_gt911(esp_lcd_panel_io_handle_t h,const esp_lcd_touch_config_t *c,esp_lcd_touch_handle_t *out){return construct(h,c,out,true);}
int main(int argc,char **argv){
    assert(argc>1);const char *mode=argv[1];esp_lcd_touch_handle_t out=(void *)1;assert(!bsp_touch_get_handle());
    if(!strcmp(mode,"candidate")){assert(argc==3);candidate=atoi(argv[2]);}
    missing=!strcmp(mode,"missing");cleanup_fail=!strcmp(mode,"cleanup");fail_at=cleanup_fail?7:atoi(mode);
    esp_err_t e=bsp_touch_init(!strcmp(mode,"optional")?NULL:&out);
    if(fail_at||missing){assert(e==(missing?ESP_ERR_NOT_FOUND:ESP_FAIL)&&!out&&!bsp_touch_get_handle()&&!controller);
        if(cleanup_fail){int before=calls;assert(io&&deletions==1);assert(bsp_touch_init(&out)==ESP_ERR_INVALID_STATE&&!out&&before==calls);return 0;}
        assert(!io);if(fail_at==7)assert(deletions==1);fail_at=0;missing=false;probes=0;assert(bsp_touch_init(&out)==ESP_OK);
    }else assert(e==ESP_OK);
    assert(io&&controller&&bsp_touch_get_handle()==&controller);int before=calls,probe_count=probes;
    assert(bsp_touch_init(&out)==ESP_OK&&out==&controller&&before==calls&&probes==probe_count);return 0;
}

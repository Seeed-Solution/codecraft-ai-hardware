/* TCA9535 16-bit IO expander on the board I2C bus.
 * Register map per TI TCA9535 datasheet (SLASE01): 0/1 input, 2/3 output,
 * 4/5 polarity inversion, 6/7 configuration (1 = input).
 * Address facts: Seeed-Solution/SenseCAP_Indicator_ESP32 (Apache 2.0)
 * sensecap_indicator_board.c — probe 0x20, fall back to 0x39. */
#include "bsp_io_expander.h"
#include "bsp_pins.h"

#include "driver/i2c_master.h"
#include "esp_check.h"

static const char *TAG = "bsp_io_expander";

static i2c_master_bus_handle_t s_bus;
static i2c_master_dev_handle_t s_dev;
static uint8_t s_addr;

/* Shadow copies of the output (reg 2/3) and config (reg 6/7) registers so
 * single-pin updates never clobber the other pins on the same port. */
static uint8_t s_output[2] = {0xFF, 0xFF};
static uint8_t s_config[2] = {0xFF, 0xFF};
static bool s_shadow_valid;

static esp_err_t exp_write_reg(uint8_t reg, uint8_t val)
{
    uint8_t buf[2] = {reg, val};
    return i2c_master_transmit(s_dev, buf, sizeof(buf), 100);
}

static esp_err_t exp_read_reg(uint8_t reg, uint8_t *val)
{
    return i2c_master_transmit_receive(s_dev, &reg, 1, val, 1, 100);
}

static esp_err_t exp_probe(uint8_t addr)
{
    i2c_master_dev_handle_t dev;
    i2c_device_config_t cfg = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = addr,
        .scl_speed_hz = BSP_I2C_FREQ_HZ,
    };
    ESP_RETURN_ON_ERROR(i2c_master_bus_add_device(s_bus, &cfg, &dev), TAG,
                        "add device 0x%02x", addr);
    esp_err_t ret = i2c_master_probe(s_bus, addr, 100);
    if (ret == ESP_OK) {
        s_dev = dev;
        s_addr = addr;
    } else {
        i2c_master_bus_rm_device(dev);
    }
    return ret;
}

esp_err_t bsp_io_expander_init(void)
{
    if (s_dev) {
        return ESP_OK;
    }
    if (!s_bus) {
        i2c_master_bus_config_t bus_cfg = {
            .i2c_port = -1,
            .sda_io_num = BSP_PIN_I2C_SDA,
            .scl_io_num = BSP_PIN_I2C_SCL,
            .clk_source = I2C_CLK_SRC_DEFAULT,
            .glitch_ignore_cnt = 7,
            .flags.enable_internal_pullup = true,
        };
        ESP_RETURN_ON_ERROR(i2c_new_master_bus(&bus_cfg, &s_bus), TAG, "bus");
    }
    esp_err_t ret = exp_probe(BSP_I2C_ADDR_EXPANDER_PRIMARY);
    if (ret != ESP_OK) {
        ret = exp_probe(BSP_I2C_ADDR_EXPANDER_FALLBACK);
    }
    ESP_RETURN_ON_ERROR(ret, TAG, "TCA9535 not found at 0x%02x or 0x%02x",
                        BSP_I2C_ADDR_EXPANDER_PRIMARY,
                        BSP_I2C_ADDR_EXPANDER_FALLBACK);

    /* Sync the shadows with the real chip state. */
    ESP_RETURN_ON_ERROR(exp_read_reg(2, &s_output[0]), TAG, "read output0");
    ESP_RETURN_ON_ERROR(exp_read_reg(3, &s_output[1]), TAG, "read output1");
    ESP_RETURN_ON_ERROR(exp_read_reg(6, &s_config[0]), TAG, "read config0");
    ESP_RETURN_ON_ERROR(exp_read_reg(7, &s_config[1]), TAG, "read config1");
    s_shadow_valid = true;
    return ESP_OK;
}

esp_err_t bsp_io_expander_set_direction(int pin, int output)
{
    ESP_RETURN_ON_FALSE(s_dev && s_shadow_valid, ESP_ERR_INVALID_STATE, TAG,
                        "not initialized");
    ESP_RETURN_ON_FALSE(pin >= 0 && pin < 16, ESP_ERR_INVALID_ARG, TAG, "pin");
    uint8_t mask = 1u << (pin & 7);
    int port = pin >> 3;
    if (output) {
        s_config[port] &= ~mask;
    } else {
        s_config[port] |= mask;
    }
    return exp_write_reg(6 + port, s_config[port]);
}

esp_err_t bsp_io_expander_set_level(int pin, int level)
{
    ESP_RETURN_ON_FALSE(s_dev && s_shadow_valid, ESP_ERR_INVALID_STATE, TAG,
                        "not initialized");
    ESP_RETURN_ON_FALSE(pin >= 0 && pin < 16, ESP_ERR_INVALID_ARG, TAG, "pin");
    uint8_t mask = 1u << (pin & 7);
    int port = pin >> 3;
    if (level) {
        s_output[port] |= mask;
    } else {
        s_output[port] &= ~mask;
    }
    return exp_write_reg(2 + port, s_output[port]);
}

esp_err_t bsp_io_expander_get_level(int pin, int *level)
{
    ESP_RETURN_ON_FALSE(s_dev, ESP_ERR_INVALID_STATE, TAG, "not initialized");
    ESP_RETURN_ON_FALSE(pin >= 0 && pin < 16 && level, ESP_ERR_INVALID_ARG,
                        TAG, "pin");
    uint8_t val;
    ESP_RETURN_ON_ERROR(exp_read_reg(pin >> 3, &val), TAG, "read input");
    *level = (val >> (pin & 7)) & 1;
    return ESP_OK;
}

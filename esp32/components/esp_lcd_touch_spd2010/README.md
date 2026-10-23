# esp_lcd_touch_spd2010 (vendored, patched)

Vendored copy of `espressif/esp_lcd_touch_spd2010` **0.0.1** (Apache-2.0,
the version the SenseCAP Watcher vendor BSP pins), with local fixes:

- Validate device-supplied frame lengths (`read_len`, `next_packet_len`)
  before using them. The SPD2010 returns garbage length fields while its
  touch firmware boots; the original code trusted them and could overflow
  the on-stack sample buffers (10-finger HDP frame, 32-byte remain buffer)
  and hang/corrupt the caller right when the controller finishes booting.
- Bound the HDP remain-data polling loop (`TP_HDP_MAX_CHUNKS`).

Modified 2026 by CodeCraft AI Hardware contributors. Original README below.

---

# ESP LCD Touch SPD2010 Controller

[![Component Registry](https://components.espressif.com/components/espressif/esp_lcd_touch_spd2010/badge.svg)](https://components.espressif.com/components/espressif/esp_lcd_touch_spd2010)

Implementation of the SPD2010 touch controller with [esp_lcd_touch](https://github.com/espressif/esp-bsp/tree/master/components/lcd_touch/esp_lcd_touch) component.

| Touch controller | Communication interface |    Component name     |                                Link to datasheet                                |
| :--------------: | :---------------------: | :-------------------: | :-----------------------------------------------------------------------------: |
|     SPD2010      |           I2C           | esp_lcd_touch_spd2010 | [PDF](https://dl.espressif.com/AE/esp-iot-solution/SPD2010(L-WEA2010)_0.50.pdf) |

## Add to project

Packages from this repository are uploaded to [Espressif's component service](https://components.espressif.com/).
You can add them to your project via `idf.py add-dependancy`, e.g.
```
    idf.py add-dependency "espressif/esp_lcd_touch_spd2010==*"
```

Alternatively, you can create `idf_component.yml`. More is in [Espressif's documentation](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/tools/idf-component-manager.html).

## Example use

I2C initialization of the touch component.

```
    esp_lcd_panel_io_i2c_config_t io_config = ESP_LCD_TOUCH_IO_I2C_SPD2010_CONFIG();

    esp_lcd_touch_config_t tp_cfg = {
        .x_max = CONFIG_LCD_HRES,
        .y_max = CONFIG_LCD_VRES,
        .rst_gpio_num = -1,
        .int_gpio_num = -1,
        .levels = {
            .reset = 0,
            .interrupt = 0,
        },
        .flags = {
            .swap_xy = 0,
            .mirror_x = 0,
            .mirror_y = 0,
        },
    };

    esp_lcd_touch_handle_t tp;
    esp_lcd_touch_new_i2c_spd2010(io_handle, &tp_cfg, &tp);
```

Read data from the touch controller and store it in RAM memory. It should be called regularly in poll.

```
    esp_lcd_touch_read_data(tp);
```

Get one X and Y coordinates with strength of touch.

```
    uint16_t touch_x[1];
    uint16_t touch_y[1];
    uint16_t touch_strength[1];
    uint8_t touch_cnt = 0;

    bool touchpad_pressed = esp_lcd_touch_get_coordinates(tp, touch_x, touch_y, touch_strength, &touch_cnt, 1);
```

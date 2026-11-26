/* SenseCAP Indicator display: ST7701S 480x480 RGB panel.
 * The panel's register channel is a bit-banged 9-bit 3-wire SPI (SCK/SDO on
 * GPIO, chip select on the TCA9535 expander); pixels stream over the 16-bit
 * RGB bus through esp_lcd. Framebuffer lives in octal PSRAM. */
#pragma once

#include <stdint.h>
#include "esp_err.h"
#include "bsp_pins.h"
#include "esp_lcd_panel_ops.h"
#include "esp_lcd_panel_rgb.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Task context only. One display-owning task or an application mutex must
 * serialize init, backlight access and all operations on the borrowed panel.
 * No ISR or non-blocking button/UI callback calls. LVGL calls still need the
 * application's LVGL lock. Native RGB interrupt callbacks must not block.
 * Initialize during serialized board startup, before touch/RP2040 consumers;
 * no other task may change the LCD CS/reset or register-channel GPIOs.
 *
 * Initializes the IO expander, releases the touch/RP2040 resets, creates the
 * esp_lcd RGB panel (480x480, RGB565, framebuffer in PSRAM) and runs the
 * ST7701S register init for the GX screen variant. The native DX build profile
 * instead releases the vendor-defined LCD CS/reset outputs without register
 * writes. The selected profile is not physical panel/revision detection;
 * see devices/sensecap-indicator/reference/display-variants.md.
 * out_panel may be NULL; otherwise it is cleared on failure. Repeated success
 * returns the same panel without pulsing shared reset or reprogramming it.
 * Only a fully initialized panel is published. Failed init deletes its RGB
 * driver; complete cleanup allows retry before other reset consumers start.
 * Incomplete cleanup blocks reuse until reset. Shared bus/rails stay owned
 * by the board; their state and panel contents are uncertain after errors.
 * Successful panel/framebuffer allocations live until reset. Do not delete
 * or independently reset the borrowed panel, expander or bus.
 *
 * Each expander access has its documented mutex/I2C wait; vendor fixed delays
 * and bit-bang GPIO waits add to init. No hard total deadline is promised.
 * In this IDF v6.0.2 single-framebuffer configuration, draw_bitmap copies an
 * external caller buffer synchronously into the RGB framebuffer. Keep that
 * buffer immutable through the call. A pointer obtained from the native RGB
 * framebuffer API is instead continuously read by DMA; it is borrowed and
 * must never be freed. Framebuffer updates may tear; transfer/copy completion
 * does not prove visible frame completion. No physical recovery is verified. */
esp_err_t bsp_display_init(esp_lcd_panel_handle_t *out_panel);

/* Start with DEFAULT, then select application policy before initialization.
 * Default preserves 18 MHz, no bounce buffer, RP2040 running and GPIO backlight.
 * hold_rp2040_reset disables the co-processor and its attached functions;
 * it is not a buzzer-only mute. BMP power follows the same enable policy.
 * bounce_buffer_lines must divide 480 (0 disables); two internal RAM buffers
 * consume lines * 480 * 2 bytes each. Clock must be 1..18 MHz; lower clocks
 * still require application/panel verification.
 * backlight_pwm_hz=0 retains GPIO on/off; otherwise reserves low-speed LEDC
 * timer 0/channel 0 at 8-bit resolution for the process lifetime. Choose a
 * frequency appropriate to the application; SDK rejects unsupported values.
 * Reinitialization with different options returns INVALID_STATE, not success.
 * Failed initialization may leave board reset/power and PWM resources configured;
 * retry only during serialized startup. No fade service is needed. */
typedef struct {
    uint32_t pclk_hz;
    unsigned bounce_buffer_lines;
    bool hold_rp2040_reset;
    uint32_t backlight_pwm_hz;
} bsp_display_config_t;
#define BSP_DISPLAY_CONFIG_DEFAULT() ((bsp_display_config_t){ .pclk_hz = BSP_LCD_PCLK_HZ })
esp_err_t bsp_display_init_with_config(const bsp_display_config_t *config,
                                     esp_lcd_panel_handle_t *out_panel);

/* Ready PWM display only; percent 0..100. Serializes like backlight_set.
 * SDK errors propagate without aborting; getter tracks last successful request,
 * not physical brightness. GPIO-only displays return INVALID_STATE. */
esp_err_t bsp_display_brightness_set(unsigned percent);
unsigned bsp_display_brightness_get(void);

/* Backlight on GPIO 45, active high; requires ready state. GPIO errors
 * propagate and leave physical state uncertain. Getter returns only the
 * last successful requested value (false initially), not a measurement. */
esp_err_t bsp_display_backlight_set(bool on);

bool bsp_display_backlight_get(void);

#ifdef __cplusplus
}
#endif

<p align="right"><strong>English</strong> · <a href="README.zh_CN.md">简体中文</a></p>

# esp-idf/sensecap-indicator

Device knowledge: [AGENTS.md](../AGENTS.md). Hardware facts are maintained there and in its references.

Build/check: `codecraft validate --target esp-idf/sensecap-indicator` from the repository root.

See `target.json` for examples, native interface references and exact toolchain selection. Build results and device observations are separate.

BSP public headers live in `bsp/include/`; implementation and private headers live in `bsp/src/`. Each example has its own `bsp.cmake` and managed dependency manifest. Exported projects include that example’s selected interfaces and source closure.

GX and DX display assemblies have sourced native profiles. Use `--variant gx` or `--variant dx` with context, new or focused Target validation. The full Target validation builds both. The repository build default is GX; exports record their selected variant and use it as the native default. Panel/PCB identity and revision compatibility remain unknown until established separately. See [pinned vendor evidence](../AGENTS.md).

## Target implementation

Device behavior, reference interfaces and example purposes are documented in the [Device implementation references](../AGENTS.md#implementation-references).

### Layout

- `bsp/include/bsp_pins.h` — every pin and timing constant, cited.
- `bsp/src/bsp_io_expander.c` — TCA9535 driver (shadowed output/config regs).
- `bsp/src/bsp_display.c` — ST7701S init + esp_lcd RGB panel + backlight.
- `bsp/src/bsp_button.c` — GPIO 38 button, debounced task.
- `bsp/src/bsp_touch.c` — FT6336U/GT911 auto-probe touch (esp_lcd_touch).
- `sdkconfig.defaults` — shared memory/flash config (symlinked into each
  example).
- `examples/display_color_bars` — 8 vertical color bars + backlight on.
- `examples/button_backlight` — white screen, button toggles backlight.
- `examples/touch_read` — logs touch coordinates as you press the screen.

## Toolchain

The executable version declaration in `target.json` requires ESP-IDF 6.0.2 for esp32s3. Activate that SDK before building.

In an exported application, use `codecraft build-project .`; reference-repository validation commands do not replace application lock checks.

## Application display options

`bsp_display_init()` retains the vendor 18 MHz/GPIO-backlight startup, including
RP2040 release and sensor power. Use `bsp_display_init_with_config()` to choose
application policy before display/touch consumers start:

```c
bsp_display_config_t cfg = BSP_DISPLAY_CONFIG_DEFAULT();
cfg.pclk_hz = 16000000;
cfg.bounce_buffer_lines = 10; // Example depth, not a measured optimum.
cfg.hold_rp2040_reset = true; // Disables co-processor functions, not only sound.
// Optional: cfg.backlight_pwm_hz = <application-selected frequency>;
esp_err_t err = bsp_display_init_with_config(&cfg, &panel);
// Handle err; do not blindly abort a running UI on an operation failure.
```

Two 10-line RGB565 bounce buffers consume 19,200 bytes of internal RAM, plus SDK
allocation overhead. Choose a depth dividing 480; zero retains direct PSRAM DMA.
The clock range accepted by this API is 1..18 MHz, not a verified panel range.
For the application's reported 64-byte data-cache configuration, put
`CONFIG_ESP32S3_DATA_CACHE_LINE_64B=y` in its sdkconfig.defaults and regenerate
its actual sdkconfig through IDF. DMA burst size is a different setting. The
reference examples retain their cache defaults; video stability needs testing
under the application's decode/network/memory load.

Holding RP2040 reset also keeps the BMP rail off; do not use co-processor
sensors or microSD in this mode. Default initialization preserves existing
applications. Reinitializing with different options returns INVALID_STATE.

Nonzero `backlight_pwm_hz` reserves low-speed LEDC timer 0/channel 0 at 8-bit
resolution; choose the frequency and avoid resource conflicts in the app.
`bsp_display_brightness_set(0..100)` returns errors without aborting and needs
no fade service. Backlight on/off maps to 100/0 in PWM mode. PWM is opt-in;
GPIO-only mode rejects percentage dimming. Access must be serialized. Failed
initialization can leave timer/reset/power state configured; retry only during
startup, and do not transfer those resources to another consumer implicitly.

GX + FT6336U now uses bounded 180-degree touch mapping; GX GT911 and DX retain
existing mappings. See the Device's sourced application observations and
unverified hardware revision limits. These options do not establish physical
acceptance of this repository firmware.

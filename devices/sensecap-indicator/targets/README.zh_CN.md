<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# esp-idf/sensecap-indicator

设备知识：[AGENTS.md](../AGENTS.md)。硬件事实在该入口及其来源中维护。

在仓库根目录构建/检查：`codecraft validate --target esp-idf/sensecap-indicator`。

示例、原生接口引用和工具链选择见 `target.json`。构建结果与设备观测分别记录。

BSP 公共头文件位于 `bsp/include/`，实现与私有头文件位于 `bsp/src/`。每个示例拥有独立的 `bsp.cmake` 和托管依赖声明；导出项目包含该示例选择的接口及源码闭包。

GX 与 DX 屏幕组件已有带来源的原生配置。context、new 和定向 Target 验证可使用 `--variant gx` 或 `--variant dx`；完整 Target 验证会构建两种配置。仓库默认构建 GX；导出会记录所选变体并将其设为原生默认值。面板／PCB 身份及修订兼容性须单独确认，目前仍为 unknown。参见[固定版本厂商来源](../AGENTS.md)。

## Target 实现

设备行为、接口参考和示例用途见 [Device 实现参考](../AGENTS.md#implementation-references)。Target 负责当前软件栈下的可构建实现、原生接口绑定和构建配置。

## 工具链

`target.json` 要求 ESP-IDF 6.0.2、esp32s3；构建前激活该版本 SDK。

导出应用使用 `codecraft build-project .`；仓库级验证命令不能替代应用锁文件检查。

## 应用显示配置

`bsp_display_init()` 保留厂商 18 MHz／GPIO 背光启动行为，包括释放 RP2040
复位和开启传感器电源。显示／触摸使用者启动前，可通过
`bsp_display_init_with_config()` 选择应用策略：

```c
bsp_display_config_t cfg = BSP_DISPLAY_CONFIG_DEFAULT();
cfg.pclk_hz = 16000000;
cfg.bounce_buffer_lines = 10; // 示例深度，不是实测最优值。
cfg.hold_rp2040_reset = true; // 停用协处理器功能，不只是静音。
// 可选：cfg.backlight_pwm_hz = <应用选择的频率>;
esp_err_t err = bsp_display_init_with_config(&cfg, &panel);
// 处理 err；不要在运行中的 UI 遇到操作错误时直接终止程序。
```

两个 10 行 RGB565 缓冲共占 19,200 字节内部 RAM，另有 SDK 分配开销。
行数须整除 480；零保留直接 PSRAM DMA。接口接受 1..18 MHz，
不代表整个范围都已获得面板验证。要采用应用报告的 64 字节数据缓存行，
在应用 sdkconfig.defaults 中设置 `CONFIG_ESP32S3_DATA_CACHE_LINE_64B=y`，
并通过 IDF 重新生成实际 sdkconfig。DMA burst size 是另一项设置。
参考示例保留原缓存默认值；播放稳定性须在应用的解码、网络和内存负载下实测。

保持 RP2040 复位时，也会关闭 BMP 电源；该模式不要使用协处理器传感器或
microSD。默认初始化保留现有应用行为。使用不同配置重复初始化返回 INVALID_STATE。

非零 `backlight_pwm_hz` 独占低速 LEDC timer 0/channel 0，使用 8 位分辨率；
应用须选择频率并避免资源冲突。`bsp_display_brightness_set(0..100)` 返回错误，
不终止程序，也不依赖 fade 服务。PWM 模式的背光开关对应 100/0。
PWM 为可选项；GPIO 模式拒绝百分比调光。访问必须串行化。
初始化失败可能保留定时器、复位和电源配置；只能在启动期间重试，
不要隐式将这些资源转交其他使用者。

GX＋FT6336U 使用带边界检查的 180° 触摸映射；GX GT911 和 DX 保留原映射。
来源及硬件修订限制见 Device 中的应用观测；这些配置不代表当前仓库固件已完成实机验收。

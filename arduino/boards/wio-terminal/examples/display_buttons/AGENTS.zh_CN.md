<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# display_buttons — Agent 说明

## 目的

用一个低闪烁草图同时验证 ILI9341 显示、三个顶部按键与五向开关。它是 LCD
方向、按键极性与消抖行为的参考示例。

## 硬件事实

- LCD：2.4 英寸 320x240 ILI9341，使用 SPI3；SCK 68、MOSI 67、MISO 66、
  CS 69、DC 70、复位 71、背光 72。`setRotation(3)` 产生全部示例使用的
  320 像素宽横向视图。来源：原理图 v1.2 第 07 页、核心 1.8.6
  `variants/wio_terminal/variant.h` 与内置 LCD `User_Setup.h`；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 顶部按键：A=GPIO 28、B=29、C=30。来源：原理图第 05 页与核心
  `variant.h`（`WIO_KEY_A/B/C`）。
- 五向开关：上=31、左=32、右=33、下=34、按下=35。来源：原理图第 04 页
  与核心 `variant.h`（`WIO_5S_*`）。
- 所有开关均低电平有效并带上拉。草图还启用了 SAMD51 内部上拉，并对每个
  输入做 20 ms 消抖。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/display_buttons
```

除 `Seeeduino:samd@1.8.6` 内置 LCD 库外不需要外部库。烧录流程与工具链见
板卡 [AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)，且必须取得明确授权。

## 预期行为

- LCD 标题为 `WIO TERMINAL INPUT`。
- 显示 `A B C`、`UP LEFT PRESS RIGHT DOWN` 灰色按键框；按住时对应框变绿。
- 115200 baud 串口在每次稳定状态变化时打印一行，例如 `UP pressed` 与
  `UP released`。

## 上机验证

1. 获得授权后构建并上传。
2. 以 115200 baud 打开串口监视器。
3. 依次按下并释放全部八个输入。
4. 确认每次操作恰好出现一条按下与一条释放日志，且 LCD 对应框同步变化。
5. 确认显示为横向且所有标签完整、无裁切。

2026-10-28 已实测：横向 LCD 渲染完整输入图，A/B/C 与五向开关事件同时出现
在 LCD 和串口上，用户已确认示例。未做电气时序测量。

## 限制

- 仅轮询，不使用中断，也不负责从休眠唤醒。
- 20 ms 消抖窗口是有意设计；未经实际按键测试不要缩短。
- 消抖由人工按键确认，未用逻辑分析仪测量；20 ms 仍是经验值。

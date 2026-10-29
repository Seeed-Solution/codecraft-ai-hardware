<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# oled_status — Agent 说明

## 目的

验证套件 128x64 OLED 在共享硬件 I2C 总线上的显示，并在不分配完整帧缓冲的
情况下显示运行时间。

## 硬件事实

- 套件 OLED 是 0.96 英寸 128x64 SSD1306/SSD1315 兼容屏，接硬件 I2C，
  SDA 为 A4/引脚 18，SCL 为 A5/引脚 19。来源：套件 wiki 默认引脚表和
  Lesson 7；Seeeduino Lotus v1.1 原理图。
- 厂商引脚表给出的 0x78 是 8 位写地址。本示例使用 7 位形式 0x3C，并兼容
  备用的 0x3D。来源：套件 wiki 与 Grove OLED wiki。
- 使用 U8x8 文本模式而不是 U8g2 完整帧缓冲，因为 ATmega328P 只有 2 KB
  SRAM。来源：Seeeduino Lotus wiki 的内存参数与 U8g2/U8x8 API。
- 示例调用 `setFlipMode(1)`，把显示输出旋转 180 度。这是应用层的方向选择，
  不是 I2C 属性。来源：U8g2 `U8X8::setFlipMode()` API。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/oled_status
```

## 预期行为

- 9600 波特率下，setup 打印一次 `oled_status: OLED on I2C 0x3C/0x3D`。
- 屏幕显示四行 16 列文本：`GROVE BEGINNER`、`KIT / OLED TEST`、
  `128x64 I2C OLED` 和 `uptime 0 s`。最后一行约每秒递增。
- 串口监视器同时打印 `uptime 1 s`、`uptime 2 s` 等。
- 如果 0x3C 不应答而 0x3D 应答，示例会在初始化前调用
  `setI2CAddress(0x7A)`（0x3D 的 8 位形式）。
- 如果两个地址都不应答，串口打印
  `OLED init failed; expected 0x3C or 0x3D`；屏幕可能保持空白，但串口仍
  继续打印运行时间。

## 物理测试

1. 运行 `i2c_scan`，记录出现的是 0x3C 还是 0x3D。
2. 打开 9600 波特率串口，确认四行文本出现。
3. 至少观察三次计数更新，确认屏幕与串口一致。
4. 如果文字上下颠倒，说明面板安装方向与 `setFlipMode(1)` 假设相反；
   把参数改为 0 后重新检查。不要通过修改 I2C 地址来“修复”方向。

## 限制

- U8x8 文本模式只支持 16 列 x 8 行、每格 8x8；本示例不测试图形、滚动或
  自定义字体。
- I2C ACK 只能证明控制器有响应，不能证明面板健康；有 ACK 但黑屏可能是
  对比度、方向或面板故障。
- 本示例只支持 SSD1306/SSD1315 兼容控制器。
- 构建通过不能证明 OLED 或其接线正常。

<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# air_pressure_spa06 — Agent 说明

## 目的

读取唯一支持的、2025 年 10 月后套件版本上的 SPA06-003，并打印气压、
温度和气压高度。

## 硬件事实

- 本板级目录只支持 2025 年 10 月后的套件版本。当前套件 wiki 说明
  2025.10 之后的版本使用 SPA06-003。来源：套件 wiki 的版本迭代说明和
  Lesson 9。
- SPA06-003 默认接 I2C 0x77（SDO 拉高），SDO 拉低时使用 0x76。来源：
  SPA06-003 wiki 和 `SPL07-003.h` 地址常量。
- 固定的 `Seeed Arduino SPA06` 1.0.1 库在 `begin()` 中校验器件 ID 0x11。
  来源：库 `SPL07-003.cpp` / `SPL07-003.h`。
- 示例配置为 4 Hz 连续测量、32 次气压采样和 1 次温度采样。这些是应用
  设置，不是 SPA06 唯一可用的配置。来源：库 API 常量。
- `calcAltitude()` 使用 1013.25 hPa 作为标准海平面气压。来源：库实现。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/air_pressure_spa06
```

## 预期行为

- setup 先探测 0x77，再探测 0x76，并打印 `SPA06-003 on I2C 0x77` 或
  `SPA06-003 on I2C 0x76`。
- 有可用测量值时打印如下行：

```text
Pres: 100000.0 Pa  Temp: 25.0 C  Altitude: 123.4 m
```

- 如果两个地址都不应答，打印 `SPA06-003 not found at 0x77 or 0x76` 并停止
  输出读数。它不会重试，修好接线后需要重新上电。

## 物理测试

1. 运行 `i2c_scan`，记录出现的是 0x77 还是 0x76。
2. 打开 9600 波特率串口，确认探测到的地址。
3. 确认读数持续输出且不阻塞；配置速率为 4 Hz，但实际节奏主要受串口打印
   限制。
4. 带着套件上下走一段楼梯，比较若干次采样后的气压和高度趋势。

## 限制

- 高度是相对 1013.25 hPa 的气压高度，不是 GPS 高度，也未按 QNH 修正。
- 传感器未做实验室级绝对精度标定；相对变化比绝对数值更有意义。
- 固定的 4 Hz/32 次采样配置是示例选择；修改后需要重新检查噪声和响应时间。
- 0x77/0x76 上的 ACK 不是身份校验；示例必须通过 SPA06 库的器件 ID 0x11
  匹配后才会报告测量值。

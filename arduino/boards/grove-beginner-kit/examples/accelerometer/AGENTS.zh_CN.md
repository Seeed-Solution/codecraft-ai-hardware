<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# accelerometer — Agent 说明

## 目的

读取套件 LIS3DHTR 加速度计，并用未标定的倾斜/晃动启发式控制套件 LED。

## 硬件事实

- 套件三轴数字加速度计是 ST LIS3DHTR，接 I2C 0x19。来源：套件 wiki 默认
  引脚表和 Lesson 10。
- 固定的
  `Grove-3-Axis-Digital-Accelerometer-2g-to-16g-LIS3DHTR` 1.2.4 库默认使用
  0x18，所以示例显式调用 `lis.begin(Wire, 0x19)`。来源：库 `LIS3DHTR.h`
  的默认地址和 `LIS3DHTR.cpp`。
- 套件 LED 接 D4，高电平点亮。来源：套件 wiki 默认引脚表。
- 示例配置 2 g 量程、50 Hz 输出数据率和高分辨率模式。这些是应用设置，
  不是传感器唯一可用的模式。来源：库 API。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/accelerometer
```

## 预期行为

- 传感器有响应时，setup 打印 `LIS3DHTR on I2C 0x19`。
- 每 200 毫秒打印一行，例如：

```text
X=0.00 g  Y=0.00 g  Z=1.00 g  |a|=1.00 g  LED=REST
```

- 静止时合矢量接近 1 g，但哪个轴读到 1 g 取决于板卡安装方向。
- 当 `||a| - 1 g| > 0.20 g`，或 `|X| > 0.60 g`，或 `|Y| > 0.60 g` 时
  LED 点亮，串口字段变为 `LED=ACTIVE`。
- 传感器缺失时，setup 打印 `LIS3DHTR not found at I2C 0x19`，LED 保持
  熄灭，不再输出采样。

## 物理测试

1. 运行 `i2c_scan` 并确认 0x19。
2. 打开 9600 波特率串口，确认静止时合矢量接近 1 g。
3. 缓慢倾斜套件，记录哪个轴变化以及变化方向。
4. 轻轻敲击或晃动，确认 LED 闪到 `ACTIVE`，停止运动后回到 `REST`。

## 2026-10-29 实测

Seeeduino Lotus v1.1 平放：`X≈-0.01 g`、`Y≈0.14 g`、`Z≈0.98 g`、
`|a|≈1.00 g`、`LED=REST`。Y 轴约 0.14 g 的偏置是器件零偏，不是倾斜。

该套件板平放时读数主要落在 Y/Z 轴，而非 X/Z。倾斜并晃动时 X 从 -0.82
变到 +1.16 g，Z 从 0.05 变到 1.40 g，|a| 从 0.14 变到 1.60 g；运动期间的
每个采样都输出 `LED=ACTIVE`，静止后回到 `LED=REST`。晃动后静止偏置变为
`X≈-0.03 g`、`Y≈0.12 g`。来源：实测结果。

## 限制

- 这不是标定的倾角仪或晃动检测器；偏移、比例误差和安装方向都未修正。
- 0.20 g 和 0.60 g 阈值是示例启发式，不是传感器指标。
- 2 g 量程在硬冲击时会饱和；不要用于冲击或碰撞测量。
- 必须显式传 0x19；使用库默认的 0x18 会在本套件上失败。
- 构建通过不等于上机通过；必须记录实际板卡的轴方向。

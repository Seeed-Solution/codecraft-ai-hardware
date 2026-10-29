<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# i2c_scan — Agent 说明

## 目的

枚举受支持的 2025 年 10 月后套件共享硬件 I2C 总线上有 ACK 的器件。调试
OLED、加速度计、DHT20 或 SPA06-003 前先运行本示例。

## 硬件事实

- SDA 为 A4（PC4，Arduino 引脚 18），SCL 为 A5（PC5，引脚 19）。来源：
  Seeeduino Lotus v1.1 原理图和核心 `standard` variant。
- 扫描输出 7 位地址。厂商文档可能打印 8 位写地址，例如 OLED 0x78 对应
  7 位 0x3C。来源：套件 wiki 默认引脚表和 OLED wiki。
- 本版本套件的预期地址：LIS3DHTR 0x19、OLED 0x3C 或 0x3D、DHT20 0x38、
  SPA06-003 0x77 或 0x76。来源：套件 wiki 与相应模块 wiki。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/i2c_scan
```

## 预期行为

9600 波特率下先打印标题，每个器件打印一行 `found 0xNN`，已知套件地址附带
标签，最后打印 `I2C scan complete: N device(s)`。每 3 秒重复一次。

## 物理测试

1. 通过 USB 给完整套件供电，打开 9600 波特率串口。
2. 确认完整套件出现 0x19、0x38、0x3C 或 0x3D、0x77 或 0x76。OLED 与
   气压地址的具体取值按文档允许的备选地址判断。
3. 若模块已拆出，断开一个已知器件，确认下一次扫描中其地址消失。

## 限制

- ACK 不代表身份校验；`air_pressure_spa06` 还会要求 SPA06-003 的器件 ID
  0x11 匹配。
- 地址缺失可能是接线、供电、上拉或总线被占用；不能单独证明模块损坏。
- 扫描只读，不配置也不初始化任何传感器。

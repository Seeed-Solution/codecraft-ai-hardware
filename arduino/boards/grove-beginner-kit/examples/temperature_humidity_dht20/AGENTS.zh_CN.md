<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# temperature_humidity_dht20 — Agent 说明

## 目的

读取唯一支持的、2025 年 10 月后套件版本上的 DHT20 温湿度传感器。

## 硬件事实

- 本板级目录只支持 2025 年 10 月后的套件版本。当前套件 wiki 说明 2025.10
  之后的版本使用 DHT20，黑色温湿度模块是该版本的识别特征。来源：套件
  wiki 的版本迭代说明和 DHT20 小节。
- DHT20 接 I2C `Wire`，固定的 `Grove Temperature And Humidity Sensor`
  2.0.2 库使用地址 0x38（`DEFAULT_IIC_ADDR`）。来源：库 `DHT.h`。
- 已发布的套件原理图早于该传感器，没有记录 DHT20 布线。必须先用
  `i2c_scan` 确认 0x38；不要为 I2C 的 DHT20 虚构 D3 接线。来源：套件
  原理图归档、套件 wiki 和 DHT20 库。
- I2C 与 OLED 等模块共享 A4/引脚 18（SDA）和 A5/引脚 19（SCL），5 V 逻辑。
  来源：Seeeduino Lotus v1.1 原理图。
- 示例使用 `DHT dht(0, DHT20)` 构造。第一个参数是旧构造函数的占位符；
  DHT20 模式只用 I2C，不使用数据引脚。来源：库 `DHT.cpp` / `DHT.h`。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/temperature_humidity_dht20
```

## 预期行为

- setup 在 9600 波特率下打印 `DHT20 on I2C 0x38`。
- 每秒成功读取时打印一行，例如：

```text
Humidity: 50.0 %  Temperature: 24.0 C
```

- 如果启动时 0x38 不存在，示例打印 `DHT20 not found at I2C 0x38`，然后
  停留在每秒一次的延时循环中。它不会重试，修好接线后需要重新上电。
- 探测成功后读取失败会打印
  `DHT20 read failed; check I2C 0x38 and sensor power`。

## 物理测试

1. 运行 `i2c_scan` 并确认 0x38。如果不存在，说明这不是受支持的版本或
   传感器未连接；本板级目录有意不提供 DHT11 后备示例。
2. 打开 9600 波特率串口，确认 `DHT20 on I2C 0x38`。
3. 在传感器附近轻轻呼气并等待若干次采样；湿度应先于温度变化。
4. 确认读数约每秒更新一次。

## 限制

- 这不是标定参考；DHT20 适合观察趋势和一般环境监测，不用于实验室测量。
- 固定 0x38 地址意味着该总线只能接一个 DHT20。
- 示例不计算露点或热指数，也不控制传感器加热器。
- 已发布的原理图没有 DHT20 数据通路；构建通过不能证明套件是 DHT20 版本。

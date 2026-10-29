<p align="right">
  <a href="README.md">English</a> · <strong>简体中文</strong>
</p>

# Grove Beginner Kit for Arduino

状态：**10/10 个 Grove 器件 demo 加 `i2c_scan` 已用 `arduino:avr@1.8.8`
完成构建验证，并已在 2025 年后版本的 Seeeduino Lotus v1.1 上完成上机
验证。** 套件每个模块都有独立示例；`i2c_scan` 是额外的总线诊断，不是
第 11 个器件。四个 I2C 器件、OLED、按键、电位器全程扫描、蜂鸣器音序、
光线与声音阈值、以及加速度计倾斜都产生了预期的串口输出与实际效果；
证据记录在
[docs/on-device-validation.md](../../../docs/on-device-validation.md)。
本板级目录只支持 2025 年 10 月后、使用 DHT20 与 SPA06-003 的套件版本；
2020 版 DHT11/BMP280 套件不在支持范围内。

## 已验证事实

- MCU：Microchip ATmega328P-MU，16 MHz，32 KB Flash（草图上限 32,256
  字节），2 KB SRAM，1 KB EEPROM，5 V 逻辑。Seeeduino Lotus v1.0 使用
  CH340 USB 转串口，v1.1 使用 CP2102N。来源：
  [Seeeduino Lotus wiki](https://wiki.seeedstudio.com/Seeeduino_Lotus/) 与
  [v1.1 原理图](https://files.seeedstudio.com/wiki/Seeeduino_Lotus/res/Seeeduino%20Lotus%20v1.1%20SCH.pdf)。
- 板卡标识：厂商核心定义 `Seeeduino:avr:Lotus`，其 `boards.txt` 条目使用
  ATmega328P、`standard` variant、16 MHz、`AVR_UNO`、32,256 字节草图上限
  和 115200 波特率 Optiboot 上传。本仓库用 `arduino:avr@1.8.8` 构建功能
  等价的 `arduino:avr:uno` 目标。旧厂商核心自带的 avr-gcc 4.9.2 在 macOS
  上仅有 i386 可执行文件，且旧索引声明的 avr 1.3.0 压缩包 URL 返回 404。
  来源：
  [Seeeduino AVR 1.2.1 归档](https://raw.githubusercontent.com/Seeed-Studio/Seeed_Platform/master/Seeeduino_AVR_v1.2.1.tar.bz2)
  与 [ArduinoCore-avr 1.8.8](https://github.com/arduino/ArduinoCore-avr/tree/1.8.8)。
- 硬件 I2C：A4 为 SDA（PC4，Arduino 引脚 18），A5 为 SCL（PC5，引脚 19），
  Grove 接口为 5 V 逻辑。来源：Seeeduino Lotus v1.1 原理图和核心
  `standard` variant。
- 套件 LED：Grove LED 接 D4，高电平点亮，板上可调电阻可调亮度。D4 在该
  核心上不是 PWM 引脚。来源：套件 wiki 默认引脚表和 Lesson 1。
- 按键：Grove Button 接 D6。厂商示例使用 `INPUT`，按下读到 `HIGH`，说明
  模块自带下拉。来源：套件 wiki Lesson 2。
- 旋转电位器：Grove Rotary Potentiometer 接 A0；`analogRead()` 在 5 V
  参考下返回 0-1023。来源：套件 wiki Lesson 3。
- 蜂鸣器：Grove 无源压电蜂鸣器接 D5，需要方波（如 `tone()`），静态直流
  电平不会持续发声。来源：套件 wiki Lesson 4。
- 光线传感器：Grove Light 接 A6；厂商示例在 raw < 200 时点亮 LED。这是
  未标定的模拟量，不是 lux。来源：套件 wiki Lesson 5。
- 声音传感器：Grove Sound 接 A2；厂商示例用 raw > 400 判断声音峰值。
  输出是模拟包络，不是标定的 dB SPL，也不是录音。来源：套件 wiki Lesson 6。
- OLED：0.96 英寸 128x64 SSD1306/SSD1315 兼容屏，接 I2C。套件 wiki 给出
  的 0x78 是 8 位写地址，对应 7 位地址 0x3C；示例也兼容 0x3D。来源：
  套件 wiki 默认引脚表和
  [OLED wiki](https://wiki.seeedstudio.com/Grove-OLED-Display-0.96-SSD1315/)。
- 温湿度：支持的 2025 年 10 月后套件使用 DHT20，接 I2C。当前厂商示例
  调用 `Wire.begin()`，Grove DHT 库固定使用 I2C 地址 0x38，wiki 说明
  黑色温湿度模块是该版本的识别特征。来源：当前套件 wiki 的 DHT20 小节和
  [Grove_Temperature_And_Humidity_Sensor](https://github.com/Seeed-Studio/Grove_Temperature_And_Humidity_Sensor)。
- 气压：支持的 2025 年 10 月后套件使用 SPA06-003，接 I2C，默认 0x77，
  可选 0x76。SPA06 库会校验器件 ID 0x11。来源：
  [SPA06-003 wiki](https://wiki.seeedstudio.com/grove_barometer_sensor_spa06_003/)。
- 加速度计：ST LIS3DHTR 接 I2C 0x19。来源：套件 wiki 默认引脚表和
  Lesson 10。

以上未变更的板级连接来自
[Grove Beginner Kit wiki](https://wiki.seeedstudio.com/Grove-Beginner-Kit-For-Arduino/)
和套件的
[原理图/PCB 归档](https://files.seeedstudio.com/wiki/Grove-Beginner-Kit-For-Arduino/res/Grove-Beginner-Kit-for-Arduino-SCH-PCB.zip)
（公开原理图日期为 2020 年，版本 v1.4）。该原理图早于 DHT20/SPA06-003
版本，没有记录这两个传感器的布线。对 0x38 和 SPA06-003 地址的判断以当前
wiki 和目标套件的 `i2c_scan` 为准。

## 未提供

套件没有 Wi-Fi/BLE 无线、电机驱动、继电器、电池/充电器、摄像头或数字录音。
“声音传感器”是模拟包络输出。不要添加网络、配对、账户、遥测或云端代码：
本仓库只做本地硬件参考。

## 工具链与依赖

本目录使用的构建目标：

- 核心：`arduino:avr@1.8.8`
- FQBN：`arduino:avr:uno`
- 串口监视器：所有 `examples/` 示例均使用 9600 波特率

厂商 IDE 板卡（仅记录，不用于本仓库构建）：`Seeeduino:avr:Lotus`，旧版
索引为
`https://raw.githubusercontent.com/Seeed-Studio/Seeed_Platform/master/package_legacy_seeeduino_boards_index.json`。
必须使用该板时固定 `Seeeduino:avr@1.2.1`；不要使用索引中的 1.3.0，
其归档 URL 已不存在。

安装精确构建核心和库：

```bash
arduino-cli core install arduino:avr@1.8.8
cd arduino/boards/grove-beginner-kit
while IFS= read -r library; do
  [[ -z "$library" || "$library" == \#* ]] && continue
  arduino-cli lib install "$library"
done < libraries.txt
```

| 依赖 | 版本 | 使用者 |
|---|---|---|
| `arduino:avr` | 1.8.8 | 所有示例 |
| `U8g2` | 2.36.19 | `oled_status` |
| `Grove Temperature And Humidity Sensor` | 2.0.2 | `temperature_humidity_dht20` |
| `Seeed Arduino SPA06` | 1.0.1 | `air_pressure_spa06` |
| `Grove-3-Axis-Digital-Accelerometer-2g-to-16g-LIS3DHTR` | 1.2.4 | `accelerometer` |

## 能力矩阵

| 能力 | 硬件 | 默认连接 | 示例 | 状态 |
|---|---|---|---|---|
| I2C 诊断 | ATmega328P TWI | SDA A4/18，SCL A5/19 | `i2c_scan` | 已上机验证 |
| 数字输出 | Grove LED | D4，高电平点亮 | `led_blink` | 已上机验证 |
| 数字输入 | Grove Button | D6，按下为 HIGH | `button_led` | 已上机验证 |
| 模拟输入 | 旋转电位器 | A0 | `potentiometer` | 已上机验证；全程 0-1023 扫描 |
| PWM/音频输出 | 无源蜂鸣器 | D5 | `buzzer_tones` | 已上机验证 |
| 模拟传感器 | 光线传感器 | A6 | `light_sensor` | 已上机验证；遮挡时 LED 点亮 |
| 模拟传感器 | 声音传感器 | A2 | `sound_sensor` | 已上机验证；拍手峰值 561/686 |
| 显示 | SSD1306/SSD1315 OLED | I2C 0x3C（0x3D 备用） | `oled_status` | 已上机验证；显示已确认 |
| 温湿度 | DHT20 | I2C 0x38 | `temperature_humidity_dht20` | 已上机验证 |
| 气压 | SPA06-003 | I2C 0x77/0x76 | `air_pressure_spa06` | 已上机验证 |
| 运动 | LIS3DHTR | I2C 0x19 | `accelerometer` | 已上机验证；各轴随倾斜变化 |

“已上机验证”指该示例于 2026-10-29 在目标板卡上运行，串口输出与预期
行为一致，且物理效果已由操作者确认。模拟类示例输出的是 ADC 原始计数
而非工程单位；室内光照、声音峰值和加速度静止基线等实测值记录在各示例
自己的 `AGENTS.md` 中。

## 构建与上传

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/<sketch>

./tools/validate.sh --arduino grove-beginner-kit
```

2026-10-29 已通过 `arduino:avr:uno` 目标和 CP2102N USB 桥，将
10 个器件 demo 和 `i2c_scan` 全部上传到 Seeeduino Lotus v1.1。连接板卡后
先确认端口：

```bash
arduino-cli board list
arduino-cli upload -p <port> \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/led_blink
```

烧录必须获得当前会话的明确授权。若某个 Lotus 版本无法用该等价目标上传，
请在 Arduino IDE 中选择厂商核心的 `Seeeduino Lotus` 并继续使用同一草图；
不要通过改引脚来绕过上传问题。

## 已知限制与来源冲突

- **只支持 2025 年 10 月后版本。** 先运行 `i2c_scan` 并确认 DHT20 的
  0x38；如果不存在，停止并确认硬件版本，不要加入 DHT11 后备代码，也不要
  套用 2020 版的 D3 接线。
- **公开原理图早于当前传感器。** 它没有 DHT20 或 SPA06-003 的布线信息。
  DHT20 是否存在以 0x38 探测结果为准；气压传感器必须通过 SPA06 库的
  器件 ID 0x11 校验后才能声明受支持。
- **D4 没有 PWM。** LED 示例只能开/关。若需 PWM 调光，必须拆出 LED 模块
  并接到 D3。
- **按键模块自带下拉。** 使用 `INPUT`；`INPUT_PULLUP` 会让空闲按键读成
  按下。
- **I2C 共享且为 5 V。** 同一块板上可能同时存在 0x19/0x3C或0x3D/0x38/0x77。
  不要把仅支持 3.3 V 的外部器件直接接到该总线。
- **SRAM 只有 2 KB。** 优先使用 U8x8 文本模式或流式输出；不要未检查构建
  大小就加入全屏 framebuffer 或大块全局缓冲。
- **模拟 raw 值不是工程单位。** 电位器、光线和声音示例只报告 ADC 计数
  与估算毫伏。lux 与 dB SPL 需要本仓库尚未进行的标定。

## 示例

- `i2c_scan` — 扫描 ATmega328P 硬件 I2C 总线，标注套件已知的
  0x19、0x38、0x3C/0x3D、0x76/0x77 器件。
- `led_blink` — 让 D4 上的 Grove LED 以 1 Hz 闪烁并打印两个相位。
- `button_led` — 用 20 ms 消抖把 D6 按键状态镜像到 D4 LED。
- `potentiometer` — 读取 A0，打印 raw/mV/百分比，并在中点用滞回切换 LED；
  不做 PWM 调光。
- `buzzer_tones` — 在 D5 无源蜂鸣器上播放 440、880、1760、2000 Hz 音调。
- `light_sensor` — 读取 A6，使用厂商 raw 200 阈值和滞回在暗处点亮 LED。
- `sound_sensor` — 报告 A2 的 50 ms 峰值，超过厂商 raw 400 阈值时点亮 LED。
- `oled_status` — 在 0x3C/0x3D OLED 上显示标题、128x64/I2C 标识和运行时间。
- `temperature_humidity_dht20` — 探测 I2C 0x38，每秒读取 DHT20。
- `air_pressure_spa06` — 探测 0x77/0x76 并读取 SPA06-003。
- `accelerometer` — 读取 0x19 的 LIS3DHTR，用未标定的倾斜/晃动启发式点亮 LED。

每个示例都有独立的双语 `AGENTS.md` 文件，写明硬件事实、预期串口/OLED
输出、物理检查和限制。

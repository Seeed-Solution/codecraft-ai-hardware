<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# Grove Beginner Kit Agent 说明

本文件适用于 `arduino/boards/grove-beginner-kit/` 及其所有示例。根目录
`AGENTS.md`、`arduino/AGENTS.md` 和 `docs/development/ai-guide.md` 仍然
有效。

## 目标套件版本

本板级目录**只支持 2025 年 10 月后的 Grove Beginner Kit for Arduino**。
支持的传感器为 I2C 0x38 的 DHT20 和 I2C 0x77/0x76 的 SPA06-003。2020 版
DHT11/BMP280 不在支持范围内：不要为它添加旧示例、兼容分支或引脚重映射。

进行传感器工作前必须在目标套件上运行 `i2c_scan`。DHT20 必须出现 0x38；
SPA06-003 必须通过库的器件 ID 0x11 校验。已发布套件原理图是 2020 版，
没有记录这两个传感器的连接；当前 wiki 与目标套件的扫描结果是版本判断
依据。

## 先读

1. `git status --short --branch`；保留无关改动。
2. 本目录 `README.md` 和 `docs/vendor-sources.md` 中对应行。
3. 修改草图的 `AGENTS.md`。
4. 相关厂商源：套件 wiki 引脚表、套件原理图、Lotus v1.1 原理图，或固定
   版本的库源码。

绝不臆造引脚、总线、地址、传感器版本、极性或单位。若来源没有定义，必须
询问用户或实测。

## 固定平台事实

- 构建核心：`arduino:avr@1.8.8`。
- 构建 FQBN：`arduino:avr:uno`。
- 厂商板定义：`Seeeduino:avr:Lotus`；其 MCU、时钟、variant、Flash/RAM
  上限和 Optiboot 上传参数与 UNO 目标一致。旧核心的 i386 工具链和缺失的
  1.3.0 归档使它不适合作为仓库构建核心。
- MCU：ATmega328P-MU，16 MHz，草图上限 32,256 字节，2 KB SRAM，
  1 KB EEPROM，5 V 逻辑。
- 串口监视器：9600 波特率。
- LCD：无。唯一显示屏是 128x64 I2C OLED。
- 没有无线、云、配对、账户、摄像头、电机、继电器或录音功能。不要添加
  这类支持。

## 引脚与总线归属

| 资源 | 引脚 / 实例 | 规则 |
|---|---|---|
| LED | D4，高电平点亮 | 仅开/关；D4 不是 PWM |
| 按键 | D6，按下为 HIGH | 使用 `INPUT`；模块自带下拉 |
| 电位器 | A0 / ADC0 | 10 位 raw，5 V 标称参考 |
| 光线传感器 | A6 / ADC6 | 仅 raw；不是 lux |
| 声音传感器 | A2 / ADC2 | 模拟包络；不是 dB SPL 或音频 |
| 蜂鸣器 | D5，无源 | 使用 `tone()`；不要输出静态直流 |
| I2C | A4 SDA/18，A5 SCL/19 | 所有 I2C 模块共享 5 V 总线 |
| OLED | 默认 I2C 0x3C，可选 0x3D | 考虑 SRAM，使用 U8x8 文本模式 |
| 加速度计 | LIS3DHTR I2C 0x19 | 不要用库默认的 0x18 |
| 气压 | SPA06-003，0x77/0x76 | 必须通过器件 ID 0x11 校验 |
| 温湿度 | DHT20，I2C 0x38 | 初始化前先探测 0x38 |

## SRAM 与 Flash 约束

- 使用 U8x8 文本模式，不要默认采用 U8g2 全屏 framebuffer；除非已记录
  实测 SRAM 余量。
- 一个示例只链接必需的库。AVR 只有 32 KB Flash 和 2 KB SRAM；当前最大
  草图余量充足，但不能随意增加库。
- 避免 `String`、大块全局缓冲和无界串口格式化。

## 构建与上传

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/<sketch>

./tools/validate.sh --arduino grove-beginner-kit
```

未获得当前会话明确授权时，不要运行 `upload`、`flash` 或 avrdude。先通过
`arduino-cli board list` 确认端口。构建成功不是硬件验证。

## 验证契约

每个新增或修改的示例必须：

1. 用 `arduino:avr@1.8.8`、FQBN `arduino:avr:uno` 编译。
2. 写明准确的 9600 波特率串口输出及 OLED 文本。
3. 给出物理步骤：遮住光线传感器、对声音传感器说话、按键、旋转 A0、
   听 D5、倾斜加速度计，或运行 `i2c_scan`。
4. 说明不能推断的内容：raw ADC 不是 lux，声音包络不是 dB SPL，气压高度
   不是 GPS 高度，构建不是上机通过。
5. 保持 `AGENTS.md` 双语对一致。
6. 分别报告静态检查、板级构建和上机结果。

## 已知编译与运行陷阱

- **D6 不要使用 `INPUT_PULLUP`。** 套件按键模块已有下拉，内部上拉会让
  空闲状态看起来是按下。
- **不要对 D4 使用 `analogWrite(D4, ...)`。** D4 不是 PWM；在 AVR 上
  对非 PWM 引脚调用 `analogWrite` 只会变成数字开/关，不能调光。
- **DHT20 库固定 0x38。** 在 `begin()` 前先探测，这样缺传感器时能给出
  明确串口错误，而不是卡住。
- **SPA06 1.0.1 会产生 AVR `Wire::requestFrom` 重载提示。** 这是库警告，
  不是构建失败；不要为了消除提示去修改固定版本的库。
- **OLED 可能在 0x3D。** 只有 0x3D 应答时，`oled_status` 会把 U8x8 切到
  8 位写地址 0x7A。扩展示例时保留该映射。
- **I2C 是共享总线。** OLED、加速度计、气压和 DHT20 都用 A4/A5；没有
  第二个硬件 I2C 外设。
- **不要重新加入 2020 版 DHT11/BMP280 示例。** 它们属于不同的硬件版本，
  本目录有意不提供。

<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# Wio Terminal Agent 指南

本文件适用于 `arduino/boards/wio-terminal/` 及其下所有示例。根目录
`AGENTS.md`、`arduino/AGENTS.md` 与 `docs/development/ai-guide.md`
仍然有效。

## 先读什么

1. `git status --short --branch`；保留无关改动。
2. 本目录 `README.md` 与 `docs/vendor-sources.md` 中本板的条目。
3. 修改草图前，先读该示例自己的 `AGENTS.md`。
4. 对应厂商来源：原理图 v1.2、核心的
   `variants/wio_terminal/variant.h`，或内置 LCD 的 `User_Setup.h`。

绝不要猜引脚、总线、地址、极性或硬件能力。若以上来源均未定义，向用户
提问。

## 固定平台事实

- FQBN：`Seeeduino:samd:seeed_wio_terminal`。
- 核心：`Seeeduino:samd@1.8.6`；Board Manager URL 记录在 `README.md`。
- MCU：ATSAMD51P19A，512 KB Flash、192 KB RAM，标准主频 120 MHz。
- 串口监视器：USB CDC，115200 baud。
- LCD/引脚配置以内置库 `Seeed_Arduino_LCD` 的形式随核心提供；不要额外
  引入上游 `TFT_eSPI` 依赖。
- 外部库只能使用 `libraries.txt` 与板卡 README 已固定版本；保持草图独立，
  依赖越少越好。
- 不要添加触摸屏、电池/充电器、扬声器/音频编解码器或摄像头支持；本板没有
  这些功能。新增外设前先看板卡 README 的“板上未提供”一节。

## 总线与外设归属

| 资源 | 引脚 / 实例 | 规则 |
|---|---|---|
| LCD | SPI3；SCK 68、MOSI 67、CS 69、DC 70、RST 71、BL 72 | 使用核心 `TFT_eSPI`；不要重配这些引脚 |
| microSD | SPI2；SCK 63、MOSI 62、MISO 61、CS 64、检测 65 | 使用 Seeed Arduino FS 并显式传入 `SDCARD_SS_PIN` + `SDCARD_SPI`；绝不能依赖默认 `SS` GPIO 50 |
| IMU | `Wire1`；SDA 78、SCL 77、地址 `0x18`、INT1 79 | 内部总线；不要与 Grove `Wire` 混淆 |
| Grove I2C | `Wire`；SDA 46、SCL 45、左侧 J4 | 仅 3.3 V 设备；IMU 不在该总线上 |
| 蜂鸣器 | GPIO 12，无源 | 必须使用 `tone()`/PWM；不要用直流高电平“让它响” |
| 红外发射 | GPIO 14，经 Q13 高有效 | 仅发送的 940 nm 通路 |
| 按键 | A/B/C 28/29/30；五向 31/32/33/34/35 | 低有效；使用上拉与消抖 |
| RTL8720DN | SPI1/UART；CHIP_PU 80、IRQ0 11 | 独占无线协处理器；一次只跑一个无线示例 |

核心定义了四个 SPI 实例。不要为了塞入新外设而重映射其他示例的总线；若
存在冲突，先记录冲突并提出设计。

LIS3DHTR 地址为 `0x18`，这是 Seeed_Arduino_LIS3DHTR 的默认值。库中的
`0x19` 是备用地址，不是 Wio Terminal 的默认地址；除非在 `0x19` 上探测
成功，否则不要改地址。

## 无线前提

Arduino 上传不会更新 RTL8720DN。Wi-Fi 与 BLE 需要协处理器运行兼容的
eRPC 固件。请按照 Seeed 的
[Wi-Fi 固件更新指南](https://wiki.seeedstudio.com/Wio-Terminal-Network-Overview/)
使用 `ambd_flash_tool`，并通过 `rpc_system_version()` 验证版本，再排查
Wi-Fi/BLE 草图。

`wifi_scan` 与 `ble_scan` 都会初始化并独占协处理器。请分别运行。厂商
eRPC 固件文档说明可同时支持 Wi-Fi 与 BLE，但本仓库没有经过验证的并发
示例。

## BLE 头文件顺序陷阱

Arduino 核心可能定义 `min`、`max` 宏，导致 `rpcBLE` 1.0.0 头文件编译
失败。`ble_scan` 在 `TFT_eSPI.h` 之后、rpcBLE 头文件之前有意执行：

```cpp
#ifdef min
#undef min
#endif
#ifdef max
#undef max
#endif
```

新增使用 rpcBLE 的草图时必须保持同样顺序。不要先包含 rpcBLE 头文件，再
通过重命名库符号来掩盖编译错误。

## 构建与烧录

在仓库根目录执行：

```bash
./tools/validate.sh --static
./tools/validate.sh --arduino wio-terminal

arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/<sketch>
```

没有当前会话的明确授权，不得执行 `upload`、`flash` 或 RTL 固件工具。上传
前先用 `arduino-cli board list` 确认端口；应用模式 USB ID 为 `2886:802d`，
Bootloader 为 `2886:002d`。

## 验证约定

每个新增或修改的示例必须：

1. 使用精确 FQBN 与固定核心编译。
2. 说明预期 LCD 文本/图形以及 115200 baud 串口输出。
3. 给出物理操作：遮盖哪个传感器、按哪个键、连接哪种 Grove/SD/IR 外设，
   或准备怎样的无线环境。
4. 明确不可推断的内容，例如原始 ADC 不是 lux、模拟麦克风电平不是录音、
   IR 必须有接收端才能证明 NEC 帧有效。
5. 保持 `AGENTS.md` 与 `AGENTS.zh_CN.md` 双语内容同步。
6. 将编译、板级构建、上机结果分三项报告。不得把成功构建称为硬件验证。

无线示例上传成功却失败时，先检查 RTL 固件，再检查依赖版本；不要随机改
SPI/UART 引脚。

## 来源优先级与已知冲突

顺序为：厂商规格 / 实测结果，然后板卡引脚表、BSP 实现、厂商指南/示例。
两个已知细节：

- 原理图第 04 页是五向开关、蜂鸣器与麦克风；第 05 页是顶部 A/B/C 按键、
  Grove、光传感器与红外通路。不要用第 05 页引用五向开关。
- 核心宏 `WIO_LIS3DH_WiRE` 拼写错误。请使用 `Wire1` 以及独立的
  `WIO_LIS3DH_SDA` / `WIO_LIS3DH_SCL`。

当来源与实测冲突时，在板卡 README 中同时记录实测结果和两个来源；不要
静默“修正”其中一方。

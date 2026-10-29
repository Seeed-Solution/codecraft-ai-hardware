<p align="right">
  <a href="README.md">English</a> · <strong>简体中文</strong>
</p>

# Wio Terminal

状态：**10 个示例草图已构建验证。8 个示例上机通过；IR 发射已观察；外部
Grove 检测仍待验证。** `examples/` 中的全部草图均使用
`Seeeduino:samd@1.8.6` 编译通过。编译通过不等于外设在真实硬件上可用。

## 已验证事实

- MCU：Microchip ATSAMD51P19A，Arm Cortex-M4F，512 KB 程序 Flash、
  192 KB RAM。Arduino 核使用标准 120 MHz 主频。
  来源：[Wio Terminal 原理图 v1.2](https://files.seeedstudio.com/wiki/Wio-Terminal/res/Wio-Terminal-SCH-v1.2.pdf)
  第 02 页（MCU 与存储器）、
  [产品页](https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/)，
  以及 `Seeeduino:samd@1.8.6` 的 `boards.txt`（`seeed_wio_terminal`）。
- 外部 Flash：W25Q32JVZPIM，32 Mbit / 4 MB，挂在 QSPI 总线上。来源：
  原理图第 02 页。当前目录尚无使用它的示例。
- LCD：2.4 英寸 320x240 ILI9341 面板，连接 SPI3。核心引脚映射为 SCK
  68、MOSI 67、MISO 66、CS 69、DC 70、复位 71、背光 72。核心内置的
  `TFT_eSPI` / `Seeed_Arduino_LCD` 已选择 ILI9341 与这些引脚。示例使用
  `setRotation(3)` 得到宽 320 像素的横向显示。来源：原理图第 07 页、核心
  `variants/wio_terminal/variant.h` 与
  `libraries/Seeed_Arduino_LCD/User_Setup.h`。
- 顶部按键：`WIO_KEY_A`、`WIO_KEY_B`、`WIO_KEY_C` 分别为 GPIO
  28、29、30，低电平有效，带外部上拉。来源：原理图第 05 页与核心
  `variant.h`。
- 五向开关：`WIO_5S_UP`、`WIO_5S_LEFT`、`WIO_5S_RIGHT`、
  `WIO_5S_DOWN`、`WIO_5S_PRESS` 分别为 GPIO 31、32、33、34、35，
  低电平有效。来源：原理图第 04 页与核心 `variant.h`。
- IMU：ST LIS3DHTR 位于内部 `Wire1` 总线，SDA 78、SCL 77，7 位地址
  `0x18`；INT1 为 GPIO 79。来源：原理图第 07 页与核心 `variant.h`
  （`WIO_LIS3DH_SDA`、`WIO_LIS3DH_SCL`、`WIO_LIS3DH_INT`）给出总线与
  引脚；地址来自 Seeed_Arduino_LIS3DHTR 的 `LIS3DHTR_DEFAULT_ADDRESS`
  （`0x19` 是库中的备用 `LIS3DHTR_ADDRESS_UPDATED`）。上游宏
  `WIO_LIS3DH_WiRE` 有拼写错误，请直接使用 `Wire1`。
- 光传感器：光电二极管输出接 `WIO_LIGHT`，GPIO 27 / ADC 通道 15。
  产品页给出 400-1050 nm 响应范围。来源：原理图第 05 页、核心
  `variant.h`，光谱范围来自产品页。
- 麦克风：模拟驻极体麦克风与 LMV358 放大级，调理后的信号接
  `WIO_MIC`，GPIO 39 / ADC 通道 12。来源：原理图第 04 页与核心
  `variant.h`。这是模拟电平通路，不是数字 I2S 麦克风。
- 蜂鸣器：无源蜂鸣器 B4013AM422-042，由 Q4 从 `WIO_BUZZER`（GPIO
  12）驱动。它需要交流/PWM 波形；示例使用核心 `tone()` API。来源：
  原理图第 04 页与核心 `variant.h`。
- microSD：独立 SPI2 总线，SCK 63、MOSI 62、MISO 61、CS 64、卡检测
  65。产品页标注最大支持 16 GB。来源：原理图第 02 页、核心
  `variant.h`（`SDCARD_*`）与产品页。
- 红外发射器：940 nm LED D10 由 Q13 从 `WIO_IR`（GPIO 14）驱动；
  `IR_CTL` 为高有效。它位于外壳背面、microSD 卡槽左侧，只是发射器，
  不含接收器。来源：原理图第 05 页、核心 `variant.h`、产品页与 Seeed
  wiki《Getting Started with Infrared Emitter》。
- Grove 接口：左侧 J4 为 I2C，使用 `Wire`，SDA 46 / SCL 45，供电
  3.3 V。J4 板上没有 I2C 上拉电阻；Grove 模块通常会自带（原理图第 05
  页只有连接器、去耦与 DNP 的 SWD 连线）。右侧 J5 引出 A0/D0 与 A1/D1。
  来源：原理图第 05 页与核心 `variant.h`（`PIN_WIRE_SDA`、
  `PIN_WIRE_SCL`）。本目录只维护 J4 I2C 示例。
- 无线：Realtek RTL8720DN，支持 2.4/5 GHz 802.11 a/b/g/n 与 BLE
  5.0。SAMD51 通过 SPI1/UART 与其通信，并用 CHIP_PU GPIO 80 和
  IRQ0 GPIO 11 控制。来源：原理图第 06 页、核心 `variant.h` 与产品页。
- 扩展：40 针 Raspberry Pi 兼容排针、20 针 FPC、USB Type-C 供电/OTG、
  I2S 与 DAC 引脚。产品具备这些能力，但当前示例未覆盖；不要根据本目录
  推断它们已被验证。

## 板上未提供

厂商功能列表与原理图 v1.2 均不包含触摸控制器、板载电池/充电器、扬声器/音频
编解码器或摄像头。请把 LCD 视为仅显示设备，通过 USB-C 或已记录的电源引脚
供电，并以无源蜂鸣器作为声音输出。不要为不存在的硬件添加库或引脚配置。

## 工具链与依赖

Board Manager URL：

```text
https://files.seeedstudio.com/arduino/package_seeeduino_boards_index.json
```

已验证核心与 FQBN：

- 核心：`Seeeduino:samd@1.8.6`
- FQBN：`Seeeduino:samd:seeed_wio_terminal`

在 shell 中安装核心与固定版本依赖：

```bash
arduino-cli config add board_manager.additional_urls \
  https://files.seeedstudio.com/arduino/package_seeeduino_boards_index.json
arduino-cli core update-index
arduino-cli core install Seeeduino:samd@1.8.6

cd arduino/boards/wio-terminal
while IFS= read -r library; do
  [[ -z "$library" || "$library" == \#* ]] && continue
  arduino-cli lib install "$library"
done < libraries.txt
```

| 依赖 | 版本 | 使用者 |
|---|---|---|
| `Seeeduino:samd` | 1.8.6 | 全部示例；同时以 `Seeed_Arduino_LCD` 内置 `TFT_eSPI` |
| `Seeed Arduino rpcUnified` | 2.1.4 | Wi-Fi 与 BLE 的 RPC 传输层 |
| `Seeed_Arduino_mbedtls` | 3.0.2 | `rpcWiFi` 依赖链 |
| `Seeed Arduino FS` | 2.1.5 | `sd_card_log`，同时属于 `rpcWiFi` 依赖链 |
| `Seeed Arduino SFUD` | 2.0.2 | `rpcWiFi` 依赖链 |
| `Seeed Arduino rpcWiFi` | 1.1.0 | `wifi_scan` |
| `Seeed Arduino rpcBLE` | 1.0.0 | `ble_scan` |

不要为此板另装上游 `TFT_eSPI`：Wio Terminal 的驱动与引脚配置使用
核心 1.8.6 内置副本。

## 能力矩阵

| 能力 | 硬件 | 示例 | 状态 |
|---|---|---|---|
| 显示 | ILI9341，320x240，SPI3 | `display_buttons` | 已实现；上机通过 |
| 输入 | 三个顶部按键 + 五向开关 | `display_buttons` | 已实现；上机通过 |
| 加速度计 | `Wire1` 上的 LIS3DHTR | `accelerometer` | 已实现；上机通过（0x18） |
| 环境光 | `WIO_LIGHT`，ADC 通道 15 | `light_sensor` | 已实现；上机通过；仅原始 ADC |
| 麦克风电平 | `WIO_MIC`，ADC 通道 12 | `microphone_level` | 已实现；上机通过；仅模拟电平 |
| 蜂鸣器 | GPIO 12 上的无源蜂鸣器 | `buzzer_tones` | 已实现；上机通过 |
| microSD | SPI2，最大 16 GB | `sd_card_log` | 已实现；上机通过 |
| 红外发射 | GPIO 14 上的 940 nm NEC 发送 | `ir_nec_sender` | 已实现；已观察发射；NEC 解码待验证 |
| Grove I2C | 左侧 J4，使用 `Wire` | `grove_i2c_scan` | 已实现；空总线上机通过；设备检测待验证 |
| Wi-Fi | RTL8720DN，2.4/5 GHz，仅扫描 | `wifi_scan` | 已实现；扫描已上机通过 |
| BLE | RTL8720DN，BLE 5.0，仅扫描 | `ble_scan` | 已实现；扫描已上机通过（固件 2.1.3） |
| 4 MB QSPI Flash | W25Q32JVZPIM | 无 | 仅记录 |
| 右侧 Grove / 40 针排针 / FPC | GPIO、ADC、I2S、DAC、USB | 无 | 未覆盖 |

## 构建与烧录

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/display_buttons

./tools/validate.sh --arduino wio-terminal
```

通过 USB 连接板卡后，先确认端口再构建/烧录。应用端口 USB ID 为
`2886:802d`，Bootloader 端口为 `2886:002d`。

```bash
arduino-cli board list
arduino-cli upload -p <port> \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/display_buttons
arduino-cli monitor -p <port> -c baudrate=115200
```

烧录必须获得当前会话的明确授权。不要将成功编译或上传描述成硬件验证。

## 已知限制

- **RTL8720 固件独立于 Arduino 草图。** 上传 Arduino 程序只会写入
  ATSAMD51。Wi-Fi 与 BLE 需要 RTL8720DN 上兼容的 eRPC 固件。请按 Seeed
  [Wi-Fi 固件更新指南](https://wiki.seeedstudio.com/Wio-Terminal-Network-Overview/)
  操作，并用 `rpc_system_version()` 确认 RTL 固件版本，再排查草图。
  本仓库不固定具体固件二进制版本。
- `wifi_scan` 已于 2026-10-28 在测试设备现有 RTL8720DN eRPC 固件上完成
  上机验证。串口扫描返回附近多个 2.4 GHz 与 5 GHz SSID，RSSI 约
  -40 至 -46 dBm，用户确认了 LCD 显示。未调用 `rpc_system_version()`，
  因此不声明具体 RTL 固件版本。
- `ble_scan` 已于 2026-10-29 在官方 RTL8720DN eRPC 固件 2.1.3 上上机通过
  （`rpc_system_version()` 返回 `2.1.3`）。每轮 5 秒扫描上报数百个广播，
  列出了附近设备的名称、地址与 RSSI，用户已确认 LCD。该结果依赖两条协
  处理器行为，记录在 `examples/ble_scan/AGENTS.zh_CN.md`：`BLEDevice::init()`
  之后必须调用一次 `ble_start()`，否则 BT 栈不会启动、`BD_ADDR` 全为 0；
  且必须使用非阻塞的 `BLEScan::start` 重载，阻塞重载在无超时的信号量上
  等待，在本机上会冻住 LCD。
- `wifi_scan` 与 `ble_scan` 都会初始化并独占 RTL8720DN。请作为两个独立
  草图运行；尽管厂商 eRPC 固件说明支持 Wi-Fi/BLE 同时工作，本仓库未演示
  并发场景。
- `rpcBLE@1.0.0` 与 Arduino 的 `min`/`max` 宏冲突。`ble_scan` 在包含
  rpcBLE 头文件前取消这两个宏；扩展代码时必须保持该顺序。
- 光传感器示例输出原始 ADC 与估算毫伏值，未校准为 lux；原始值变化方向
  未作断言，必须先在实际设备上观察，再设定阈值。
- 麦克风示例测量模拟交流包络，不是校准后的 dB SPL；若需数字音频，必须
  自行增加采样与存储逻辑。
- `sd_card_log` 使用不超过 16 GB 的 FAT32 卡，只挂载一次，不处理热插拔/
  重新挂载。卡检测 GPIO 65 已记录，但草图未使用。它使用 Seeed Arduino FS
  2.1.5 显式指定 SPI2 与 CS 64；标准 Arduino `SD` 库默认使用 `SS`
  GPIO 50，不能替代。
- `ir_nec_sender` 是阻塞式、软件时序的 NEC 发射器，不含接收与协议解码。
- `grove_i2c_scan` 只扫描 `0x08`-`0x77`，并在每次扫描前向保留地址
  `0x7F` 发一笔哑事务。J4 板上没有 I2C 上拉，Grove 模块通常会自带。
  未接模块时，空闲后的第一笔 `Wire` 事务会把第一个地址误报为 ACK；
  该现象已实测并规避。
- `ir_nec_sender` 目前只用相机观察过发射；尚未用 38 kHz 接收器解码
  NEC 帧。

## 示例

- `display_buttons` — 横向 LCD 测试，以及三个顶部按键与五向开关；20 ms
  消抖并通过串口输出按下/释放事件。
- `accelerometer` — LIS3DHTR `WHO_AM_I` 探测、100 Hz / +/-2 g 配置、
  g 值与 LCD 上的 X/Y 圆点。
- `light_sensor` — 12 位 `WIO_LIGHT` 采样、滚动显示原始值/毫伏值及串口
  输出；不做 lux 换算。
- `microphone_level` — 200 点 `WIO_MIC` 窗口，去除直流后计算峰峰值、
  RMS 与相对电平条。
- `buzzer_tones` — A：440 Hz 持续 250 ms；B：4 kHz 持续 120 ms；C：停止。
- `sd_card_log` — 挂载独立 SPI2 microSD，列出根目录，并在每次挂载时向
  `/wio_terminal.log` 追加一行。
- `ir_nec_sender` — 通过板载 940 nm 发射器发送 NEC 地址 `0x00`，命令分别
  为 `0x45`、`0x10`、`0x02`。
- `grove_i2c_scan` — 扫描左侧 Grove J4 的 `Wire` 总线，最多报告 16 个
  有应答的 7 位地址。
- `wifi_scan` — 扫描本地 2.4/5 GHz 网络，按 RSSI 显示最强 5 个 SSID；
  不使用凭据、不关联网络。
- `ble_scan` — 执行 5 秒主动 BLE 扫描，按 RSSI 显示最强 5 个广播；不连接、
  不配对。

每个示例目录都有独立的中英双语 `AGENTS.md`，说明硬件事实、预期行为与
上机检查步骤。

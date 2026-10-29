<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# ble_scan — Agent 说明

## 目的

使用 RTL8720DN 执行 5 秒主动 BLE 扫描，并按 RSSI 显示最强五个广播。它不会
与任何设备配对或连接。

## 硬件事实

- RTL8720DN 提供 BLE 5.0，通过 SPI1/UART 与 ATSAMD51 通信；CHIP_PU 为
  GPIO 80，IRQ0 为 GPIO 11。来源：原理图 v1.2 第 06 页、核心 1.8.6
  `variants/wio_terminal/variant.h` 与产品页；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 软件路径：`Seeed Arduino rpcBLE@1.0.0` 基于
  `Seeed Arduino rpcUnified@2.1.4`，协处理器为官方 eRPC 固件 2.1.3
  （`rpc_system_version()`）。
- 草图调用 `setActiveScan(true)`：会发送 BLE scan request 获取广播/
  扫描响应；它从不调用 connect 或 pair。
- `rpcBLE@1.0.0` 与 Arduino 的 `min`/`max` 宏不兼容。草图在包含
  `TFT_eSPI.h` 之后、rpcBLE 头文件之前 `#undef` 两者，必须保持该顺序。

## 本示例依赖的两条协处理器行为

两条均在协处理器固件 2.1.3 的 Wio Terminal 上实测，也是本草图与厂商
`BLE_scan` 例程不同的原因。

- `BLEDevice::init()` 不会启动 BT 栈，必须在它之后调用一次 `ble_start()`。
  缺少该调用时协处理器不会拉起 BT 栈：`BLEDevice::getAddress()` 返回
  `00:00:00:00:00:00`，扫描会按时完成但上报 0 个广播。
- `BLEScan::start(duration, continue)` 是阻塞重载，它在
  `m_semaphoreScanEnd` 上无超时等待，在本机上会冻住 LCD。应使用非阻塞的
  `start(duration, scanCompleteCallback, continue)` 重载，在 `loop()` 中用
  `millis()` 轮询。

协处理器固件 2.1.3 在自己的 `ble_start()` 里会自旋等待 BT 栈就绪，因此
扫描进行中不要再调用 `ble_start()`。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/ble_scan
```

安装 `libraries.txt` 中固定版本的无线依赖。授权烧录与设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 第 1 行 `BLE SCAN`；第 2 行 `RTL 2.1.3`；第 3 行 `local <MAC>`。
- 扫描中状态框显示 `scanning <n>s  <count> adv`，计数随广播到达递增。
- 5 秒后显示 `<count> adv / 5 shown`，有设备时为绿色（为 0 时橙色），随后
  最多按 RSSI 列出 5 行 `<名称> <地址> <rssi>`，无名称广播显示
  `(unnamed)`。
- 循环往复：扫描 5 秒、空闲 4 秒。
- 串口以 115200 baud 打印同样的设备行。

## 上机验证

已在协处理器固件 2.1.3 的 Wio Terminal 上验证：

1. 每轮 5 秒扫描上报数百个广播，并列出附近真实设备的名称、地址与负值
   RSSI。
2. 第 3 行显示的本机地址非零，说明 BT 栈已启动。

复验步骤：

1. 用 `rpc_system_version()` 确认协处理器固件；仅支持 Wi-Fi 或版本不匹配
   的镜像会在扫描逻辑之前改变故障表现。
2. 获得明确授权后构建并上传。
3. 在附近放置已知 BLE 广播设备（手机、beacon 或另一块板）。
4. 确认其名称或地址出现，且 RSSI 随距离变化合理。
5. 测试期间只运行本无线示例；不要同时运行 Wi-Fi 草图，也不要根据本仓库
   推断组合行为。

## 限制

- 主动扫描会发送 scan request，不是纯被动接收。
- 只显示最强五个广播；不做 GATT 服务发现、连接、配对或数据交换。
- 名称与地址格式取决于 rpcBLE 1.0.0；不要仅凭显示内容推断隐私行为或地址
  类型。
- 广播计数是一轮扫描中上报的不同地址数量，不代表附近人数或每人设备数。

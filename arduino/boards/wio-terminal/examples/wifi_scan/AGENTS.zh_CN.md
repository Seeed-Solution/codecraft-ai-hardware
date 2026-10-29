<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# wifi_scan — Agent 说明

## 目的

使用 RTL8720DN 协处理器扫描附近 Wi-Fi，并按 RSSI 显示最强五个 SSID。草图不
使用凭据、不关联接入点，也不访问云服务。

## 硬件事实

- 无线协处理器：Realtek RTL8720DN，通过 SPI1/UART 与 ATSAMD51 通信。
  CHIP_PU 为 GPIO 80，IRQ0 为 GPIO 11。来源：原理图 v1.2 第 06 页与核心
  1.8.6 `variants/wio_terminal/variant.h`；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 产品支持 2.4 GHz 与 5 GHz 的 802.11 a/b/g/n。本示例只显示 SSID 与 RSSI，
  不标注、也不验证每个结果所属频段。
- 软件路径：`Seeed Arduino rpcWiFi@1.1.0` 基于
  `Seeed Arduino rpcUnified@2.1.4`，完整固定依赖见 `libraries.txt`
  （`mbedtls` 3.0.2、`FS` 2.1.5、`SFUD` 2.0.2）。
- Arduino 烧录只写 ATSAMD51；RTL8720DN 必须运行兼容 eRPC 固件，详见板卡
  README 与厂商固件指南。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/wifi_scan
```

构建前安装精确版本的无线库；不要假设 Arduino IDE 中“最新版”一定兼容。
授权烧录与设置见板卡 [AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`WI-FI SCAN`；状态先显示 `starting radio...`，`WiFi.mode(WIFI_STA)`
  成功后显示 `ready`。
- 启动时扫描一次，之后每 10 秒扫描一次。状态显示
  `<count> found / top <n>`，LCD 按 RSSI 显示最多五个条目。
- 串口为每个保留网络打印 `<排名> <SSID> <RSSI>`；空 SSID 显示为
  `(hidden)`。
- 初始化/扫描失败时串口出现 `WiFi.mode(WIFI_STA) failed` 或
  `WiFi.scanNetworks() failed`。无线网卡不会关联任何网络。

## 上机验证

1. 按厂商 Wi-Fi 固件指南安装/确认 RTL8720DN 运行兼容 eRPC 固件；不能用
   上传本草图替代此步骤。
2. 获得明确授权后构建并上传，以 115200 baud 打开串口。
3. 确认状态进入 `ready`，扫描结果包含现场真实存在的网络。
4. 将最强 SSID/RSSI 与已知本地接入点比对；记录实测结果，不要假定已覆盖
   5 GHz。
5. 测试期间只运行本无线示例；不要同时加载 BLE 草图，也不要根据本仓库推断
   并发 Wi-Fi/BLE 行为。

2026-10-28 实测：测试设备现有 RTL8720DN eRPC 固件可正常响应本草图；串口
扫描返回附近多个 2.4 GHz 与 5 GHz SSID，RSSI 约 -40 至 -46 dBm，用户
确认了 LCD 显示。未调用 `rpc_system_version()`，因此不声明固件版本。

## 限制

- 仅扫描：不输入凭据、不关联、不测试 IP 协议栈、不建立连接或云端通路。
- 只显示最强五个网络；屏幕不显示 BSSID/信道/安全类型，也不区分 2.4 GHz
  与 5 GHz。
- RTL 固件缺失或不匹配会造成编译检查无法发现的无线故障。
- 本次上机使用的 RTL 固件版本未记录；安装与版本确认仍是复现步骤的一部分。

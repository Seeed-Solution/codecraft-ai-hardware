<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# grove_i2c_scan — Agent 说明

## 目的

扫描左侧 Grove I2C 接口并报告有应答的 7 位地址。它是外接 Grove I2C 模块的
第一项测试，也用于确认外部 `Wire` 总线与板载 IMU 总线相互独立。

## 硬件事实

- 左侧 Grove J4 是外部 I2C 总线，核心以 `Wire` 暴露：SDA GPIO 46、SCL
  GPIO 45。来源：原理图 v1.2 第 05 页与核心 1.8.6
  `variants/wio_terminal/variant.h`（`PIN_WIRE_SDA`、`PIN_WIRE_SCL`）；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 接口供电为 3.3 V，只能连接 3.3 V Grove I2C 设备。
- J4 板上没有 I2C 上拉电阻：原理图第 05 页只有连接器、1 uF 去耦电容
  与 DNP 的 SWD 连线。Grove I2C 模块通常会自带上拉。草图在
  `Wire.begin()` 前启用 SAMD51 内部上拉；短距离台面测试可用，但更稳妥的
  做法是外接一对 4.7 kOhm 上拉电阻。
- LIS3DHTR 位于 `Wire1`（SDA 78 / SCL 77，地址 `0x18`），不应出现在本扫描
  中。右侧 Grove J5 不是这条 I2C 总线。
- 草图将 `Wire` 设为 100 kHz，每三秒扫描 7 位用户地址范围 `0x08` 到
  `0x77`，最多保留 16 个有应答地址。保留地址 `0x00`-`0x07` 与
  `0x78`-`0x7F` 会被跳过；扫描这些地址可能在空总线上误报设备。
- 2026-10-28 实测：J4 空总线时，空闲后的第一笔 `Wire` 事务会把第一个
  扫描地址误报为 ACK。草图在每次扫描前先向保留地址 `0x7F` 发一笔哑事务，
  之后空总线会报告 0 个设备。仅启用 SAMD51 内部上拉不能消除该现象。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/grove_i2c_scan
```

不需要外部库。授权烧录与设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`GROVE I2C SCAN`；显示 `Wire: SDA 46 / SCL 45 / left Grove J4`
  与 `3.3 V devices only`。
- 未接设备时先完成 `0x7F` 哑事务，随后串口无地址输出，屏幕显示
  `no ACK on Wire` 并提示检查线缆/3V3。
- 接入设备后，LCD 显示例如 `0x48`，串口为每个应答设备打印
  `I2C device at 0x48`。
- 板载 IMU 地址 `0x18` 不应出现；若出现，说明 J4 外部设备也使用了该地址，
  并不是 IMU。

## 上机验证

1. 获得明确授权后构建并上传，以 115200 baud 打开串口。
2. 断电后在左侧 J4 接入一个已知的 3.3 V Grove I2C 设备。
3. 上电，确认三秒内 LCD 与串口出现该设备文档中的地址。
4. 拔掉设备，确认下一次扫描显示 `no ACK on Wire`。
5. 记录地址与外部模块型号；不要直接套用其他 Grove 产品的地址。

2026-10-28 已实测：加入 `0x7F` 哑事务修正后，J4 空总线报告 0 个设备，
串口无地址输出。临时探针在 `Wire1` 上找到板载 IMU `0x18`，证明扫描逻辑
正常。本次没有可用的外部 Grove I2C 设备，因此未做设备检测，仍待验证。

## 限制

- 只做 ACK 探测；不识别设备、不读寄存器，也不验证时钟拉伸。
- 最多保留/显示 16 个地址；找到 16 个应答后会提前结束扫描。
- 右侧 J5 不在此扫描范围内。
- 首笔事务假 ACK 的规避是经验性修正；长线或噪声环境仍建议外接
  4.7 kOhm 上拉电阻。

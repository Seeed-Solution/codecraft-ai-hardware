<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# light_sensor — Agent 说明

## 目的

通过 SAMD51 ADC 读取板载光电二极管，显示滤波后的原始值、估算毫伏值和
电平条。它是相对光照演示，不是校准过的 lux 计。

## 硬件事实

- 信号：`WIO_LIGHT`，GPIO 27 / ADC 通道 15，来自原理图 v1.2 第 05 页的
  光电二极管前端。来源：原理图与核心 1.8.6 `variant.h`；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 产品页给出传感器 400-1050 nm 响应范围。
- 草图设置 12 位 ADC，并使用标称 3.3 V 参考。毫伏显示由 ADC 计数换算，
  只是估算值。
- 每 100 ms 采样一次，并应用 3:1 滚动滤波。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/light_sensor
```

不需要外部库。授权烧录与设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`LIGHT SENSOR`；显示引脚、原始值、滤波后毫伏值与绿色电平条。
- 串口输出类似 `light_raw=1234 filtered=1210.5`。
- 遮盖与松开传感器窗口会改变 ADC 值。草图有意不断言原始计数升高还是降低。

## 上机验证

1. 获得明确授权后构建并上传，以 115200 baud 打开串口。
2. 记录正常室内光下的原始值。
3. 完全遮盖传感器窗口，等待稳定后记录新值。
4. 松开遮挡，确认数值按相反方向恢复。
5. 根据实测变化方向与范围设置应用阈值；不要直接复用其他设备的值。

2026-10-28 已实测：室内光下原始计数约 14-20，用户已确认 LCD/串口示例。
本次未记录遮盖/松开时的变化方向；设定阈值前必须自行测量。

## 限制

- 无 lux 校准、光谱校正或暗电流校准。
- 模拟参考按标称 3.3 V 处理；板级容差会影响毫伏估算。
- 原始基线随环境光变化；示例不提供通用阈值。

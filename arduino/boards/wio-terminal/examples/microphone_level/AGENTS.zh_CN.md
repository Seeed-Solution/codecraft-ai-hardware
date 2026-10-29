<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# microphone_level — Agent 说明

## 目的

将板载模拟麦克风用作相对声音电平表。草图去除 ADC 直流偏置后，报告峰峰值
计数、RMS 计数与变色电平条。

## 硬件事实

- 板载驻极体麦克风经 LMV358 放大，输出网络为 `MIC_OUTPUT`，连接到
  `WIO_MIC`（GPIO 39 / ADC 通道 12）。来源：原理图 v1.2 第 04 页与核心
  1.8.6 `variants/wio_terminal/variant.h`；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 每 100 ms 读取 200 个样本，样本间隔 40 微秒（约 8 ms 窗口）。
- ADC 为 12 位，参考电压按标称 3.3 V 处理。显示的毫伏值是估算，不是校准
  后的声学测量。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/microphone_level
```

不需要外部库。授权烧录与设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`MICROPHONE LEVEL`；显示峰峰值、RMS、`relative level` 与
  绿/黄/红色电平条。
- 串口每秒约 10 次打印 `mic_mean=... pp=... rms=...`。
- 在麦克风附近说话、吹口哨或轻敲，电平条与峰峰值/RMS 会升高；安静时，
  去除直流后的数值应接近零。

## 上机验证

1. 获得明确授权后构建并上传，以 115200 baud 打开串口。
2. 在安静环境记录 mean、峰峰值与 RMS 的短时基线。
3. 在正常距离说话或吹口哨，确认三个数值随声音升降。
4. 停止发声，确认数值回到基线附近。
5. 若信号一直为 0 或 4095，先检查麦克风通路与模拟参考，不要直接改算法。

2026-10-28 已实测：安静环境峰峰值约 162-666 counts、RMS 约
37-129 counts；说话或吹口哨时峰峰值达到 3458 counts、RMS 约
1578 counts；停止发声后两者均回落到安静基线。用户同时确认 LCD 电平条
随声音变化。

## 限制

- 仅单声道模拟包络；不是数字 I2S 麦克风，若需录音必须另做采样与存储设计。
- 无 dB SPL 校准、A 计权或频率响应校正。
- 8 ms 采样窗口不是连续采集，可能漏掉短瞬态。
- 电平是相对的 ADC 测量值；实测基线会随环境噪声与麦克风直流偏置变化。

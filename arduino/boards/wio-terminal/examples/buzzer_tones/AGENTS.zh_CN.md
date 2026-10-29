<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# buzzer_tones — Agent 说明

## 目的

通过三个顶部按键驱动板载无源蜂鸣器播放有限时长音调，演示正确的交流/PWM
驱动与停止行为。

## 硬件事实

- `WIO_BUZZER` 为 GPIO 12 / TCC0 通道 4，驱动 Q4 与无源蜂鸣器
  B4013AM422-042。来源：原理图 v1.2 第 04 页、核心 1.8.6
  `variants/wio_terminal/variant.h` 与核心引脚表；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 蜂鸣器是无源的：直流电平不会产生持续音调。草图使用 SAMD 核心 `tone()`
  定时器 API。
- 顶部按键 A、B、C 分别为 GPIO 28、29、30，低有效并带上拉。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/buzzer_tones
```

不需要外部库。授权烧录与设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`PASSIVE BUZZER`；提示：`A: 440 Hz   B: 4 kHz   C: stop`。
- A 播放 440 Hz 持续 250 ms，B 播放 4 kHz 持续 120 ms；C 调用 `noTone()`，
  随后将 GPIO 12 拉低。
- 串口分别打印 `tone 440 Hz for 250 ms`、`tone 4000 Hz for 120 ms` 或
  `buzzer stopped`。
- “last action”区域随命令改变颜色。

## 上机验证

1. 获得明确授权后构建并上传，以 115200 baud 打开串口。
2. 按 A，确认能听到短促 440 Hz 音调并出现对应串口输出。
3. 按 B，确认听到更高、更短的提示音。
4. 按 A 或 B 后再按 C，确认声音停止且 GPIO 12 为低。
5. 确认按钮操作后没有任何持续不断的音调。

2026-10-28 已实测：多次按下 A/B/C 时，串口分别输出
`tone 440 Hz for 250 ms`、`tone 4000 Hz for 120 ms` 与
`buzzer stopped`。用户确认两个音调均可听见，LCD 操作状态随按键更新，
且有限时长结束或按 C 后没有持续音。

## 限制

- 只有三个固定音调，没有旋律、音量控制或音频编解码器。
- `tone()` 占用硬件定时器资源；新增 PWM 代码前应检查定时器冲突。
- B 音调较高且很短，手机麦克风或人耳判断可能不可靠，应以实际板卡为准。
- 未测量实际音量或频率；是否可听由用户现场确认。

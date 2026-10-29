<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# sound_sensor — Agent 说明

## 目的

观察模拟声音包络，并在短时声音峰值时点亮 LED。

## 硬件事实

- Grove Sound 接 A2（ADC2）。来源：套件 wiki 默认引脚表和 Lesson 6。
- 厂商示例用 raw > 400 点亮 LED。来源：套件 wiki Lesson 6。
- 输出是模拟声音包络，不是数字麦克风流。来源：套件 wiki Lesson 6 的
  模拟信号与 LED 练习。
- Grove LED 接 D4，高电平点亮。来源：套件 wiki。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/sound_sensor
```

## 预期行为

草图从 A2 取 50 ms 峰值窗口，每窗口以 9600 波特率打印一行：

```text
A2 peak=180 LED=OFF
```

峰值超过 raw 400 时，LED 至少再保持点亮 100 ms。可在串口绘图器中观察说
话或拍手时的包络变化。

## 物理测试

1. 在安静房间打开 9600 波特率串口或绘图器。
2. 在声音传感器附近拍手或说话，确认峰值上升并点亮 LED。
3. 等待环境安静，确认保持时间结束后 LED 熄灭。

## 2026-10-29 实测

Seeeduino Lotus v1.1，安静室内：50 ms 峰值稳定在 100-170，`LED=OFF`。
在距麦克风约 20 cm 处拍两次手，分别得到 `peak=561` 与 `peak=686`，随后
`LED=ON`；后续采样在 `peak=162`、`282` 时仍保持 `LED=ON`，再回到
`LED=OFF`。来源：实测结果。

因此 400 阈值相对室内噪声约有 2 倍余量，拍手包络也远高于它。隔着房间
拍手不一定能触发，请把手靠近麦克风小圆筒。

## 限制

- 这不是标定的 dB SPL，也不能录音。
- raw 400 是厂商示例阈值，不是通用房间阈值；先记录安静/峰值，再调整。
- 模拟输出可能饱和；不能用峰值推断完整波形。

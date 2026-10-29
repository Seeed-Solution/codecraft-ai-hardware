<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# light_sensor — Agent 说明

## 目的

读取套件光线传感器，并用暗处阈值切换套件 LED。

## 硬件事实

- Grove Light 接 A6（ADC6）。来源：套件 wiki 默认引脚表和 Lesson 5。
- 厂商示例在 raw < 200 时点亮 LED，否则熄灭。来源：套件 wiki Lesson 5。
- Grove LED 接 D4，高电平点亮。D4 不是 PWM。来源：套件 wiki。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/light_sensor
```

## 预期行为

每 250 ms 以 9600 波特率打印：

```text
A6 raw=123 mV=601 LED=ON
```

raw 低于 200 时点亮，高于 260 时熄灭；两个阈值形成滞回。遮住传感器应让
光线变暗；请在实际板卡上观察 raw 方向，不要把 raw 直接换算成 lux。

## 物理测试

1. 在室内光线下打开 9600 波特率串口，记录 raw。
2. 用手遮住传感器，确认 raw 向暗阈值移动且 LED 点亮。
3. 移开手，确认 raw 高于上限后 LED 熄灭。

## 2026-10-29 实测

Seeeduino Lotus v1.1，室内自然光：A6 读数 raw 776-781（3.8 V），
`LED=OFF`。用手完全遮住光敏电阻后 A6 降到 0-26，LED 点亮；短暂半遮挡时
raw 76-118，同样点亮 LED。移开手后恢复到 raw 779、`LED=OFF`。来源：实测
结果。

复现时有两个坑：

- 遮错模块时 A6 会一直停在约 777。光敏电阻是带锯齿花纹的小圆片。
- A6 和 A7 都会响应同一个光敏电阻，但 A7 只降到约 raw 80；请使用文档
  指定的 A6。

## 限制

- raw ADC 和估算毫伏不是 lux。
- 200/260 阈值是应用选择；应根据实际外壳和光照测量后调整。
- D4 不具备 PWM，本示例不测试调光。

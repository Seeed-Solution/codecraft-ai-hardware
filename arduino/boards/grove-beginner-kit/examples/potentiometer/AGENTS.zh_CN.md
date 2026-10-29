<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# potentiometer — Agent 说明

## 目的

读取套件旋转电位器，并演示不使用 PWM 的阈值数字动作。

## 硬件事实

- Grove Rotary Potentiometer 接 A0。来源：套件 wiki 默认引脚表和 Lesson 3。
- 套件中的 ATmega328P ADC 以 5 V 参考返回 0-1023。来源：套件 wiki
  Lesson 3 和核心 `standard` variant。
- Grove LED 接 D4，且 D4 不是 PWM 引脚。来源：套件 wiki Lesson 4。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/potentiometer
```

## 预期行为

9600 波特率下每 200 ms 打印一行，例如：

```text
A0 raw=512 mV=2502 pct=50 LED=ON
```

raw 高于 532 时点亮 LED，低于 492 时熄灭。两个阈值在中点附近形成滞回。
端点不一定正好达到 0 或 1023。

## 物理测试

1. 打开 9600 波特率串口。
2. 将电位器从一端转到另一端，记录 raw 范围。
3. 确认 LED 在中点附近切换。未实测前不要假设顺时针一定增大。

## 限制

- 本示例不是 PWM 调光；PWM 必须把 LED 模块改接到 D3。
- 毫伏值按标称 5000 mV 参考估算。
- 中点阈值是应用选择，不是电气极限。

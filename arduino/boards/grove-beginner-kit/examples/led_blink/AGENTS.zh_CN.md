<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# led_blink — Agent 说明

## 目的

套件 Grove LED 的最小数字输出测试。

## 硬件事实

- Grove LED 接 D4，高电平点亮。来源：套件 wiki 默认引脚表和 Lesson 1。
- ATmega328P 核心上 D4 不是 PWM 引脚，因此本示例只做闪烁。来源：套件
  wiki Lesson 4 的 PWM 引脚列表与核心 `standard` variant。
- LED 模块带可调亮度电阻。来源：套件 wiki Lesson 1。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/led_blink
```

## 预期行为

- LED 亮 500 ms、灭 500 ms，形成 1 Hz 闪烁。
- 9600 波特率串口先打印 `led_blink: Grove LED on D4`，之后每个相位打印
  一次 `LED=ON` 或 `LED=OFF`。

## 物理测试

1. 给套件供电并打开 9600 波特率串口。
2. 确认稳定的 1 Hz 闪烁。
3. 若能看到但偏暗，调整 LED 模块上的微调电阻；不要用软件改变 GPIO 驱动。

## 限制

- 本示例只证明 D4 输出和 LED 模块通路，不证明 PWM 调光。
- 邮票孔断线或 LED 损坏都表现成“软件故障”；改代码前先检查模块和 D4
  连通性。

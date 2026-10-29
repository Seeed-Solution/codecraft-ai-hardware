<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# buzzer_tones — Agent 说明

## 目的

用可复现的音调序列验证套件无源压电蜂鸣器。

## 硬件事实

- Grove Buzzer 接 D5。来源：套件 wiki 默认引脚表和 Lesson 4。
- 套件蜂鸣器是无源器件，需要交流/方波；静态 HIGH 不会持续发声。来源：
  套件 wiki Lesson 4。
- 草图在 D5 上使用 Arduino `tone()` API。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/buzzer_tones
```

## 预期行为

- 依次播放四个音调：440 Hz/250 ms、880 Hz/250 ms、1760 Hz/250 ms、
  2000 Hz/400 ms，音调之间间隔 80 ms。
- 随后静音 1.5 秒，循环。
- 9600 波特率串口打印每个频率及 `silence 1500 ms`。

## 物理测试

1. 打开 9600 波特率串口并听声音。
2. 确认四个可区分的上行音调和静音间隔。
3. 若无声，检查 D5 和无源蜂鸣器模块；不要把 `tone()` 改成静态
   `digitalWrite(HIGH)`。

## 限制

- `tone()` 固定波形和电压；草图不控制主观音量。
- 若与其他依赖 Timer2 的 PWM 组合，必须先核对 AVR 定时器分配。
- 构建成功不代表蜂鸣器或其三极管通路正常。

<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# button_led — Agent 说明

## 目的

验证套件数字按键输入，并将状态镜像到套件 LED。

## 硬件事实

- Grove Button 接 D6。厂商示例使用 `pinMode(D6, INPUT)`，按下为 `HIGH`，
  说明模块自带下拉。来源：套件 wiki 默认引脚表和 Lesson 2。
- 不要在该输入上启用 ATmega328P 内部上拉，否则空闲按键会被读成按下。
- Grove LED 接 D4，高电平点亮。来源：套件 wiki Lesson 2。

## 构建

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/button_led
```

## 预期行为

- 空闲：LED 灭，启动时打印 `Button=RELEASED LED=OFF`。
- 按住：LED 亮，20 ms 消抖后打印一次 `Button=PRESSED LED=ON`。松开后只
  再打印一次反向状态。
- 串口波特率为 9600。

## 物理测试

1. 打开 9600 波特率串口，确认初始为 released。
2. 按住按键，确认 LED 立即跟随，且 pressed 行只出现一次。
3. 松开按键，确认 LED 熄灭，released 行只出现一次。

## 限制

- 草图只实现 20 ms 消抖，不检测长按、双击或释放时长。
- 若按键永远不释放，先检查模块是否卡住，并确认代码没有误加
  `INPUT_PULLUP`，再考虑更换硬件。

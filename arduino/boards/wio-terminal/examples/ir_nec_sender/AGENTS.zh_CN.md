<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# ir_nec_sender — Agent 说明

## 目的

不依赖外部红外库，通过板载 940 nm 红外发射器发送三组 NEC 帧。示例展示高有效
驱动、38 kHz 载波与 NEC 地址/命令编码。

## 硬件事实

- `WIO_IR` 为 GPIO 14，经 Q13 高有效驱动板载 940 nm LED D10。来源：原理图
  v1.2 第 05 页与核心 1.8.6 `variants/wio_terminal/variant.h`；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 产品页标明发射波长为 940 nm。
- 发射器位于外壳背面、microSD 卡槽左侧。来源：Seeed wiki
  《Getting Started with Infrared Emitter》。
- 草图每 13 微秒翻转一次 GPIO 14，用软件生成 38 kHz 载波；发送地址
  `0x00`、反码地址、命令、反码命令，最后再发一个 mark。
- 按键：A=GPIO 28、B=29、C=30，低有效。A 发送命令 `0x45`，B 为 `0x10`，
  C 为 `0x02`。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/ir_nec_sender
```

不需要外部库。授权烧录与设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`IR NEC SENDER`；显示 `WIO_IR = GPIO 14 / 940 nm`、NEC 时序摘要
  与命令表。
- 按 A/B/C 后，last frame 分别显示 `A: cmd 0x45`、`B: cmd 0x10`、
  `C: cmd 0x02`，串口打印 `sending NEC addr=0x00 cmd=45`（或其他对应
  大写十六进制值）。
- 没有发送帧时，GPIO 14 保持低电平。

## 上机验证

1. 获得明确授权后构建并上传，以 115200 baud 打开串口。
2. 将板卡**背面**（microSD 卡槽左侧的发射器）正对已知的 38 kHz 解调
   接收器。手机摄像头可能只看到微弱闪烁，不能证明 NEC 帧有效；iPhone
   后摄通常会滤除 940 nm。可先用电视遥控器做相机对照。
3. 依次按 A、B、C，确认接收端解出的地址均为 `0x00`，命令分别为 `0x45`、
   `0x10`、`0x02`。
4. 确认未按键时接收端不会解出帧。
5. 若无法解码，先检查视线、接收器供电与 GPIO 14 驱动，再改 NEC 时序。

2026-10-28 已实测：将 iPhone 相机对准背面 microSD 卡槽左侧的发射器后，
用户观察到了 940 nm 发射。这只证明 LED 有动作，不能证明 NEC 帧有效；
用 38 kHz 接收器解码仍待验证。

## 限制

- 仅发送；草图没有红外接收器或 NEC 解码器。
- 帧由软件 bit-bang，会阻塞 CPU 数十毫秒；具体时长取决于命令中 0/1 位的
  组合。若需严格的实时任务，必须先重构时序。
- 13 微秒半周期是软件近似值；接收器容差与 CPU 负载会影响实际载波。
- 尚未做接收端解码；当前上机证据仅为相机观察到的发射。

---
name: codecraft-device-test
description: 为 codecraft-ai-hardware 板卡烧录固件并在真机上验证行为。用于烧录与真机测试；烧录需要用户逐会话明确授权。
---

<p align="right"><strong>简体中文</strong> · <a href="SKILL.md">English</a></p>

# 真机测试

1. 烧录需要每个会话的明确用户授权；未获授权时先询问。
2. 烧录前确认端口与板卡身份（例如 reSpeaker Lite 的控制台是小尺寸
   XIAO 板上的 USB，VID:PID 为 `303a:1001`）。
3. 不要把读回整片 flash 作为前置条件；除非用户明确要求，绝不整片擦除。
4. 烧录后验证本次改动声称的行为：观察日志与物理功能（屏幕、LED、按键、
   音频），并准确报告实际观察到的现象。
5. 注意板级前置条件（例如 reSpeaker Lite 的 XMOS 必须运行 Seeed 的
   16 kHz I2S 固件 v1.0.9——ESP32 烧录不会更新 XMOS）。

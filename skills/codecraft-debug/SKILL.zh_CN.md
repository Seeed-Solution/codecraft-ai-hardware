---
name: codecraft-debug
description: 诊断 codecraft-ai-hardware 的构建、烧录与真机故障。用于构建失败、烧录失败或硬件行为与代码不符时。
---

<p align="right"><strong>简体中文</strong> · <a href="SKILL.md">English</a></p>

# 调试

1. 用最小命令复现；捕获完整错误文本。
2. 分类：环境（IDF 版本错误）、配置（sdkconfig、分区表）、代码、
   接线/硬件，或硬件事实错误。
3. 怀疑硬件事实错误时，按根 `AGENTS.md` 的事实优先级链处理——厂商规格
   与实测结果优先于 README 与示例代码。任何更正要记录到板卡 README。
4. 一次只改一个变量；每改一次重跑失败的检查。
5. 未按 `codecraft-device-test` 要求获得授权，不得把实验性修复烧录到设备。

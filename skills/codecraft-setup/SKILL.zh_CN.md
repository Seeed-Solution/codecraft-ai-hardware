---
name: codecraft-setup
description: 为 codecraft-ai-hardware 准备工具链——按各板要求安装对应版本的 ESP-IDF、主机测试依赖与 Linux 板卡前置条件。用于环境初始化或工具链缺失时。
---

<p align="right"><strong>简体中文</strong> · <a href="SKILL.md">English</a></p>

# 环境准备

1. 确认目标平台与板卡；读其 README 获取所需 ESP-IDF 版本（各板版本不同
   ——reSpeaker Lite 要求 v6.0.1）。
2. ESP-IDF 缺失时，按板卡声明的版本与 Espressif 官方安装流程为主机系统
   安装。需要网络/系统权限时先取得授权，不绕过权限。
3. 验证：`. $IDF_PATH/export.sh`（或平台等效方式）后 `idf.py --version`
   必须报告板卡要求的版本。
4. 主机测试需要 C 编译器与 Python 3。
5. Linux 板卡需安装 libgpiod 及 Python 绑定；记录所需 udev 规则或用户组。
6. 准确报告已安装内容与仍缺失项。

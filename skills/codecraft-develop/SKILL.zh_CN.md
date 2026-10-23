---
name: codecraft-develop
description: codecraft-ai-hardware 仓库的开发工作流——任务路由、硬件事实优先级、BSP/应用边界与交付格式。用于任何代码或文档改动；不构建、不烧录、不提交、不发布。
---

<p align="right"><strong>简体中文</strong> · <a href="SKILL.md">English</a></p>

# 开发

1. 先执行 `git status --short --branch`；保留已有用户改动。
2. 阅读根 `AGENTS.md` 路由表中对应任务的条目，以及
   `docs/development/ai-guide.md`。
3. 涉及板卡工作时，编辑前先读该板 README 及其在
   `docs/vendor-sources.md` 中的行。克隆并检索厂商源码；绝不猜测硬件事实。
4. 把代码放到正确层级：板级事实与可复用驱动放板卡 BSP，共享芯片驱动放
   `esp32/components/`，演示与应用逻辑放 `examples/` 或应用目录。
5. 让可测逻辑独立于 ESP-IDF/LVGL，并在 `tests/` 添加主机测试。
6. 按 `docs/development/ai-guide.md` 的报告格式交付：静态检查、逐板构建
   结果、真机验证情况（或明确说明未做真机验证）。

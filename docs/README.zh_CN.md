<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# 文档

仅 README 保留中文版。工程指南、规则、Skills 和报告统一维护英文。

## 当前规则与实现

- [通用编程 Agent 接入](development/agent-client-setup.md) — Codex、Claude Code、Cursor、ZCode、WorkBuddy、Muse 等工具共用的入口。
- [仓库与贡献约束](../AGENTS.md)
- [架构](architecture/overview.md)
- [CodeCraft 硬件应用平台架构](architecture/codecraft-hardware-app-platform.md)
- [独立 Skill 契约与发布归属](architecture/skill-contracts.md)
- [仿真范围决策](architecture/simulation-scope.md)
- [私有项目工作区与项目说明](development/project-workspaces.md)
- [仓库维护入口](../CONTRIBUTION.md)
- [外部 SenseCraft CLI 安装与路由](development/sensecraft-cli.md)
- [本地 CLI 职责边界与 Skill 契约收拢](development/cli-boundaries-review.md) — 当前命令归属与已完成的实施验收。
- [设备文档格式与统一引用](development/device-documents.md)
- [AI 开发指南](development/ai-guide.md)
- [独立 Skill 打包与版本工作流](development/skill-releases.md)
- [主机测试归属与定向执行](development/testing.md)
- [CI 影响范围选择与汇总结果](development/ci.md)
- [ESP-IDF BSP 组合与布局](development/native-compositions.md)
- [硬件变体与原生构建选择](development/hardware-variants.md)
- [固件镜像交付与 TODO](development/firmware-images.md)
- [生成的 Skill 索引](skill-inventory.json)
- [可移植应用工作流](development/application-workflow.md)
- [生成的原生接口清单](inventory.md)与 [JSON 绑定索引](interface-matrix.json)
- [生成的厂商索引](vendor-sources.md)
- [设备验收](on-device-validation.md)

## 规划与待办

- [Agent 客户端兼容设计](development/agent-client-compatibility-design.md) — 提议以最小 Skill/文档入口改进兼容性，不改变架构或 schema。
- [本地硬件架构规划（由飞书文档转换并对齐）](architecture/hardware-architecture-plan.md)
- [仓库工具迁移 TODO](development/tools-rust-migration-todo.md)
- [架构对齐 TODO](development/architecture-alignment-todo.md)

- [V0.2 实施 TODO](development/v0.2-todo.md) — 设计对齐、已完成工作与待验收事项。
- [reCamera SDK 优化 TODO](development/recamera-sdk-todo.md) — Skill/工具职责、已完成检查与后续 Device/Target 接入；参考文档和候选包命令见 [SDK 维护流程](development/recamera-sdk-maintenance.md)。

- [架构要求](development/codecraft-hardware-distilled.md) — 定义三个内容所有者的范围与验收要求。
- [优化方案与 TODO](development/optimization-plan.md) — 尚未完成的工具、外部验收与治理事项。

从[设备目录](../devices/AGENTS.md)进入各设备。[设备文档指南](development/device-documents.md)统一定义唯一维护来源、Markdown 字段、公共引用与导出检查；原生目录布局见[架构](architecture/overview.md)。

- [Rust CLI 安装、更新与 Python 边界](development/rust-cli.md)

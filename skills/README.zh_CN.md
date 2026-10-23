<p align="right">
  <strong>简体中文</strong> · <a href="README.md">English</a>
</p>

# CodeCraft Agent 技能

五个技能约束本仓库的 AI 辅助开发。每个技能独立成目录，内含带 YAML
frontmatter（`name`、`description`）的 `SKILL.md`，并配对本文件对应的
`SKILL.zh_CN.md`。

| 技能 | 覆盖 | 不覆盖 |
|---|---|---|
| [codecraft-develop](codecraft-develop/SKILL.zh_CN.md) | 工作流：路由、硬件事实优先级、BSP/应用边界、交付格式 | 构建、烧录 |
| [codecraft-setup](codecraft-setup/SKILL.zh_CN.md) | 工具链准备（各板所需 ESP-IDF 版本、Linux 依赖） | 编码 |
| [codecraft-build](codecraft-build/SKILL.zh_CN.md) | 逐板构建与验证固件/示例 | 烧录、发布 |
| [codecraft-device-test](codecraft-device-test/SKILL.zh_CN.md) | 烧录与真机验证 | 构建 |
| [codecraft-debug](codecraft-debug/SKILL.zh_CN.md) | 诊断构建、烧录与真机故障 | 功能开发 |

开发前把全部五个技能装入你的 agent 环境（见根 `AGENTS.md`）。只使用与
当前任务匹配的技能。按 [技能验证](validation.zh_CN.md) 检查路由与边界
行为。

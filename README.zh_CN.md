<p align="right">
  <a href="README.md">English</a> · <strong>简体中文</strong>
</p>

# CodeCraft AI Hardware

面向 AI agent 驱动开发的 Seeed 设备硬件能力参考仓库。

这是一个**纯硬件**仓库：板级支持包（BSP）、独立示例、经过验证的硬件事实，
以及面向 agent 的开发指引。没有云服务、没有设备配对、没有账户体系——
用本仓库构建的固件完全在设备本地运行。

## 面向对象

- **AI 编程 agent**（Codex、Muse Code、Claude Code 等）：仓库结构保证
  agent 能找到权威硬件事实、遵循任务路由表、并通过验证门禁，无需人工
  干预。入口是 [AGENTS.md](AGENTS.md)。
- **人类开发者**：以每块板的示例为起点开发自己的固件。
  入口是 [docs/README.md](docs/README.md)。

## 仓库结构

| 路径 | 内容 |
|---|---|
| `esp32/boards/<board>/` | 每块 ESP32 板卡的 sdkconfig、板级 BSP 和独立示例 |
| `esp32/components/` | 跨 ESP32 板卡共享的组件 |
| `linux/boards/<board>/` | Linux 设备（设备树 overlay、GPIO 映射、Python/C 示例） |
| `skills/` | 管理 develop / setup / build / device-test / debug 工作流的 agent skills |
| `docs/` | 硬件指南、厂商来源引用、开发规范 |
| `tools/validate.sh` | 每次交付必须通过的验证门禁 |
| `tests/` | 板级无关逻辑的宿主机单元测试 |

## 支持的板卡

| 板卡 | 平台 | 状态 |
|---|---|---|
| SenseCAP Watcher | ESP32-S3 | Phase 0：事实已收集，BSP 抽取待进行 |
| SenseCAP Indicator | ESP32-S3 | Phase 0：事实已收集，BSP 抽取待进行 |
| reTerminal E1001 | ESP32-S3 | Phase 0：事实已收集，BSP 抽取待进行 |
| reTerminal E1002 | ESP32-S3 | Phase 0：事实已收集，BSP 抽取待进行 |
| reSpeaker Lite (XIAO ESP32-S3) | ESP32-S3 | Phase 0：事实已收集，BSP 抽取待进行 |
| reTerminal (CM4) | Linux | 计划中 |

能力矩阵和抽取来源见 [docs/inventory.md](docs/inventory.md)，
每块板的权威上游参考见 [docs/vendor-sources.md](docs/vendor-sources.md)。

## 验证门禁

```bash
./tools/validate.sh --static           # 仓库检查 + 宿主机测试
./tools/validate.sh --esp32 <board>    # 单板 ESP-IDF 构建
./tools/validate.sh --linux <board>    # Linux 板卡检查
./tools/validate.sh                    # 完整门禁
```

## 许可证与署名

Apache License 2.0，见 [LICENSE](LICENSE)。第三方署名见 [NOTICE](NOTICE)。

<p align="right">
  <a href="README.md">English</a> · <strong>简体中文</strong>
</p>

# CodeCraft AI Hardware

面向 AI agent 驱动开发的 Seeed 设备硬件能力参考仓库。

这是一个**纯硬件**仓库：板级支持包（BSP）、独立示例、经过验证的硬件事实，
以及面向 agent 的开发指引。本仓库不包含云运行时、设备配对或账户体系——
用本仓库构建的固件完全在设备本地运行。

## 面向对象

- **AI 编程 agent**（Codex、Muse Code、Claude Code 等）：仓库结构保证
  agent 能找到权威硬件事实、遵循任务路由表、并通过验证门禁，无需人工
  干预。入口是 [AGENTS.md](AGENTS.md)。
- **人类开发者**：以每块板的示例为起点开发自己的固件。
  入口是 [docs/README.md](docs/README.md)。

## CodeCraft Hub

[CodeCraft Hub](https://www.npmjs.com/package/@seeed-studio/sensecraft-cli?activeTab=readme)
是固件分享的云平台，不属于本仓库。

若要上传固件做线上烧录或分享，使用
[`@seeed-studio/sensecraft-cli`](https://www.npmjs.com/package/@seeed-studio/sensecraft-cli?activeTab=readme)
生成并上传。安装和用法见该页面，其中包括 CLI 自带的 agent skill。
AI agent 遵循 [AGENTS.zh_CN.md](AGENTS.zh_CN.md) 中的 CodeCraft Hub 一节，
并且只在用户要求上传、线上烧录或分享固件时这样做。

## 仓库结构

| 路径 | 内容 |
|---|---|
| `esp32/boards/<board>/` | 每块 ESP32 板卡的 sdkconfig、板级 BSP 和独立示例 |
| `esp32/components/` | 跨 ESP32 板卡共享的组件 |
| `linux/boards/<board>/` | Linux 设备（设备树 overlay、GPIO 映射、Python/C 示例） |
| `arduino/boards/<board>/` | Arduino 框架板卡（arduino-cli 草图）；见 `arduino/AGENTS.md` |
| `skills/` | 管理 develop / setup / build / device-test / debug 工作流的 agent skills |
| `docs/` | 硬件指南、厂商来源引用、开发规范 |
| `tools/validate.sh` | 每次交付必须通过的验证门禁 |
| `tests/` | 板级无关逻辑的宿主机单元测试 |

## 支持的板卡

| 板卡 | 平台 | 状态 |
|---|---|---|
| SenseCAP Watcher | ESP32-S3 | BSP 功能完整（摄像头、显示、触摸、音频、旋钮、电池、microSD）+ 8 个示例构建验证通过（IDF v6.0.2）；尚未上机验证 |
| SenseCAP Indicator | ESP32-S3 | BSP（RGB 显示、IO 扩展、按键、触摸）+ 示例构建验证通过（IDF v6.0.2）；尚未上机验证 |
| reTerminal E1001 | ESP32-S3 | BSP 功能完整（墨水屏、按键、LED、SHT4x、电池、RTC、充电器、蜂鸣器、PDM 麦克风、microSD）+ 4 个示例构建验证通过（IDF v6.0.2）；尚未上机验证 |
| reTerminal E1002 | ESP32-S3 | 与 E1001 相同 BSP，面板换为 Spectra 6 彩色；构建验证通过（IDF v6.0.2）；尚未上机验证 |
| reSpeaker Lite (XIAO ESP32-S3) | ESP32-S3 | BSP + 3 个示例构建验证通过（IDF v6.0.2）；尚未上机验证 |
| reTerminal (CM4) | Linux | 外设 overlay + 演示（按键/LED/蜂鸣器/传感器/RTC）；待上机验证 |

能力矩阵和抽取来源见 [docs/inventory.md](docs/inventory.md)，
上机验证清单见 [docs/on-device-validation.md](docs/on-device-validation.md)，
每块板的权威上游参考见 [docs/vendor-sources.md](docs/vendor-sources.md)。

## 验证门禁

```bash
./tools/validate.sh --static           # 仓库检查 + 宿主机测试
./tools/validate.sh --esp32 <board>    # 单板 ESP-IDF 构建
./tools/validate.sh --linux <board>    # Linux 板卡检查
./tools/validate.sh                    # 完整门禁
```

## 贡献

提交、拉取请求和 issue 规范见 [`.github/CONTRIBUTING.zh_CN.md`](.github/CONTRIBUTING.zh_CN.md)。
AI agent 从 [AGENTS.zh_CN.md](AGENTS.zh_CN.md) 开始。

## 许可证与署名

Apache License 2.0，见 [LICENSE](LICENSE)。第三方署名见 [NOTICE](NOTICE)。

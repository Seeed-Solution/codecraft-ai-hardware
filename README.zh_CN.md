<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# CodeCraft AI Hardware

**用你的 AI 编程助手，开发和修改 Seeed 硬件项目。**

CodeCraft AI Hardware 汇集设备知识、原生板级支持包（BSP）、示例和 Agent Skills，帮助你使用 **ESP-IDF、Arduino 和 Linux** 进行 AI 辅助嵌入式开发。从现有示例创建自己的项目，为编程助手提供所需的板卡上下文和开发流程。

[选择示例](#选择一个起点) · [快速开始](#快速开始) · [设备目录](devices/AGENTS.md) · [Agent Skills](skills/README.zh_CN.md) · [文档](docs/README.zh_CN.md) · [English](README.md)

## 你可以做什么

- **先了解板卡。** 修改固件前，找到设备参考资料、原生接口、工具链要求和已知限制。
- **从示例开始创作。** 导出包含 BSP、依赖和源码锁定信息的私有应用，沿用板卡的原生软件栈开发。
- **与 AI 编程助手协作。** 使用 Skills 完成环境配置、开发、构建、调试和经过授权的设备测试。
- **按需加入云端功能。** 通过独立 Skills 为项目扩展云端能力，例如文本生成、语音识别、语音合成和图像生成；本地原生开发无需云端账户。

## 选择一个起点

以下均为仓库已有的源码示例，可作为应用开发起点。硬件版本要求见各设备文档，环境配置见对应 Target 指南。

| 开始体验 | 设备与软件栈 | 示例源码 | 环境配置 |
| --- | --- | --- | --- |
| 探索墨水屏显示 | [reTerminal E1001](devices/reterminal-e1001/AGENTS.md) · ESP-IDF | [epd_hello](devices/reterminal-e1001/targets/examples/epd_hello) | [Target 指南](devices/reterminal-e1001/targets/README.zh_CN.md) |
| 探索显示与按键 | [Wio Terminal](devices/wio-terminal/AGENTS.md) · Arduino | [display_buttons](devices/wio-terminal/targets/examples/display_buttons) | [Target 指南](devices/wio-terminal/targets/README.zh_CN.md) |
| 从按键和 LED 入门 | [Grove Beginner Kit](devices/grove-beginner-kit/AGENTS.md) · Arduino | [button_led](devices/grove-beginner-kit/targets/examples/button_led) | [Target 指南](devices/grove-beginner-kit/targets/README.zh_CN.md) |

更多板卡和示例见[设备目录](devices/AGENTS.md)。生成的[原生接口与证据矩阵](docs/inventory.md)分别记录实现、构建和实机验证情况。当前源码对应的实机验收仍未完成，历史观察记录不能证明新版本源码的物理表现。

## 快速开始

使用 Codex、Claude Code、Cursor、ZCode、WorkBuddy、Muse 或其他编程 Agent？
从[通用 Agent 接入指南](docs/development/agent-client-setup.md)开始。
指南提供适用于新应用和已有项目的统一请求模板，也可通过客户端的原生 Skill
或项目规则进入同一工作流。客户端体验测试留给用户。

### 1. 获取仓库并选择板卡

准备 Git 和独立的 CodeCraft CLI。克隆后用 `sh tools/cli/install.sh` 安装最新正式版 Release 二进制（Windows：`tools/cli/install.ps1`）。仅在本地开发 CLI 时使用 `--from-source`（Windows：`-FromSource`）。下载的二进制无需 Python 或 Rust。参见[安装与更新策略](docs/development/rust-cli.md)。

```sh
git clone https://github.com/Seeed-Solution/codecraft-ai-hardware.git
cd codecraft-ai-hardware
codecraft discover --interface display
```

以下流程以 **reTerminal E1001 / ESP-IDF** 为例。使用其他板卡时，请遵循它的 Target 指南和声明的软件栈。

```sh
codecraft context --target esp-idf/reterminal-e1001
```

阅读返回的上下文和 [E1001 环境配置指南](devices/reterminal-e1001/targets/README.zh_CN.md#工具链)。构建前安装并激活 [target.json](devices/reterminal-e1001/targets/target.json) 声明的精确工具链版本；与 Agent 协作时，使用 [codecraft-development-setup](skills/codecraft-development-setup/SKILL.md) 处理缺失工具。

### 2. 创建自己的工作区

在仓库根目录运行：

```sh
codecraft new --target esp-idf/reterminal-e1001 --example epd_hello --output projects/my-e1001-project
codecraft verify-project projects/my-e1001-project
```

输出目录必须尚不存在。`projects/` 已被 Git 忽略，便于将自己的应用与参考示例分开；也支持显式指定仓库外目录。`project.yaml` 记录选择，`codecraft.lock` 记录导入源码与工具链来源。使用已安装的独立可执行文件；修改项目不会重新构建 CLI。

### 3. 构建所选示例

激活声明的工具链后运行：

```sh
codecraft lock-project projects/my-e1001-project --toolchain
codecraft build-project projects/my-e1001-project
```

导出的工作区记录 CLI 身份，不携带 CodeCraft Python 运行时，在该目录内可运行 `codecraft build-project .`。构建不会刷写设备。需要实机测试时，先确认板卡和端口并明确授权设备写入，再遵循所选[设备验收步骤](devices/reterminal-e1001/AGENTS.md#physical-acceptance)和 [codecraft-development-device-test](skills/codecraft-development-device-test/SKILL.md)。

应用修改、需求简述、经审阅的重新锁定和后续构建，参见[可移植应用流程](docs/development/application-workflow.md)。使用 [codecraft-development-ideate](skills/codecraft-development-ideate/SKILL.md) 和 `new --brief <file.json>`，将用途、用户、场景和物料清单记录到项目中。

## 与 AI 编程助手协作

在编程助手中打开本仓库，让它先阅读 [AGENTS.md](AGENTS.md)。AI 辅助开发前，将以下五个[硬件工作流 Skill](skills/README.zh_CN.md) 的完整目录安装到助手的 Skill 发现位置：`codecraft-development-develop`、`codecraft-development-setup`、`codecraft-development-build`、`codecraft-development-device-test` 和 `codecraft-development-debug`。仓库合同要求 Agent 安装并核实缺失的工作流。

例如，可以这样开始一个新应用：

```text
阅读 AGENTS.md，并使用要求的 CodeCraft Skills。
我有一台 reTerminal E1001，请从 epd_hello 示例创建私有项目，
输出到 projects/my-e1001-project。
检查板卡上下文和声明的工具链，然后构建。
报告构建结果与未知项。刷写前先让我确认板卡、端口，
并明确授权设备写入。
```

如果已经完成快速开始，请换一个尚不存在的输出路径。对于已有应用，在助手中打开应用工作区并遵循生成的 `AGENTS.md`。Agent 工作流涵盖环境配置、开发、构建、调试和设备测试，原生命令由所选 Platform 负责。

## 可选的云端功能与固件分享

通过 [skills/](skills/README.zh_CN.md#云端能力) 中的独立客户端扩展云端能力，例如 LLM 文本生成、ASR/STT、TTS 和图像生成。每个客户端拥有自己的 `contract.json` 和 API 参考，可独立安装。云端服务和账户基础设施由外部项目负责；凭据、限制与经过授权的 API 使用方式，见所选 Skill。

每台设备每月可调用次数及 5 台设备的每月合计见[免费额度表](skills/README.zh_CN.md#免费额度)。各功能共享设备每月额度，表中各行是不同使用方式下的估算。

将客户端组合进应用时，CLI 契约版本 4 使用 `new --skills <selection.json>` 显式选择；项目配置与锁文件采用 schema 4。参见[项目组合](skills/README.zh_CN.md#可选项目组合)和生成的 [Skill 索引](docs/skill-inventory.json)。

[CodeCraft Hub](https://codecraft.seeed.cc/hub) 是独立的固件分享平台。用户要求上传、在线刷写、SenseCraft 设备管理或云端部署时，使用外部 [sensecraft-cli](https://github.com/Seeed-Solution/sensecraft-cli) 及其官方内置 Skill。已核实版本、前提和安装方式见[安装与任务路由](docs/development/sensecraft-cli.md)。原生开发无需此工具，安装 Skill 或 CLI 不代表授权云端操作。

## 文档与贡献

| 你想要…… | 从这里开始 |
| --- | --- |
| 查找设备知识和原生示例 | [设备目录](devices/AGENTS.md) |
| 查看接口实现与证据状态 | [原生接口矩阵](docs/inventory.md) |
| 修改导出的项目 | [应用开发流程](docs/development/application-workflow.md) |
| 了解硬件与云端 Skills | [Skills 指南](skills/README.zh_CN.md) |
| 理解职责划分和架构 | [架构说明](docs/architecture/overview.md) |
| 报告问题或参与贡献 | [Issues](https://github.com/Seeed-Solution/codecraft-ai-hardware/issues) · [贡献指南](.github/CONTRIBUTING.md) |
| 浏览全部指南和待办 | [文档目录](docs/README.zh_CN.md) · [实施进度](docs/development/v0.2-todo.md) |

### 仓库结构

| 路径 | 内容 |
| --- | --- |
| `devices/<device>/` | 设备知识、参考资料、素材，以及 `targets/` 下的原生 BSP 和示例 |
| `platforms/` | 软件栈适配器、构建辅助和共享组件 |
| `skills/` | `codecraft-development-*` 开发工作流与 `codecraft-capabilities-*` 独立云端能力 Skill |
| `tools/` | `codecraft/` 负责本地 CLI 与项目编排；`skill_packages/` 负责客户端维护与包发布记录 |
| `projects/` | 被 Git 忽略的私有应用工作区 |
| `tests/` | 按源码归属组织的测试、夹具、mock 和留存的设备证据 |
| `docs/` | 工程指南、生成的索引和待办事项 |

### 贡献者检查

遵循[仓库合同](AGENTS.md)、[维护入口](CONTRIBUTION.md)和[测试归属指南](docs/development/testing.md)。源码元数据变化后执行 `codecraft maintenance generate`，再检查生成结果漂移和静态/主机行为：

```sh
codecraft maintenance generate --check
codecraft validate --static
```

仅修改文档时无需原生构建。固件或原生构建发生变化时，按[验证策略](AGENTS.md#required-validation-and-delivery)执行对应范围的检查；不带选项的 `codecraft validate` 会运行完整软件门禁。构建结果、仿真和实机观察分别报告。生成的固件和临时构建产物不跟踪，留存的脱敏证据位于 `tests/devices/<device>/`。

修改独立客户端包时，遵循[候选包准备、检查、审阅与应用流程](docs/development/skill-releases.md)。打包不等于安装或发布 Skill。尚未完成的验收和仿真工作见上方实施进度文档。

## 许可证

[Apache License 2.0](LICENSE)。第三方归属保留在 [NOTICE](NOTICE)。

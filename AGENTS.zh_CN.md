<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# AI Agent 仓库指南

本文件是在本仓库进行 AI 辅助开发的唯一强制入口。根据下方路由表阅读
任务相关文档；不要默认加载所有 README。

## 本仓库是什么

Seeed 设备的纯硬件能力参考仓库。包含板级支持包、独立示例和经过验证的
硬件事实。刻意**不包含云运行时**：没有设备配对、没有账户体系、没有远程
协议、没有遥测。不要把这些内容引进本仓库。

## CodeCraft Hub

[CodeCraft Hub](https://codecraft.seeed.cc/hub/)
是独立的固件分享云平台。本仓库不实现它。

当用户希望上传固件、做线上烧录或分享时，使用
npm 发布包中的 `sensecraft-cli`：

- 尚未安装 CLI 时，运行 `npx @seeed-studio/sensecraft-cli install`。
- 更新已有安装时，运行 `sensecraft-cli update`。
- 运行 `sensecraft-cli --version` 验证安装。

按照 [CLI 文档](https://sensecraft.seeed.cc/wiki/en/sensecraft-cli/)
完成授权，并加载 CLI 随包提供的 agent skill。使用上面的 npm 发布版本命令，
不要通过克隆或构建 CLI 的 GitHub 仓库来安装。仅在用户提出上传请求时这样做：
普通的构建、刷机或板级改动不是上传请求。硬件事实仍以本仓库为准。

## 必需的 AI skills

在本仓库进行 AI 辅助开发需要以下五个 skills：`codecraft-develop`、
`codecraft-setup`、`codecraft-build`、`codecraft-device-test`、
`codecraft-debug`。它们的维护源码位于 `skills/` 目录。

开始开发前，AI 必须检查这五个 skills 是否已安装并在当前环境中可用。
如有缺失，AI 必须自行选择合适的安装方式完成安装并验证可用性。不要
等待用户提出安装请求。不强制使用特定的脚本、安装目录或 AI 客户端。

五个 skills 全部可用不代表每个任务都要加载或调用全部五个：只使用与
任务匹配的 skills，并保留刷机、Git 写操作和发布的单独授权要求。

## 项目与安全基线

- 本仓库是多板卡、多平台仓库。ESP32 固件位于 `esp32/`，Linux 设备
  支持位于 `linux/`，Arduino 框架板卡位于 `arduino/`（见
  `arduino/AGENTS.md`）。各平台构建系统绝不混用。
- 硬件事实遵循以下优先级：**厂商规格书与实测结果 → 板卡引脚头文件 →
  BSP 实现 → 硬件指南 → README/示例代码**。如果任务所需的硬件细节在
  这些来源中均未定义，向用户提问，不要猜测。绝不编造引脚号。
- 代码中的每条硬件事实必须在附近注释中引用其来源（厂商原理图、厂商
  BSP 或实测结果）。见 [docs/vendor-sources.md](docs/vendor-sources.md)。
- 保留用户的现有改动。开始工作前执行 `git status --short --branch`；
  绝不覆盖或清理无关文件。
- 刷新固件不需要备份设备上已有的固件；不要把 Flash 回读作为前置条件。
  这不保证保留用户数据，也不授权整片擦除。
- 可复用的板级逻辑属于该板的 BSP 目录或 `esp32/components/`；页面、
  状态机、动画和应用任务属于示例或应用层。
- 绝不提交凭据、设备密钥、私钥、个人数据或未脱敏的日志。
- 每个维护中的 Markdown 文档以英文作为默认 `.md` 路径，简体中文位于
  配套的 `.zh_CN.md` 文件。保持两个版本同步并保留互链。（当前合规矩
  为根级 `README` 和 `AGENTS` 双语对；其余文档在内容稳定后补齐翻译。）

## 任务上下文路由

| 任务 | 编辑前必读 |
| --- | --- |
| 任何代码改动 | `docs/development/ai-guide.md`、相关头文件与邻近实现 |
| 环境引导或工具链缺失 | `skills/codecraft-setup/SKILL.md` |
| ESP32 板卡工作（引脚、总线、显示、音频、电池） | `esp32/AGENTS.md`、对应板的 `esp32/boards/<board>/README.md`、`docs/vendor-sources.md` |
| Linux 板卡工作 | `linux/AGENTS.md`、对应板的 `linux/boards/<board>/README.md` |
| Arduino 板卡工作 | `arduino/AGENTS.md`、对应板的 `arduino/boards/<board>/README.md`、`docs/vendor-sources.md` |
| 新增板卡 | `docs/development/ai-guide.md`（板卡接入章节）、`docs/vendor-sources.md` |
| 构建、测试、依赖、分区表 | `skills/codecraft-build/SKILL.md`、该板的 sdkconfig 默认值 |
| 刷机或设备上测试 | `skills/codecraft-device-test/SKILL.md` |
| 通过 CodeCraft Hub 上传、线上烧录或分享固件 | 本文件的 CodeCraft Hub 一节，然后阅读 [CLI 文档](https://sensecraft.seeed.cc/wiki/en/sensecraft-cli/)。不要在此添加云运行时。 |
| 文档 | 本文件的双语规则、`docs/README.md` |
| 提交或 PR | `docs/development/ai-guide.md`（交付格式章节）、`.github/CONTRIBUTING.md`、`.github/PULL_REQUEST_TEMPLATE.md` |

## 必需的验证与交付

迭代时运行最小相关检查，交付前运行完整门禁：

```bash
./tools/validate.sh --static           # 仓库检查 + 宿主机测试
./tools/validate.sh --esp32 <board>    # 受影响板卡的 ESP-IDF 构建
./tools/validate.sh                    # 完整门禁
```

ESP-IDF 门禁需要已激活的 ESP-IDF 环境；每块板在其 README 中声明所需
的 IDF 版本。不要把构建成功描述为硬件验证。最终交付必须分别报告以下
字段：

1. 静态检查结果。
2. 每块受影响板卡的构建结果。
3. 设备上验证结果，或明确声明未执行。

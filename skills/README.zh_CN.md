<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# Agent Skills

Skills 负责硬件开发工作流和独立云端客户端。设备事实与原生实现归 devices/，共享软件栈归 platforms/。阅读 [Skill 规则](AGENTS.md)与[契约归属](../docs/architecture/skill-contracts.md)。

| 分类 | 包命名 | 用途 |
| --- | --- | --- |
| 开发工作流 | `codecraft-development-*` | 硬件应用的发现、开发、环境配置、构建、调试与验证 |
| 云端能力 | `codecraft-capabilities-*` | 通过 CodeCraft Device API 使用 ASR、TTS、文本生成和图像生成 |

每个 Skill 都是 skills/ 的直接子目录，拥有自己的 SKILL.md；复制一个完整目录即可安装。仓库工具统一位于 [tools/](../tools/AGENTS.md)，与可安装的 Skill 分开。

## 硬件开发

以 codecraft-development-develop 作为通用应用开发入口。
按[通用 Agent 接入指南](../docs/development/agent-client-setup.md)选择原生 Skill
发现、项目规则引用或显式阅读文档。开发 Skill 需要从所选仓库或导出应用读取
硬件资料与工作流；Skill 安装目录不是该工作区。独立的云能力和 reCamera 包
继续使用各自包内的使用说明。

必需工作流为 codecraft-development-develop、codecraft-development-setup、codecraft-development-build、codecraft-development-device-test 和 codecraft-development-debug。使用 codecraft-development-ideate 整理应用需求，按需使用 codecraft-development-discover 和 codecraft-development-validate。创作流程编排位于 tools/cli/，仓库审计位于 tools/cli/，原生命令仍由对应 Platform 实现。[可移植工作流](../docs/development/application-workflow.md)及所选 Platform、工作流指南随应用导出并纳入锁文件。

## reCamera Pro SDK 应用

[codecraft-development-recamera-pysdk](codecraft-development-recamera-pysdk/SKILL.md)
用于 reCamera Pro / RV1126B Python SDK 应用开发、离线验证和 App Center 打包。
复制完整 Skill 目录即可独立使用。该工作流锁定官方 SDK 打包器和 API 参考，
不注册原生 Target，也不表示已验证实体板卡支持。设备上传、安装和测试仍是
需要相应授权的独立操作。参见其[来源记录](codecraft-development-recamera-pysdk/references/provenance.md)。
参考文档生成和经过审阅的包文件清单由仓库工具维护，不随 Skill 安装。
参见 [SDK 维护流程](../docs/development/recamera-sdk-maintenance.md)和
[优化 TODO](../docs/development/recamera-sdk-todo.md)。

## 外部 SenseCraft CLI

平台认证、云端设备、遥测、Hub、文件、模型库和云端部署使用 @seeed-studio/sensecraft-cli 自带的官方 Skill，遵循[安装与路由指南](../docs/development/sensecraft-cli.md)。仅在需要时安装到仓库外；原生开发无需此工具，安装不授权平台操作。

## 云端能力

| Skill | 操作 |
| --- | --- |
| [codecraft-capabilities-llm](codecraft-capabilities-llm/SKILL.md) | 文本生成与 SSE |
| [codecraft-capabilities-asr](codecraft-capabilities-asr/SKILL.md) | WAV 转录、实时 ASR/STT、状态查询与恢复 |
| [codecraft-capabilities-tts](codecraft-capabilities-tts/SKILL.md) | WAV 合成与可选归档 |
| [codecraft-capabilities-image-generation](codecraft-capabilities-image-generation/SKILL.md) | 可配置尺寸的图像生成与可选归档 |

每个包包含 contract.json、SKILL.md、公开 API/设备接入/原生集成参考、可移植的 C 认证头文件、Agent 元数据和包内脚本。复制完整目录即可独立使用。MCU/Linux 集成使用设备自己的网络栈；Python 和 SenseCraft CLI 是工作站工具，不是固件依赖。可选辅助脚本需要 Python 3.10+；实时流需要 websockets==15.0.1，图像解码需要 Python 3.11+ 和 Pillow==13.0.0，按各包说明配置环境。无需硬件目录、兄弟包或仓库 CLI。准确限制、默认值、计费声明和未知项由该包契约与 API 参考说明；[生成索引](../docs/skill-inventory.json)提供 ID、操作及各自版本。

各包的接入参考说明 API 地址来源、EUI/DeviceKey 获取、私有文件导入及访问检查；共同流程见 [LLM 接入参考](codecraft-capabilities-llm/references/onboarding.md)。优先复用现有配置。文档所依据的 CLI 版本尚未导出 CodeCraft Device API 地址，仅有平台网关不能证明对应服务映射。缺少地址时应保留现有凭证，不应阻断独立的本地导入或已配置环境中的 CLI 设备接入操作。

执行已授权 API 任务前，确认网络/TLS、隐私决定和凭证引用可用性。按包内说明配置 CODECRAFT_API_ORIGIN 与 CODECRAFT_DEVICE_FILE，真实凭证、输入和输出保存在包外。服务路由固定，不能传入模型、供应商或音色覆盖参数。LLM 成功需要正常结束状态；实时恢复使用原始幂等键，不会自动重放音频。媒体默认保存在本地，仅在明确需要时归档；归档失败保留本地结果，不授权再次生成。

直接运行所选 Skill 的入口，例如在已安装目录执行：

```sh
python3 scripts/transcribe.py --file /work/recording.wav
```

客户端校验真实文件、认证、协议与输出。即使存在部分文本或有效本地文件，非零退出仍表示失败。禁止自动重试收费请求；主机侧执行不证明设备集成或线上服务验收。

## 免费额度

每台设备每月获得下列免费额度，由各项 AI 功能共享。表中每一行均按该功能独占设备当月全部未使用额度估算，**各行是不同使用方式，不能相加**。

| 功能 | 每台设备每月可调用 | 每用户 5 台设备每月合计 |
| --- | --- | --- |
| 文件 ASR / STT（每次最多 60 秒） | **62 次** | **310 次** |
| 实时 ASR（每会话最多 60 秒） | **62 个会话** | **310 个会话** |
| TTS（每次最多 500 个输入字符） | **62 次** | **310 次** |
| 生图 | **6 张** | **30 张** |
| LLM：输入 100＋输出 50 token | **约 41 次** | **约 205 次** |
| LLM：输入 100＋输出 100 token | **约 31 次** | **约 155 次** |
| LLM：输入 200＋输出 100 token | **约 20 次** | **约 100 次** |

5 台设备的合计是各台设备可完成调用次数之和，不是可合并使用的账户余额。LLM 次数根据表中的 token 用量估算；实际用量包括完整请求上下文和生成内容，较长的对话或回复会减少可调用次数。使用任一功能都会消耗其他功能可用的额度。每月额度的更新由服务端管理，重复检查设备访问状态不会补充额度。

## 可选项目组合

`new --skills <selection.json>` 与 `context --skills <selection.json>` 使用显式列表，例如：

```json
[
  {
    "skill": "codecraft-capabilities-asr",
    "configuration": {},
    "environment": {
      "network": true,
      "tls": true,
      "privacy_approved": true,
      "environment_refs": ["CODECRAFT_API_ORIGIN"],
      "secret_refs": ["CODECRAFT_DEVICE_FILE"]
    },
    "limits": {}
  }
]
```

这些字段记录已确认条件和引用名，不是凭证或 API 授权。缺失条件或受约束但未知的资源会使选择失败。纯原生项目不包含云端契约与客户端；显式选择后，精确包内容、包外发布记录和来源进入同一源码图与锁文件。

## 维护

tools/skill_packages/packages/ 中的发布记录锁定全部分发文件，包括二进制素材与 contract.json；tools/skill_packages/import.json 保留原始来源，tools/skill_packages/upstream.json 单独记录已审查的上游源快照。维护包更名后，上游来源路径仍保留原始名称。共享运行时辅助代码在 tools/skill_packages/ 维护，使用[候选、检查、审查与应用流程](../docs/development/skill-releases.md)。包、API、语义契约及策略版本分别审查。应用必须指定准确摘要，重新运行隔离一致性检查并拒绝来源漂移；打包不会安装或发布 Skill。

执行以下仓库根目录检查前，先激活[宿主机测试环境](../docs/development/testing.md)；安装依赖不会调用 API。

```sh
codecraft maintenance check-cloud-skills
codecraft maintenance test --path tests/tools/skill_packages/test_protocol.py
codecraft validate --static
```

check-cloud-skills 检查仓库包，--skills-root 检查已安装副本。缺失、多余文件和符号链接均失败；缓存不纳入清单，CLI 检查使用新复制的已锁定源码且不继承 PYTHONPATH。各 Skill 测试位于 tests/skills/<skill-id>/，共享契约位于 tests/tools/cli/，安装、协议与发布测试位于 tests/tools/skill_packages/。仅使用本地回环服务与合成凭证；真实 API 验收仍是[待办](../docs/development/optimization-plan.md)。

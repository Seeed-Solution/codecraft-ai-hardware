<p align="right">
  <a href="CONTRIBUTING.md">English</a> · <strong>简体中文</strong>
</p>

# 为 CodeCraft AI Hardware 做贡献

这是一个纯硬件能力参考仓库：面向 Seeed 设备的板级支持包（BSP）、
独立示例和经过验证的硬件事实。仓库刻意不包含云端运行时、设备配对、
账号系统或遥测——贡献内容不得引入这些。

## 开始之前

- **人类开发者**：从 [docs/README.md](../docs/README.md) 和
  `esp32/boards/` 或 `linux/boards/` 下的板卡 README 开始。
- **AI 编码 agent**：从 [AGENTS.md](../AGENTS.md) 开始；它是包含任务
  路由表的强制入口。
- 硬件事实遵循严格的来源优先级：厂商规格与实测结果 → 板卡引脚头文件
  → BSP 实现 → 硬件指南 → README/示例代码。绝不编造引脚号；每条硬件
  事实都要在附近注释中标注来源。

## 提交（Commit）

- 使用英文祈使句，遵循 Conventional Commit 格式：
  `type(scope): description`，type 取 `feat`、`fix`、`docs`、`refactor`、
  `perf`、`test`、`chore`、`build` 或 `ci`。
- 每个 commit 应可独立评审。描述最终 diff，而不是调试过程。
- 排除凭证、生成的固件、构建产物和无关文件。

## 拉取请求（PR）

1. Fork 本仓库，从 `main` 创建分支。
2. 填写 [`.github/PULL_REQUEST_TEMPLATE.md`](PULL_REQUEST_TEMPLATE.md)。
   静态检查、各板卡构建和上机验证要分开报告；未做的硬件验证列入
   `Unverified`。
3. 提交 PR 前运行验证门禁：

   ```bash
   ./tools/validate.sh --static           # 仓库检查 + 宿主侧测试
   ./tools/validate.sh --esp32 <board>    # 受影响板卡的 ESP-IDF 构建
   ./tools/validate.sh --linux <board>    # Linux 板卡检查
   ```

4. 构建成功不等于硬件验证通过。引脚、显示、总线和电源类改动需要先
   提供板卡版本号和实测结果才能合并；显示类改动请附照片。
5. 受维护的文档保持双语对（`.md` / `.zh_CN.md`）同步。

## 问题（Issue）

使用 GitHub issue 模板：缺陷报告、功能建议或使用问题。请始终注明
板卡名称、硬件版本和 ESP-IDF 版本（如适用）。绝不要在 issue 中包含
凭证、私密设备数据或个人数据。

## 许可证

提交贡献即表示你同意按 Apache License 2.0 许可你的贡献；见
[LICENSE](../LICENSE)。第三方署名记录于 [NOTICE](../NOTICE)。

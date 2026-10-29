<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# Arduino 平台规范

适用于 `arduino/` 下的全部内容。根目录 `AGENTS.md` 优先，本文件追加
平台专属规则。

## 范围

使用 Arduino 框架（arduino-cli / Arduino IDE）开发的板卡，无论芯片是
SAMD（例如 Wio Terminal）、AVR（例如 Grove Beginner Kit 的 Seeeduino
Lotus）、ESP32 还是其他；同一硬件若使用 ESP-IDF 开发，应放在 `esp32/`。

## 板卡目录契约

每个 `arduino/boards/<board>/` 目录必须包含：

- `fqbn.txt` — 一行 arduino-cli 完整板卡名，例如
  `Seeeduino:samd:seeed_wio_terminal`。
- `core-version.txt` — 一行用于验证的精确核心版本 `ID@version`，且核心 ID
  必须与 FQBN 一致，例如 `Seeeduino:samd@1.8.6`。
- `libraries.txt` — Arduino Library Manager 依赖，每行一个精确的
  `Name@x.y.z`；允许注释与空行。随厂商核心内置的 LCD 库应在注释中说明，
  不要重复安装。
- `README.md` 与 `README.zh_CN.md` — 带引用的已验证硬件事实、非内置核心的
  Board Manager URL、能力矩阵、限制/来源冲突、精确工具链版本，以及与示例
  目录完全一致的列表。
- `AGENTS.md` 与 `AGENTS.zh_CN.md` — 板级 agent 指令：构建/烧录规则、总线
  归属、固件前提、已知编译陷阱与上机验证约定。
- `examples/<name>/` — 独立草图，包含 `<name>.ino`、`AGENTS.md` 与
  `AGENTS.zh_CN.md`。每个 agent 对必须说明目的、带来源的硬件事实、精确
  构建命令、预期 LCD/串口行为、物理测试步骤与限制。

## 新板接入

1. 先在 `docs/vendor-sources.md` 添加厂商来源行。
2. 按上述目录契约建板；每个硬件事实记录来源，核心与库版本必须精确。
3. 在根 README 表格与 `docs/inventory.md` 注册板卡。
4. 使用 `./tools/validate.sh --static` 与
   `./tools/validate.sh --arduino <board>` 验证。目录落地后，CI 会自动发现
   `arduino/boards/` 下的新板。

## 规则

- 硬件事实优先级与其它平台相同（见根 `AGENTS.md`）：绝不猜引脚。
- 优先使用核心定义的宏（例如 Wio Terminal 的 `WIO_KEY_A`），同时仍需引用
  厂商来源。
- 在板卡 README 中固定核心版本以及所有外部库的精确版本。
- 草图保持独立：不提交 `libraries/` 目录，不依赖共享 sketchbook。
- 非 Seeed 核心若需要不同 Board Manager URL：写入板卡 README，并加入
  `.github/workflows/arduino-build.yml`。
- 使用 arduino-cli 构建：
  `arduino-cli compile --fqbn "$(cat fqbn.txt)" examples/<sketch>`（在板卡
  目录中），或在仓库根目录使用相对路径。
- 没有用户明确授权，不得烧录板卡或更新无线协处理器固件。成功编译不等于
  硬件验证。

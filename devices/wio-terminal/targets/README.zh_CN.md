<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# arduino/wio-terminal

硬件知识：[AGENTS.md](../AGENTS.md)。阅读[工具链安装说明](README.md#toolchain)，然后在仓库根目录运行 `codecraft validate --target arduino/wio-terminal --projects`。

Target 清单定义 FQBN、依赖版本和独立示例。使用 --example 选择一个示例。构建不会上传；实机步骤由 [Device 验收清单](../AGENTS.md#physical-acceptance)维护。

## Target 实现

硬件行为和实现参考见 [Device 入口](../AGENTS.md#implementation-references)。本目录维护 Arduino 软件栈的接口绑定、运行约束和构建配置。

- BLE 初始化后必须调用一次 `ble_start()`，否则 BT 栈未启动、地址全零；使用非阻塞 `BLEScan::start`，阻塞版本曾使 LCD 等待无超时信号量。详细实现约束见 `examples/ble_scan/AGENTS.md`。
- Wi-Fi 与 BLE 示例分别拥有协处理器；一次运行一个示例，仓库没有验证两者并发。rpcBLE 1.0.0 与 Arduino `min`/`max` 宏冲突，包含 rpcBLE 头文件前必须取消这两个宏定义。
- `sd_card_log` 使用不超过 16 GB 的 FAT32 卡，仅挂载一次，不支持热插拔/重新挂载；GPIO65 插卡检测未被使用。采用 Seeed Arduino FS 2.1.5、显式 SPI2 与 CS64，不能替换为默认使用 SS GPIO50 的标准 SD 库。

## 工具链

以 `target.json` 为可执行版本声明，使用 Arduino CLI 1.5.0 及以下精确依赖。库版本来自该板维护的依赖声明；捆绑驱动来自锁定 core。

```sh
arduino-cli core update-index --additional-urls https://files.seeedstudio.com/arduino/package_seeeduino_boards_index.json
arduino-cli core install Seeeduino:samd@1.8.6 --additional-urls https://files.seeedstudio.com/arduino/package_seeeduino_boards_index.json
arduino-cli lib install --no-deps "Seeed Arduino rpcUnified@2.1.4"
arduino-cli lib install --no-deps "Seeed_Arduino_mbedtls@3.0.2"
arduino-cli lib install --no-deps "Seeed Arduino FS@2.1.5"
arduino-cli lib install --no-deps "Seeed Arduino SFUD@2.0.2"
arduino-cli lib install --no-deps "Seeed Arduino rpcWiFi@1.1.0"
arduino-cli lib install --no-deps "Seeed Arduino rpcBLE@1.0.0"
```

用 ARDUINO_DIRECTORIES_DATA、ARDUINO_DIRECTORIES_DOWNLOADS、ARDUINO_DIRECTORIES_USER 隔离安装目录。适配器对所选 core、原生工具和锁定库计算指纹，并显式传递库路径；无关 core 不进入锁文件。构建输出位于 .codecraft/builds/arduino/。

导出应用使用 `codecraft build-project .`；仓库级验证命令不能替代应用锁文件检查。

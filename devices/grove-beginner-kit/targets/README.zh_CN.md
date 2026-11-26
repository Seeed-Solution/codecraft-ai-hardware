<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# arduino/grove-beginner-kit

硬件知识：[AGENTS.md](../AGENTS.md)。阅读[工具链安装说明](README.md#toolchain)，然后在仓库根目录运行 `codecraft validate --target arduino/grove-beginner-kit --projects`。

Target 清单定义 FQBN、依赖版本和独立示例。使用 --example 选择一个示例。构建不会上传；实机步骤由 [Device 验收清单](../AGENTS.md#physical-acceptance)维护。

## 原生实现

### 构建身份

`arduino:avr:uno` 使用 ATmega328P、standard variant、16 MHz、AVR_UNO 和 32,256 字节 sketch 上限，与 Lotus v1.1 的配置一致。来源为 Seeeduino AVR 1.2.1 boards.txt 与 [ArduinoCore-avr 1.8.8](https://github.com/arduino/ArduinoCore-avr/tree/1.8.8)。Target 仅支持声明的 core。

## 工具链

以 `target.json` 为可执行版本声明，使用 Arduino CLI 1.5.0 及以下精确依赖。库版本来自该板维护的依赖声明；捆绑驱动来自锁定 core。

```sh
arduino-cli core update-index
arduino-cli core install arduino:avr@1.8.8
arduino-cli lib install --no-deps "U8g2@2.36.19"
arduino-cli lib install --no-deps "Grove Temperature And Humidity Sensor@2.0.2"
arduino-cli lib install --no-deps "Seeed Arduino SPA06@1.0.1"
arduino-cli lib install --no-deps "Grove-3-Axis-Digital-Accelerometer-2g-to-16g-LIS3DHTR@1.2.4"
```

用 ARDUINO_DIRECTORIES_DATA、ARDUINO_DIRECTORIES_DOWNLOADS、ARDUINO_DIRECTORIES_USER 隔离安装目录。适配器对所选 core、原生工具和锁定库计算指纹，并显式传递库路径；无关 core 不进入锁文件。构建输出位于 .codecraft/builds/arduino/。

导出应用使用 `codecraft build-project .`；仓库级验证命令不能替代应用锁文件检查。

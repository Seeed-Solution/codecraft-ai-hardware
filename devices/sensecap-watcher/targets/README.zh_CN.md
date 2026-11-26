<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# esp-idf/sensecap-watcher

设备知识：[AGENTS.md](../AGENTS.md)。硬件事实在该入口及其来源中维护。

在仓库根目录构建/检查：`codecraft validate --target esp-idf/sensecap-watcher`。

示例、原生接口引用和工具链选择见 `target.json`。构建结果与设备观测分别记录。

BSP 公共头文件位于 `bsp/include/`，实现与私有头文件位于 `bsp/src/`。每个示例拥有独立的 `bsp.cmake` 和托管依赖声明；导出项目包含该示例选择的接口及源码闭包。

## 单文件镜像

执行 `codecraft validate --target esp-idf/sensecap-watcher --example battery_status` 后，运行 `codecraft firmware plan --target esp-idf/sensecap-watcher --example battery_status --output .codecraft/watcher-plan.json`，再运行 `codecraft firmware merge --plan .codecraft/watcher-plan.json --output .codecraft/watcher-release` 和 `codecraft firmware verify .codecraft/watcher-release`。保持声明的 ESP-IDF 环境处于激活状态，输出必须是新路径。factory BIN 与清单绑定成功构建、原生布局和输入哈希。此流程不会烧录或上传。

## Target 实现

设备行为、接口参考和示例用途见 [Device 实现参考](../AGENTS.md#implementation-references)。Target 负责当前软件栈下的可构建实现、原生接口绑定和构建配置。

- 音频 BSP 使用 esp_codec_dev 1.6.2 与原生 IDF I2C 设备句柄；SPD2010 触摸采用原生 I2C 轮询。BSP 负责总线、句柄、共享显示电源和复位路由。

## 工具链

`target.json` 要求 ESP-IDF 6.0.2、esp32s3；构建前激活该版本 SDK。

导出应用使用 `codecraft build-project .`；仓库级验证命令不能替代应用锁文件检查。

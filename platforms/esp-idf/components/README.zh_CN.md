<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# 共享 ESP32 组件

此处组件必须供至少两块板卡使用。仅单板使用的驱动应放在对应板卡的 `bsp/` 目录。

已落地：

- `codecraft_sht4x/` — E1001/E1002 共享的 SHT4x 协议、CRC 和转换解码。每个传感器串行执行完整采样事务，转换期间允许总线上的其他设备访问；BSP 提供总线、地址和速率。
- `codecraft_pcf8563/` — E1001/E1002 共享的 PCF8563 时间日期协议，保留现有 2000–2099 年约定。BSP 保留板级接线和进程生命周期所有权；组件校验字段范围及响应 BCD。
- `sscma_client/` — 通用静态相机抽象及 SSCMA-over-SPI 后端（SenseCAP Watcher 上的 Seeed Himax WiseEye2，也适用于 Grove Vision AI V2）。提取自硬件参考快照的 `esp32/components/camera/`（Apache 2.0，保留 Meta 版权头，标明修改）。

提取的传感器协议保留原有来源与许可证注释。各 BSP 继续保留板级 I2C0 辅助实现：它负责本板总线与引脚，要求串行初始化，不代表不同板卡之间具有硬件兼容性。
传感器公开契约位于组件的 `include/`，实现位于 `src/`。可执行 C 用例与 Python 运行器统一位于 [tests/platforms/esp-idf/components/](../../../tests/platforms/esp-idf/components/)。
SHT4x 与 PCF8563 运行器使用有限的 I2C/任务替身，在仓库根目录执行 `codecraft validate --static` 即可运行。
这些检查验证软件行为，不验证物理时序或传感器精度。

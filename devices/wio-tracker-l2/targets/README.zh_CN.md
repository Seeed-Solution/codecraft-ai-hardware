[English](README.md) · 简体中文

# Wio Tracker L2 原生开发目标

启用面向 ESP32-S3 的 ESP-IDF 6.0.2，在仓库根目录导出示例并使用锁定的源码构建。

```sh
python3 codecraft.py new --target esp-idf/wio-tracker-l2 --example wifi_scan --output projects/l2-scan
python3 codecraft.py build-project projects/l2-scan
```

示例初始化 Wi-Fi STA，被动扫描一次，通过 USB Serial/JTAG 输出接入点数量并释放 STA。零是有效结果。示例使用 ESP-IDF 默认分区表，刷入会替换当前应用，也可能覆盖已有固件数据。运行 Meshtastic 固件的 L2 请参考[设备指南](../reference/meshtastic.md)。

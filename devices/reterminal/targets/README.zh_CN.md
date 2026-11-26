<p align="right"><a href="README.md">English</a> · <strong>简体中文</strong></p>

# linux/reterminal

设备知识：[AGENTS.md](../AGENTS.md)。硬件事实在该入口及其来源中维护。

在仓库根目录构建/检查：`codecraft validate --target linux/reterminal`。

示例、原生接口引用和工具链选择见 `target.json`。构建结果与设备观测分别记录。

## 原生实现

### 目录与运行环境

- `overlays/reterminal-peripherals-overlay.dts` 提供使用主线驱动的按键、LED、蜂鸣器、光照、加速度、RTC 和电源键 overlay，基于厂商事实以 Apache-2.0 实现，可用独立 dtc 编译。安装步骤及完整厂商 overlay 的使用条件见 `overlays/README.md`。
- `buttons_evdev.py` 读取 evdev 按键；`leds_buzzer_sysfs.py` 控制三个 LED 和蜂鸣器并在退出时关闭；`sensors_sysfs.py` 读取 IIO 光照/加速度，三者仅需 Python 标准库。
- `header_gpio_blink.py` 使用 libgpiod 2.x 控制 40 针排针 BCM4（物理 7），展示 chip/line 寻址。
- 使用 64 位 Raspberry Pi OS（bookworm 或更新）、Python 3，排针示例需要 gpiod 2.x。实机证据必须记录 `uname -a` 与 `cat /etc/os-release` 的实际系统版本；当前没有绑定本次源码的硬件验证结果。

### 启动后检查

```sh
ls /sys/class/leds/            # usr_led0 usr_led1 usr_led2 usr_buzzer
cat /sys/class/input/event*/device/name | grep gpio-keys
cat /sys/bus/iio/devices/iio:device*/name   # ltr303, lis331dlh
```

## 工具链

主机检查使用 Python 3 与 dtc。设备 OS/kernel 和可选 gpiod 版本必须在目标设备记录，观察前保持未知。

导出应用使用 `codecraft build-project .`；仓库级验证命令不能替代应用锁文件检查。

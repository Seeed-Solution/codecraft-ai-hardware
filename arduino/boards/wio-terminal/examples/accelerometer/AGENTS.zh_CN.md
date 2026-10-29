<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# accelerometer — Agent 说明

## 目的

不依赖外部传感器库，直接通过 `Wire1` 读取板载 ST LIS3DHTR。草图用于验证
总线、地址、`WHO_AM_I` 值与基本加速度换算。

## 硬件事实

- LIS3DHTR：`Wire1`，SDA GPIO 78、SCL GPIO 77，7 位地址 `0x18`。INT1
  为 GPIO 79，但本示例未使用。来源：原理图 v1.2 第 07 页与核心 1.8.6
  `variant.h` 给出总线/引脚；地址 `0x18` 是 Seeed_Arduino_LIS3DHTR 的
  `LIS3DHTR_DEFAULT_ADDRESS`（`0x19` 是库中的备用
  `LIS3DHTR_ADDRESS_UPDATED`）。参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 草图将 `CTRL_REG1` 设为 `0x57`：100 Hz 输出率、普通模式、X/Y/Z 使能。
  `CTRL_REG4` 的 `0x80` 启用块数据更新。默认量程为 +/-2 g，因此换算为
  `raw / 16000` g。
- `WHO_AM_I` 寄存器 `0x0F` 必须读到 `0x33`。
- `readRegisters()` 在寄存器字节上置 bit 7（`reg | 0x80`），使多字节读
  自动递增；这与 Seeed 的 `readRegisterRegion()` 实现一致。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/accelerometer
```

不需要外部库。授权烧录与工具链设置见板卡
[AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`LIS3DHTR ACCELEROMETER`。
- 探测成功后串口打印 `LIS3DHTR online at 0x18 on Wire1`。
- 每秒约 10 次输出 `x=... y=... z=...`，单位为 g。
- LCD 显示 X/Y/Z、`|a|`，以及方框内的 X/Y 圆点。
- 板卡静止时模值应接近 1 g；倾斜时各轴变化且圆点移动。
- `setRotation(3)` 下实测屏幕映射为 `displayX = -yg`、`displayY = xg`，
  Z 不变。LIS3DHTR 的 Y 轴指向屏幕横向，X 轴指向屏幕纵向；180 度屏幕
  旋转又会反转横向方向。

## 上机验证

1. 获得明确授权后构建并上传，以 115200 baud 打开串口。
2. 将板卡水平静止放置，确认在线消息与接近 1 g 的稳定模值。
3. 分别绕三个轴倾斜，确认圆点按实测映射 `displayX = -yg`、
   `displayY = xg` 向低处滚动。
4. 若反复出现 `NOT FOUND at 0x18`，检查 `Wire1` 与地址；不要改接 Grove
   `Wire`（GPIO 46/45）。

2026-10-28 已实测：`0x18` 探测成功，`WHO_AM_I` 为 `0x33`。屏幕映射在
硬件上最终修正为 `displayX = -yg`、`displayY = xg`，倾斜板卡时圆点按
正确方向向低处滚动。用户已确认最终效果。

## 限制

- 10 Hz 轮询，不使用 INT1 与 FIFO。
- 固定 +/-2 g 量程与 100 Hz 传感器输出率；修改任一项都必须同步调整配置
  与换算。
- 未做零偏、温度或绝对精度校准。

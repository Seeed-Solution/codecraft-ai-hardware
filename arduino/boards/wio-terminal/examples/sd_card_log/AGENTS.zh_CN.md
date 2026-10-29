<p align="right">
  <a href="AGENTS.md">English</a> · <strong>简体中文</strong>
</p>

# sd_card_log — Agent 说明

## 目的

挂载独立 microSD 插槽，列出根目录并向 `/wio_terminal.log` 追加一行。它验证
SD 卡引脚映射与文件系统基本流程，不占用 LCD 总线。

## 硬件事实

- microSD 插座 ST-TF-003J 使用 SPI2：SCK 63、MOSI 62、MISO 61、CS 64；
  卡检测为 GPIO 65。来源：原理图 v1.2 第 02 页与核心 1.8.6
  `variants/wio_terminal/variant.h`（`SDCARD_*`）；参见
  [`../../README.zh_CN.md`](../../README.zh_CN.md)。
- 产品页标注最大支持 16 GB；草图期望 FAT32 格式。
- 草图使用 Seeed Arduino FS 2.1.5，并调用
  `SD.begin(SDCARD_SS_PIN, SDCARD_SPI, 4000000UL)`，显式指定 SPI2 与片选
  GPIO 64。不要替换为标准 Arduino `SD` 库：其无参 `begin()` 会使用 `SS`
  （GPIO 50），那不是 microSD 的片选。
- 在 Seeed Arduino FS 2.1.5 中，`FILE_WRITE` 展开为
  `FA_CREATE_ALWAYS | FA_WRITE | FA_READ`，会截断已有文件；追加必须使用
  `FILE_APPEND`。草图会重新打开日志，打印字节数与最后一行作为回读验证。

## 构建

在仓库根目录执行：

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/wio-terminal/fqbn.txt)" \
  arduino/boards/wio-terminal/examples/sd_card_log
```

所需外部库是 `libraries.txt` 固定版本的 `Seeed Arduino FS@2.1.5`。授权烧录
与设置见板卡 [AGENTS.zh_CN.md](../../AGENTS.zh_CN.md)。

## 预期行为

- LCD 标题：`MICROSD CARD`；显示 SPI2 引脚映射与状态。
- 串口首先输出 `sd_card_log boot`。草图最多等待 5 秒，让主机打开 CDC
  串口，避免错过只发送一次的挂载与根目录日志。
- 挂载成功后串口打印 `SD mounted`、最多六个根目录条目、
  `appended /wio_terminal.log`，随后输出回读所得的
  `log_bytes=... last_line=...`。根目录只显示文件名，因为库的
  `File::name()` 对根目录条目返回内部 `0://...` 路径。
- LCD 状态变绿并显示相同根目录条目；底部显示日志路径。
- 无卡或无法读取时，每两秒打印 `SD mount failed; retrying` 并继续重试。
  挂载失败后会调用 `SD.end()`，避免失败尝试累积 FatFs 驱动实例。

## 上机验证

1. 将 16 GB 或更小的卡格式化为 FAT32，并在上电前插入。
2. 获得明确授权后构建并上传，以 115200 baud 打开串口。
3. 确认出现 `SD mounted`、根目录条目、日志追加消息，以及回读的
   `log_bytes=` / `last_line=` 数值。
4. 取出卡并在电脑上确认 `/wio_terminal.log` 含挂载记录。
5. 复位后确认 `log_bytes` 增长且 `last_line` 指向新的挂载记录；同时确认
   LCD 仍正常显示且未受 SD 总线影响。

2026-10-28 已用一张已插入的 FAT32 卡实测：板卡列出 6 个根目录文件，追加
`/wio_terminal.log`，回读结果为
`log_bytes=98 last_line=wio-terminal sd_card_log mounted at millis=3494`。
第二次复位后回读增长到 `log_bytes=147`，最后一行为新的 `millis=3714`
记录，证明实际采用了追加语义。用户已确认 LCD 显示正常。

## 限制

- 草图只挂载一次，不处理热插拔/重新挂载；失败会重试，但成功后再拔卡不会
  被检测。
- 卡检测 GPIO 65 已记录，但示例未使用。
- 根目录最多显示六项，且不是文件复制示例。
- 未测量本次测试卡的容量与底层文件系统变体；示例只证明该卡可挂载、日志可
  追加并可回读。容量仍受文档所述 16 GB 支持上限约束。

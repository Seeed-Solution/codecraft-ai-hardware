---
name: codecraft-build
description: 为 codecraft-ai-hardware 的板卡构建并验证固件/示例，含静态检查与经 tools/validate.sh 的逐板 ESP-IDF 或 arduino-cli 构建。用于构建与测试请求；不烧录、不提交、不发布。
---

<p align="right"><strong>简体中文</strong> · <a href="SKILL.md">English</a></p>

# 构建

1. 先执行 `git status --short --branch`；绝不清理无关文件。
2. 激活板卡 README 声明的 ESP-IDF 版本。不要凭个人经验推断 IDF 路径；
   未激活时先询问。
3. 先跑最小相关检查，再跑门禁：

   ```bash
   ./tools/validate.sh --static
   ./tools/validate.sh --esp32 <board>
   ./tools/validate.sh --arduino <board>
   ```

4. 构建通过不等于硬件验证通过——报告中必须写明这一点。
5. 失败时切换到 `codecraft-debug`，不要盲目重试。

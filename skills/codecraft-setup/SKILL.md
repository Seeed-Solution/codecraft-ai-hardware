---
name: codecraft-setup
description: Bootstrap toolchains for codecraft-ai-hardware — ESP-IDF at the version each board requires, host test dependencies, and Linux board prerequisites. Use for environment bootstrap or a missing toolchain.
---

# Setup

1. Identify the target platform and board; read its README for the required
   ESP-IDF version (versions differ per board — reSpeaker Lite requires
   v6.0.1).
2. If ESP-IDF is missing, install it using the board's declared version and
   the official Espressif install flow for the host OS. Obtain required
   network/system permissions rather than bypassing them.
3. Verify: `. $IDF_PATH/export.sh` (or the platform equivalent) and
   `idf.py --version` must report the board's required version.
4. For host tests, ensure a C compiler and Python 3 are present.
5. For Linux boards, install libgpiod and the Python bindings; document any
   udev rules or group membership.
6. Report exactly what was installed and what remains missing.

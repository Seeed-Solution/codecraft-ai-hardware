---
name: codecraft-debug
description: Diagnose build, flash, and on-device failures in codecraft-ai-hardware. Use when a build fails, a flash fails, or hardware behavior contradicts the code.
---

# Debug

1. Reproduce with the smallest command; capture the complete error text.
2. Classify: environment (wrong IDF version), configuration (sdkconfig,
   partition table), code, wiring/hardware, or a hardware-fact error.
3. For suspected hardware-fact errors, check the fact priority chain from
   the root `AGENTS.md` — vendor spec and measured results outrank README
   and demo code. Record any correction in the board README.
4. Change one variable at a time; re-run the failing check after each change.
5. Do not flash experimental fixes to a device without the authorization
   required by `codecraft-device-test`.

---
name: codecraft-build
description: Build and validate firmware/examples for a board in codecraft-ai-hardware, including static checks and per-board ESP-IDF or arduino-cli builds via tools/validate.sh. Use for build and test requests; does not flash, commit, or publish.
---

# Build

1. Run `git status --short --branch`; never clean unrelated files.
2. Activate the ESP-IDF version the board declares in its README. Do not
   infer a developer-specific IDF path; ask if none is activated.
3. Run the smallest relevant check first, then the gate:

   ```bash
   ./tools/validate.sh --static
   ./tools/validate.sh --esp32 <board>
   ./tools/validate.sh --arduino <board>
   ```

4. A clean build is not hardware validation — say so in the report.
5. On failure, switch to `codecraft-debug` rather than retrying blindly.

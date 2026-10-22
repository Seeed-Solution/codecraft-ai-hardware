---
name: codecraft-develop
description: Development workflow for the codecraft-ai-hardware repository — task routing, hardware-fact priority, BSP/application boundary, and delivery format. Use for any code or documentation change; does not build, flash, commit, or publish.
---

# Develop

1. Run `git status --short --branch`; preserve existing user changes.
2. Read the root `AGENTS.md` routing table entry for the task, plus
   `docs/development/ai-guide.md`.
3. For board work, read the board README and its row in
   `docs/vendor-sources.md` before editing. Clone and search the vendor
   source; never guess a hardware fact.
4. Place code at the right layer: board facts and reusable drivers in the
   board BSP, shared silicon drivers in `esp32/components/`, demos and
   application logic in `examples/` or the app.
5. Keep testable logic independent from ESP-IDF/LVGL and add host tests in
   `tests/`.
6. Deliver with the report format from `docs/development/ai-guide.md`:
   static checks, per-board build result, on-device verification (or an
   explicit statement it was not performed).

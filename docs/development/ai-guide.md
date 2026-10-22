# AI Development Guide

The detailed workflow for AI-assisted development in this repository. The
root `AGENTS.md` is the mandatory entry point; this guide is the long form.

## Context setup

1. Read the root `AGENTS.md` routing table entry for your task.
2. Read the board README for the target board and the matching row in
   `docs/vendor-sources.md`.
3. Before inventing any hardware detail, check the vendor source. Clone it
   and search it; do not rely on web summaries.
4. Confirm the five required skills are available; use only the ones matching
   the task.

## Application / BSP boundary

- Board facts and reusable drivers belong in the board's BSP
  (`esp32/boards/<board>/bsp/` or a shared component in
  `esp32/components/` when two or more boards use it).
- Demos, pages, state machines and application tasks live in the board's
  `examples/` directory or the application being built.
- Keep testable logic (state machines, protocols, timing, layout math)
  independent from ESP-IDF/LVGL and cover it with host tests in `tests/`.

## Runtime invariants (ESP32)

- LVGL is not thread-safe. Code outside the LVGL task must hold the board
  BSP's LVGL lock while accessing LVGL objects.
- Button callbacks must stay non-blocking. Audio, storage, networking, and
  other slow operations belong in worker tasks.
- A demo must stop every task, timer, callback, and event handler that can
  access its UI before deleting the screen.
- Keep the default partition table minimal unless the product requires
  otherwise; validate any custom table.

## Adding a new board

1. Add the vendor source row to `docs/vendor-sources.md` first.
2. Gather facts with citations: chip and module (target, flash size, PSRAM
   size and mode), display, touch, audio path, power chip, buttons/LEDs,
   USB-UART path, and the required ESP-IDF version.
3. Create `esp32/boards/<board>/` with `sdkconfig.defaults`, `bsp/`,
   `examples/`, and a README recording the facts and their sources.
4. Register the board in `tools/validate.sh` and the root README table.
5. Add the board's IDF version to the CI matrix when CI lands.

## Delivery format

Every final report states separately:

1. Static check result (`tools/validate.sh --static`).
2. Build result per affected board (`tools/validate.sh --esp32 <board>`).
3. On-device verification result, or an explicit statement that it was not
   performed.

A successful build is not hardware validation.

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
   USB-UART path, and the required toolchain version (ESP-IDF version for
   `esp32/`; core and library versions for `arduino/`).
3. Create the board directory under the matching platform —
   `esp32/boards/<board>/`, `linux/boards/<board>/`, or
   `arduino/boards/<board>/` — following that platform's `AGENTS.md`
   directory contract.
4. Register the board in `tools/validate.sh` and the root README table.
5. Register the board in CI: ESP32 boards join the build matrix; Arduino
   boards are discovered automatically from `arduino/boards/`.

## Delivery format

Every final report states separately:

1. Static check result (`tools/validate.sh --static`).
2. Build result per affected board (`tools/validate.sh --esp32 <board>`).
3. On-device verification result, or an explicit statement that it was not
   performed.

A successful build is not hardware validation.

Commits use an English imperative Conventional Commit subject
(`type(scope): description`); keep each commit independently reviewable and
exclude credentials, generated firmware, and unrelated files. Creating a
commit does not authorize pushing, opening a PR, or merging.

Pull requests fill in `.github/PULL_REQUEST_TEMPLATE.md` and follow
`.github/CONTRIBUTING.md`. Report static checks, per-board builds, and
on-device verification separately in the PR body; list unperformed hardware
work under `Unverified`.

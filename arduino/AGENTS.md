<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

# Arduino Platform Rules

Applies to everything under `arduino/`. The root `AGENTS.md` governs first;
this file adds platform-specific rules.

## Scope

Boards developed with the Arduino framework (arduino-cli / Arduino IDE),
regardless of silicon: SAMD (e.g. Wio Terminal), AVR (e.g. the Grove
Beginner Kit's Seeeduino Lotus), ESP32 in Arduino mode, and similar. A
board developed with ESP-IDF belongs in `esp32/`, even when the same
hardware could also run Arduino.

## Board directory contract

Each `arduino/boards/<board>/` directory contains:

- `fqbn.txt` — one line: the arduino-cli fully qualified board name, e.g.
  `Seeeduino:samd:seeed_wio_terminal`.
- `core-version.txt` — one line: the exact core used for verification, as
  `ID@version` matching the FQBN core, e.g. `Seeeduino:samd@1.8.6`.
- `libraries.txt` — Arduino Library Manager pins, one exact
  `Name@x.y.z` per line; comments and blank lines are allowed. The LCD
  library bundled inside a vendor core is noted in a comment, not
  re-installed.
- `README.md` and `README.zh_CN.md` — verified hardware facts with citations,
  the board-manager index URL when the core is not built in, the capability
  matrix, limitations/conflicts, exact toolchain versions, and an example
  list that matches the directories exactly.
- `AGENTS.md` and `AGENTS.zh_CN.md` — board-specific agent instructions:
  build/upload rules, bus ownership, firmware prerequisites, known compile
  pitfalls, and the on-device verification contract.
- `examples/<name>/` — standalone sketches, with `<name>.ino`,
  `AGENTS.md` and `AGENTS.zh_CN.md`. Each agent pair states the purpose,
  hardware facts with sources, exact build command, expected LCD/serial
  behavior, physical test steps and limitations.

## Onboarding a board

1. Add the vendor source row to `docs/vendor-sources.md` first.
2. Create the directory contract above; record every hardware fact with its
   source, exact core, and exact library versions.
3. Register the board in the root README table and `docs/inventory.md`.
4. Verify with `./tools/validate.sh --static` and
   `./tools/validate.sh --arduino <board>`. CI discovers every board under
   `arduino/boards/` automatically once the directory lands.

## Rules

- Same hardware-fact priority as the other platforms (root `AGENTS.md`):
  never guess pins.
- Prefer core-defined pin macros (e.g. `WIO_KEY_A` from the vendor core's
  `variant.h`) over raw numbers, and cite the vendor source anyway.
- Pin the core version and every external library version used for
  verification in the board README.
- Keep sketches standalone: no committed `libraries/` folder, no shared
  sketchbook.
- Non-Seeed cores needing a different board-manager index URL: document
  the URL in the board README and add it to
  `.github/workflows/arduino-build.yml`.
- Build with arduino-cli:
  `arduino-cli compile --fqbn "$(cat fqbn.txt)" examples/<sketch>` from the
  board directory, or use the repository-relative path from the root.
- Do not flash or update a wireless coprocessor without explicit user
  authorization. A successful compile is not hardware validation.

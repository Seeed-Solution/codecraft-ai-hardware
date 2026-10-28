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
- `README.md` — verified hardware facts with citations, the Arduino core
  (name + version used for verification), the board-manager index URL when
  the core is not built in, external library dependencies pinned to exact
  versions, and known conflicts between sources.
- `examples/` — standalone sketches; one directory per sketch, with the
  `.ino` file named after its directory.

## Onboarding a board

1. Add the vendor source row to `docs/vendor-sources.md` first.
2. Create the directory contract above; record every hardware fact with
   its source.
3. Register the board in the root README table and `docs/inventory.md`.
4. Verify with `./tools/validate.sh --arduino <board>`. CI discovers every
   board under `arduino/boards/` automatically once the directory lands.

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
  `arduino-cli compile --fqbn "$(cat fqbn.txt)" examples/<sketch>`.

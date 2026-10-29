#!/usr/bin/env bash
# Validation gate for codecraft-ai-hardware. See AGENTS.md.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

ESP32_BOARDS=()
if [[ -d esp32/boards ]]; then
  while IFS= read -r d; do ESP32_BOARDS+=("$(basename "$d")"); done < <(find esp32/boards -mindepth 1 -maxdepth 1 -type d | sort)
fi

ARDUINO_BOARDS=()
if [[ -d arduino/boards ]]; then
  while IFS= read -r d; do ARDUINO_BOARDS+=("$(basename "$d")"); done < <(find arduino/boards -mindepth 1 -maxdepth 1 -type d | sort)
fi

usage() {
  cat <<'USAGE'
Usage: tools/validate.sh [--static] [--esp32 <board>] [--linux <board>] [--arduino <board>]

  --static          repository checks + host tests
  --esp32 <board>   ESP-IDF build for one board (requires activated IDF)
  --linux <board>   Linux board checks
  --arduino <board> arduino-cli compile for one board (requires arduino-cli + the board's core)
  (no arguments)    complete gate: static + every board with firmware
USAGE
}

fail() { echo "FAIL: $*" >&2; exit 1; }
note() { echo "note: $*"; }

check_static() {
  echo "== static checks =="
  local rc=0

  # Bilingual baseline: root README, AGENTS, and CLAUDE pairs.
  for base in README AGENTS CLAUDE; do
    [[ -f "$base.md" ]] || { echo "  MISSING $base.md"; rc=1; }
    [[ -f "$base.zh_CN.md" ]] || { echo "  MISSING $base.zh_CN.md"; rc=1; }
  done
  [[ -f NOTICE ]] || { echo "  MISSING NOTICE"; rc=1; }

  # Skills contract: every skill has a bilingual SKILL pair; the index and
  # validation checklist are bilingual too.
  if [[ -d skills ]]; then
    [[ -f skills/README.md && -f skills/README.zh_CN.md ]] || { echo "  MISSING skills/README(.zh_CN).md"; rc=1; }
    [[ -f skills/validation.md && -f skills/validation.zh_CN.md ]] || { echo "  MISSING skills/validation(.zh_CN).md"; rc=1; }
    while IFS= read -r d; do
      [[ -f "$d/SKILL.md" ]] || { echo "  MISSING $d/SKILL.md"; rc=1; }
      [[ -f "$d/SKILL.zh_CN.md" ]] || { echo "  MISSING $d/SKILL.zh_CN.md"; rc=1; }
    done < <(find skills -mindepth 1 -maxdepth 1 -type d | sort)
  fi

  # Board directory contract: every board has a README.
  for b in "${ESP32_BOARDS[@]}"; do
    [[ -f "esp32/boards/$b/README.md" ]] || { echo "  MISSING esp32/boards/$b/README.md"; rc=1; }
  done
  if [[ -d linux/boards ]]; then
    while IFS= read -r d; do
      [[ -f "$d/README.md" ]] || { echo "  MISSING $d/README.md"; rc=1; }
    done < <(find linux/boards -mindepth 1 -maxdepth 1 -type d | sort)
  fi

  # Arduino board directory contract: bilingual board docs, an FQBN, the
  # pinned core/library metadata, and one reproducible AGENTS pair per sketch.
  if [[ -d arduino/boards ]]; then
    while IFS= read -r d; do
      for f in README.md README.zh_CN.md AGENTS.md AGENTS.zh_CN.md fqbn.txt core-version.txt libraries.txt; do
        [[ -s "$d/$f" ]] || { echo "  MISSING or empty $d/$f"; rc=1; }
      done
      if [[ -f "$d/fqbn.txt" ]]; then
        fqbn_lines="$(grep -cve '^[[:space:]]*$' "$d/fqbn.txt" || true)"
        [[ "$fqbn_lines" -eq 1 ]] || { echo "  $d/fqbn.txt must contain exactly one non-empty line"; rc=1; }
      fi
      while IFS= read -r ex; do
        name="$(basename "$ex")"
        [[ -f "$ex/$name.ino" ]] || { echo "  MISSING $ex/$name.ino"; rc=1; }
        [[ -s "$ex/AGENTS.md" ]] || { echo "  MISSING or empty $ex/AGENTS.md"; rc=1; }
        [[ -s "$ex/AGENTS.zh_CN.md" ]] || { echo "  MISSING or empty $ex/AGENTS.zh_CN.md"; rc=1; }
      done < <(find "$d/examples" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort)
    done < <(find arduino/boards -mindepth 1 -maxdepth 1 -type d | sort)
  fi

  # Secret scan (tracked files, common key/token forms).
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    if git grep -nI -E 'BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY' -- . >/dev/null 2>&1; then
      echo "  private key material found in tracked files"; rc=1
    fi
  fi

  # Host tests (if any are present).
  if compgen -G "tests/test_*.py" >/dev/null 2>&1; then
    if python3 -c "import pytest" >/dev/null 2>&1; then
      python3 -m pytest tests/ || rc=1
    else
      note "pytest not installed; using the zero-dependency fallback runner"
      python3 tests/run_tests.py || rc=1
    fi
  else
    note "no host tests yet"
  fi

  [[ $rc -eq 0 ]] && echo "static checks: PASS" || fail "static checks"
}

check_esp32_board() {
  local board="$1"
  local dir="esp32/boards/$board"
  echo "== esp32 board: $board =="
  [[ -d "$dir" ]] || fail "unknown ESP32 board '$board' (have: ${ESP32_BOARDS[*]:-none})"

  local examples=()
  while IFS= read -r d; do examples+=("$d"); done < <(find "$dir/examples" -mindepth 1 -maxdepth 1 -type d 2>/dev/null | sort)
  if [[ ${#examples[@]} -eq 0 ]]; then
    note "$board has no firmware examples yet — build skipped (not a failure)"
    return 0
  fi
  command -v idf.py >/dev/null 2>&1 || fail "ESP-IDF not activated; see $dir/README.md for the required version"
  for ex in "${examples[@]}"; do
    [[ -f "$ex/CMakeLists.txt" ]] || continue
    echo "  building $ex"
    ( cd "$ex" && idf.py build )
  done
  echo "$board build: PASS"
}

check_linux_board() {
  local board="$1"
  local dir="linux/boards/$board"
  echo "== linux board: $board =="
  [[ -d "$dir" ]] || fail "unknown Linux board '$board'"
  [[ -f "$dir/README.md" ]] || fail "$dir/README.md missing"

  # Python demos must at least parse.
  local rc=0 f
  while IFS= read -r f; do
    python3 -m py_compile "$f" || { echo "  FAIL py_compile $f"; rc=1; }
  done < <(find "$dir" -name "*.py" -not -path "*/build/*" 2>/dev/null | sort)

  # Overlays must compile when dtc is available (plain-constant overlays
  # need no kernel headers).
  if command -v dtc >/dev/null 2>&1; then
    while IFS= read -r f; do
      dtc -@ -I dts -O dtb -o /dev/null "$f" >/dev/null 2>&1 || { echo "  FAIL dtc $f"; rc=1; }
    done < <(find "$dir" -name "*.dts" 2>/dev/null | sort)
  else
    note "$board: dtc not found — overlay compile check skipped"
  fi

  [[ $rc -eq 0 ]] && echo "$board linux checks: PASS" || fail "$board linux checks"
}

check_arduino_board() {
  local board="$1"
  local dir="arduino/boards/$board"
  echo "== arduino board: $board =="
  [[ -d "$dir" ]] || fail "unknown Arduino board '$board' (have: ${ARDUINO_BOARDS[*]:-none})"
  [[ -f "$dir/README.md" ]] || fail "$dir/README.md missing"
  [[ -f "$dir/fqbn.txt" ]] || fail "$dir/fqbn.txt missing (one line: the arduino-cli FQBN, e.g. Seeeduino:samd:seeed_wio_terminal)"
  local fqbn
  fqbn="$(head -n1 "$dir/fqbn.txt" | tr -d '[:space:]')"
  [[ -n "$fqbn" ]] || fail "$dir/fqbn.txt is empty"

  local sketches=()
  while IFS= read -r f; do sketches+=("$f"); done < <(find "$dir/examples" -mindepth 2 -maxdepth 2 -name "*.ino" 2>/dev/null | sort)
  if [[ ${#sketches[@]} -eq 0 ]]; then
    note "$board has no sketches yet — compile skipped (not a failure)"
    return 0
  fi
  command -v arduino-cli >/dev/null 2>&1 || fail "arduino-cli not found; see $dir/README.md for the required core and index URL"
  for sk in "${sketches[@]}"; do
    echo "  compiling $sk ($fqbn)"
    arduino-cli compile --fqbn "$fqbn" "$(dirname "$sk")"
  done
  echo "$board compile: PASS"
}

mode="${1:-}"
case "$mode" in
  --static)
    check_static
    ;;
  --esp32)
    [[ $# -eq 2 ]] || { usage; exit 2; }
    check_esp32_board "$2"
    ;;
  --linux)
    [[ $# -eq 2 ]] || { usage; exit 2; }
    check_linux_board "$2"
    ;;
  --arduino)
    [[ $# -eq 2 ]] || { usage; exit 2; }
    check_arduino_board "$2"
    ;;
  "" )
    check_static
    for b in "${ESP32_BOARDS[@]}"; do check_esp32_board "$b"; done
    if [[ -d linux/boards ]]; then
      while IFS= read -r d; do
        check_linux_board "$(basename "$d")"
      done < <(find linux/boards -mindepth 1 -maxdepth 1 -type d | sort)
    fi
    if [[ ${#ARDUINO_BOARDS[@]} -gt 0 ]]; then
      for b in "${ARDUINO_BOARDS[@]}"; do check_arduino_board "$b"; done
    fi
    ;;
  -h|--help)
    usage
    ;;
  *)
    usage; exit 2
    ;;
esac

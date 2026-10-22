"""Host-side consistency checks between board READMEs and the filesystem.

These guard against the two drift bugs this repository is most prone to:
a README table listing a BSP header or example that does not exist, and a
pin header losing its license block or gaining a duplicate define.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ESP32_BOARDS = [
    "sensecap-watcher",
    "sensecap-indicator",
    "reterminal-e1001",
    "reterminal-e1002",
    "respeaker-lite",
]


def test_readme_bsp_headers_exist():
    for board in ESP32_BOARDS:
        readme = (ROOT / "esp32/boards" / board / "README.md").read_text()
        listed = set(re.findall(r"`(bsp_[a-z0-9_]+\.h)`", readme))
        assert listed, f"{board}: README lists no bsp headers"
        for header in sorted(listed):
            path = ROOT / "esp32/boards" / board / "bsp/include" / header
            assert path.is_file(), f"{board}: README lists {header} but {path} is missing"


def test_readme_examples_exist():
    for board in ESP32_BOARDS:
        readme = (ROOT / "esp32/boards" / board / "README.md").read_text()
        section = readme.split("## Examples", 1)
        assert len(section) == 2, f"{board}: README has no Examples section"
        listed = set(re.findall(r"^- `([a-z0-9_]+)`", section[1], flags=re.M))
        assert listed, f"{board}: README lists no examples"
        for name in sorted(listed):
            path = ROOT / "esp32/boards" / board / "examples" / name
            assert path.is_dir(), f"{board}: README lists example {name} but {path} is missing"


def test_pin_headers_have_license_and_unique_defines():
    for board in ESP32_BOARDS:
        header = ROOT / "esp32/boards" / board / "bsp/include/bsp_pins.h"
        assert header.is_file(), f"{board}: bsp_pins.h missing"
        text = header.read_text()
        assert "SPDX-License-Identifier: Apache-2.0" in text, (
            f"{board}: bsp_pins.h lost its license header"
        )
        names = re.findall(r"^#define\s+(BSP_[A-Z0-9_]+)\b", text, flags=re.M)
        dupes = {n for n in names if names.count(n) > 1}
        assert not dupes, f"{board}: duplicate defines in bsp_pins.h: {sorted(dupes)}"

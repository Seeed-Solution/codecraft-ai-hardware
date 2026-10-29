"""Host-side checks for the Arduino board contract under arduino/boards.

The validation gate already verifies the shell-level directory contract.
These tests protect the AI-facing parts that are easy to drift: missing
bilingual notes, an example omitted from both READMEs, an unpinned core, or a
sketch whose hardware-fact citations disappeared.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BOARD_ROOT = ROOT / "arduino" / "boards"


def boards():
    if not BOARD_ROOT.is_dir():
        return []
    return sorted(path for path in BOARD_ROOT.iterdir() if path.is_dir())


def examples(board):
    path = board / "examples"
    if not path.is_dir():
        return []
    return sorted(item for item in path.iterdir() if item.is_dir())


def readme_examples(path):
    text = path.read_text()
    match = re.search(r"^## (?:Examples|示例)\s*$", text, flags=re.M)
    assert match, f"{path}: missing Examples section"
    section = text[match.end() :]
    next_heading = re.search(r"^## ", section, flags=re.M)
    if next_heading:
        section = section[: next_heading.start()]
    return set(re.findall(r"^- `([^`]+)`", section, flags=re.M))


def test_board_metadata_is_bilingual_and_complete():
    for board in boards():
        for name in (
            "README.md",
            "README.zh_CN.md",
            "AGENTS.md",
            "AGENTS.zh_CN.md",
            "fqbn.txt",
            "core-version.txt",
            "libraries.txt",
        ):
            path = board / name
            assert path.is_file(), f"{board}: missing {name}"
            assert path.read_text().strip(), f"{board}: empty {name}"

        fqbn = board.joinpath("fqbn.txt").read_text().strip()
        assert re.fullmatch(
            r"[^:\s]+:[^:\s]+:[^:\s]+(?::[^:\s=]+=[^:\s]+)*", fqbn
        ), f"{board}: malformed FQBN {fqbn!r}"
        fqbn_core = ":".join(fqbn.split(":")[:2])
        core = board.joinpath("core-version.txt").read_text().strip()
        core_id, separator, version = core.partition("@")
        assert separator and version, f"{board}: core-version.txt must be ID@version"
        assert core_id == fqbn_core, (
            f"{board}: core-version {core_id} does not match FQBN core"
        )


def test_all_board_markdown_is_paired():
    for board in boards():
        for path in sorted(board.rglob("*.md")):
            if path.name.endswith(".zh_CN.md"):
                pair = path.with_name(path.name[: -len(".zh_CN.md")] + ".md")
            else:
                pair = path.with_name(path.stem + ".zh_CN.md")
            assert pair.is_file(), f"{path}: missing bilingual pair {pair.name}"


def test_every_example_has_sketch_and_agents_pair():
    for board in boards():
        for example in examples(board):
            sketch = example / (example.name + ".ino")
            assert sketch.is_file(), f"{example}: missing {sketch.name}"
            for name in ("AGENTS.md", "AGENTS.zh_CN.md"):
                path = example / name
                assert path.is_file() and path.read_text().strip(), (
                    f"{example}: missing or empty {name}"
                )
            assert "Source" in sketch.read_text(), (
                f"{sketch}: no Source citation for its hardware facts"
            )


def test_both_readmes_list_exactly_the_example_directories():
    for board in boards():
        expected = {example.name for example in examples(board)}
        for name in ("README.md", "README.zh_CN.md"):
            listed = readme_examples(board / name)
            assert listed == expected, (
                f"{board / name}: example list drift; missing={sorted(expected - listed)}, "
                f"extra={sorted(listed - expected)}"
            )


def test_library_pins_are_exact_and_unique():
    pattern = re.compile(r"^[^#].+@\d+\.\d+\.\d+$")
    for board in boards():
        libraries = []
        for raw in board.joinpath("libraries.txt").read_text().splitlines():
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            assert pattern.match(line), f"{board}: library pin is not exact: {line!r}"
            libraries.append(line)
        names = [line.rsplit("@", 1)[0] for line in libraries]
        assert len(names) == len(set(names)), f"{board}: duplicate library pins"

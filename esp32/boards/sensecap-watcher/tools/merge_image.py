#!/usr/bin/env python3
"""Merge one SenseCAP Watcher example into the image CodeCraft Hub flashes.

Hub writes a single file at 0x0 when Muse is off. That file has to contain
the bootloader, partition table, and app from the same build. The app-only
.bin is linked at 0x10000 and does not boot on its own.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

EXPECTED_OFFSETS = {0x0, 0x8000, 0x10000}
OUTPUT_NAME = "merged-binary.bin"
MIN_SIZE = 0x10000


def parse_offset(value: str) -> int:
    try:
        return int(value, 16)
    except (TypeError, ValueError) as exc:
        raise SystemExit(f"invalid flash offset {value!r}") from exc


def validate_layout(flash_files: dict, chip: str) -> None:
    if chip != "esp32s3":
        raise SystemExit(f"expected chip esp32s3, got {chip!r}")
    if "application" in OUTPUT_NAME:
        raise SystemExit("merged image name must not contain 'application'")
    offsets = {parse_offset(str(key)) for key in flash_files}
    if offsets != EXPECTED_OFFSETS:
        formatted = ", ".join(hex(item) for item in sorted(offsets))
        raise SystemExit(
            "Watcher Hub image requires flash offsets 0x0, 0x8000, and 0x10000; "
            f"got {formatted}"
        )


def merge_example(example: Path, check_only: bool) -> Path:
    args_path = example / "build" / "flasher_args.json"
    if not args_path.is_file():
        raise SystemExit(f"missing {args_path}; run idf.py build first")
    data = json.loads(args_path.read_text())
    try:
        flash_files = data["flash_files"]
        chip = data["extra_esptool_args"]["chip"]
    except (KeyError, TypeError) as exc:
        raise SystemExit(f"{args_path} is missing flash_files or chip") from exc
    validate_layout(flash_files, chip)
    output = example / "build" / OUTPUT_NAME
    if check_only:
        return output
    # esptool reads flash_args directly. idf.py merge-bin also rebuilds, and a
    # stale CMake cache can fail that rebuild after the binaries already exist.
    subprocess.run(
        [
            sys.executable,
            "-m",
            "esptool",
            "--chip",
            chip,
            "merge-bin",
            "-o",
            OUTPUT_NAME,
            "@flash_args",
        ],
        cwd=example / "build",
        check=True,
    )
    if not output.is_file():
        raise SystemExit(f"esptool merge-bin did not write {output}")
    size = output.stat().st_size
    if size <= MIN_SIZE:
        raise SystemExit(
            f"{output} is {size} bytes; a merged image must extend past 0x10000"
        )
    print(f"{output} {size}")
    return output


def main(argv: list[str]) -> int:
    check_only = "--check-only" in argv
    paths = [arg for arg in argv[1:] if arg != "--check-only"]
    if len(paths) != 1:
        print(
            "usage: merge_image.py [--check-only] <example-dir>",
            file=sys.stderr,
        )
        return 2
    merge_example(Path(paths[0]).resolve(), check_only)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv))
    except subprocess.CalledProcessError as exc:
        raise SystemExit(exc.returncode) from exc

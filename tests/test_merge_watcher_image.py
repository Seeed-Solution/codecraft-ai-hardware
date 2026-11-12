"""Layout guard for the SenseCAP Watcher Hub image."""

import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCRIPT = ROOT / "esp32/boards/sensecap-watcher/tools/merge_image.py"


def write_args(directory: Path, offsets: list[int], chip: str = "esp32s3") -> None:
    build = directory / "build"
    build.mkdir()
    payload = {
        "flash_files": {hex(offset): f"part-{offset:x}.bin" for offset in offsets},
        "extra_esptool_args": {"chip": chip},
    }
    (build / "flasher_args.json").write_text(json.dumps(payload))


def run_check(directory: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(SCRIPT), "--check-only", str(directory)],
        capture_output=True,
        text=True,
        check=False,
    )


def _scratch(name: str) -> Path:
    return Path(tempfile.mkdtemp(prefix=f"watcher-merge-{name}-"))


def test_accepts_watcher_single_app_layout(tmp_path=None):
    directory = Path(tmp_path) if tmp_path is not None else _scratch("ok")
    write_args(directory, [0x0, 0x8000, 0x10000])
    result = run_check(directory)
    assert result.returncode == 0, result.stderr


def test_rejects_a_different_partition_layout(tmp_path=None):
    directory = Path(tmp_path) if tmp_path is not None else _scratch("bad-layout")
    write_args(directory, [0x0, 0x10000, 0x20000])
    result = run_check(directory)
    assert result.returncode != 0
    assert "0x0, 0x8000, and 0x10000" in result.stderr


def test_rejects_a_non_esp32s3_chip(tmp_path=None):
    directory = Path(tmp_path) if tmp_path is not None else _scratch("bad-chip")
    write_args(directory, [0x0, 0x8000, 0x10000], chip="esp32")
    result = run_check(directory)
    assert result.returncode != 0
    assert "esp32s3" in result.stderr

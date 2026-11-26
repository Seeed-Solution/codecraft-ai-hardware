"""Run the imported standalone clients against local fake services only."""
from pathlib import Path
import importlib.util
import py_compile
import shutil
import struct
import sys
import tempfile
import os
import json
import subprocess

ROOT = Path(__file__).resolve().parents[3]


def test_cloud_helpers_match_the_maintained_source():
    source = (ROOT/'skills/codecraft-capabilities-llm/scripts/device_api.py').read_bytes()
    for package in ('codecraft-capabilities-llm','codecraft-capabilities-asr','codecraft-capabilities-tts','codecraft-capabilities-image-generation'):
        assert (ROOT/'skills'/package/'scripts/device_api.py').read_bytes() == source, package


def test_installed_skill_inventory_rejects_extra_missing_and_symlink_files():
    records = [p.stem for p in (ROOT/'tools/skill_packages/packages').glob('*.json')]
    with tempfile.TemporaryDirectory() as folder:
        installed = Path(folder)/'skills'
        for sid in records:
            shutil.copytree(ROOT/'skills'/sid, installed/sid, ignore=shutil.ignore_patterns('__pycache__'))
        extra = installed/'codecraft-capabilities-asr/scripts/unregistered.py'
        extra.write_text('# Unregistered module\n')
        assert check_packages(installed).returncode != 0
        extra.unlink()
        entry = installed/'codecraft-capabilities-asr/scripts/transcribe.py'
        source = entry.read_bytes()
        entry.unlink()
        assert check_packages(installed).returncode != 0
        entry.write_bytes(source)
        extra.symlink_to(entry)
        run = check_packages(installed)
        assert run.returncode != 0 and 'symlink' in run.stdout.lower()


def test_cli_check_ignores_unpinned_installed_bytecode():
    records = [p.stem for p in (ROOT/'tools/skill_packages/packages').glob('*.json')]
    with tempfile.TemporaryDirectory() as folder:
        installed = Path(folder)/'skills'
        for sid in records:
            shutil.copytree(ROOT/'skills'/sid, installed/sid, ignore=shutil.ignore_patterns('__pycache__'))
        source = installed/'codecraft-capabilities-asr/scripts/device_api.py'
        poisoned = Path(folder)/'bytecode_source.py'
        poisoned.write_text("raise RuntimeError('unregistered bytecode executed')\n")
        cache = Path(importlib.util.cache_from_source(str(source)))
        cache.parent.mkdir(parents=True)
        py_compile.compile(str(poisoned), cfile=str(cache), doraise=True)
        data = bytearray(cache.read_bytes())
        data[8:16] = struct.pack('<II', int(source.stat().st_mtime), source.stat().st_size)
        cache.write_bytes(data)
        run = check_packages(installed)
        assert run.returncode == 0, run.stdout + run.stderr


def check_packages(installed):
    return subprocess.run([os.environ['CODECRAFT_TEST_CLI'], '--root', str(ROOT),
        'maintenance', 'check-cloud-skills', '--skills-root', str(installed)],
        capture_output=True, text=True, timeout=60)

"""Zero-dependency fallback test runner.

`tools/validate.sh --static` prefers pytest when it is installed and falls
back to this script otherwise. Tests are plain `test_*` functions in
`test_*.py` modules using bare asserts, so both runners work.
"""

import importlib.util
import sys
import traceback
from pathlib import Path

TESTS_DIR = Path(__file__).resolve().parent


def main() -> int:
    failures = 0
    total = 0
    for path in sorted(TESTS_DIR.glob("test_*.py")):
        spec = importlib.util.spec_from_file_location(path.stem, path)
        module = importlib.util.module_from_spec(spec)
        try:
            spec.loader.exec_module(module)
        except Exception:
            failures += 1
            print(f"ERROR collecting {path.name}")
            traceback.print_exc()
            continue
        for name in sorted(dir(module)):
            if not name.startswith("test_"):
                continue
            fn = getattr(module, name)
            if not callable(fn):
                continue
            total += 1
            try:
                fn()
                print(f"PASS {path.stem}::{name}")
            except Exception:
                failures += 1
                print(f"FAIL {path.stem}::{name}")
                traceback.print_exc()
    print(f"\n{total - failures}/{total} passed")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())

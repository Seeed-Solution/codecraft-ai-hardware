"""Full client-release acceptance belongs to the pinned Python host-test gate.

The CLI owns release logic. This test only copies fixtures and invokes the public
Rust command; candidate validation selects its explicit protocol/operation
registry, which excludes this outer release test and therefore cannot recurse.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]


def test_reviewed_cloud_candidate_executes_all_registered_clients_and_applies():
    executable = os.environ['CODECRAFT_TEST_CLI']
    with tempfile.TemporaryDirectory(prefix='codecraft-cloud-release-cli-') as folder:
        base = Path(folder)
        root = base / 'repo'
        root.mkdir()
        for name in ('Cargo.toml', 'Cargo.lock', 'tests/requirements.txt', 'tests/conftest.py'):
            dest = root / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, dest)
        for name in ('tools/cli', 'tools/skill_packages', 'skills', 'tests/skills',
                     'tests/tools/skill_packages', 'docs'):
            shutil.copytree(ROOT / name, root / name,
                            ignore=shutil.ignore_patterns('__pycache__', '*.pyc'))
        source = root / 'skills/codecraft-capabilities-asr/references/api.md'
        source.write_text(source.read_text() + '\nReviewed release fixture note.\n')
        version_file = base / 'versions.json'
        record = json.loads((root / 'tools/skill_packages/packages/codecraft-capabilities-asr.json').read_text())
        major, minor, patch = map(int, record['version'].split('.'))
        version_file.write_text(json.dumps({'codecraft-capabilities-asr': {
            'skill_version': f'{major}.{minor}.{patch + 1}'}}))
        original_import = (root / 'tools/skill_packages/import.json').read_bytes()
        candidate = base / 'candidate'

        def command(*args):
            run = subprocess.run([executable, '--root', str(root), 'maintenance',
                                  'skill-release', *map(str, args)],
                                 capture_output=True, text=True, timeout=180)
            assert run.returncode == 0, run.stdout + run.stderr
            return json.loads(run.stdout)['data']

        prepared = command('prepare', '--output', candidate, '--versions', version_file)
        assert prepared['packages']['codecraft-capabilities-asr']['classification'] == 'documentation'
        applied = command('apply', '--candidate', candidate,
                          '--review-sha256', prepared['review_sha256'])
        assert applied['status'] == 'passed' and applied['repository_changed']
        assert len(applied['checks']) >= 42
        assert all(row['status'] == 'passed' for row in applied['checks'])
        assert not any('test_release_cli.py' in row['name'] for row in applied['checks'])
        assert (root / 'tools/skill_packages/import.json').read_bytes() == original_import
        assert not (root / '.codecraft/skill-release.lock').exists()
        assert source.read_bytes() == (candidate / 'skills/codecraft-capabilities-asr/references/api.md').read_bytes()

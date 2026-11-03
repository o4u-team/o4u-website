#!/usr/bin/env python3
"""Read-only export for compatibility tests. Does not export any rows."""
from pathlib import Path
import importlib.util
import os
import subprocess
spec = importlib.util.spec_from_file_location('prepare', Path(__file__).with_name('prepare-env.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
values = module.legacy_values()
env = os.environ.copy()
env['PGPASSWORD'] = values.get('DB_PASSWORD', '')
result = subprocess.run(['pg_dump', '--schema-only', '--no-owner', '--no-privileges',
    '--host', values.get('DB_HOST', 'localhost'), '--port', values.get('DB_PORT', '5432'),
    '--username', values.get('DB_USERNAME', ''), '--dbname', values.get('DB_DATABASE', '')],
    env=env, capture_output=True, text=True)
if result.returncode:
    raise SystemExit(result.stderr)
output = Path(__file__).resolve().parents[1] / 'tests/fixtures/legacy-schema.sql'
output.write_text(result.stdout)
print(f'Exported existing schema only to {output.name}; no data exported or changed.')

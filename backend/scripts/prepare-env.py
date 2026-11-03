#!/usr/bin/env python3
"""Copy only required legacy configuration into backend/.env. Never print secrets."""
from pathlib import Path
import re
import secrets
import sys

backend = Path(__file__).resolve().parents[1]
source = backend.parent / '.env'

def legacy_values():
    values = {}
    for line in source.read_text().splitlines():
        match = re.match(r'^([A-Z][A-Z0-9_]*)=(.*)$', line)
        if match:
            key, value = match.groups()
            if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
                value = value[1:-1]
            values[key] = value
    return values

def main():
    target = backend / '.env'
    if target.exists():
        sys.exit('backend/.env already exists; left unchanged.')
    values = legacy_values()
    keys = ['DATABASE_URL', 'DB_HOST', 'DB_PORT', 'DB_DATABASE', 'DB_USERNAME', 'DB_PASSWORD',
            'O4U_SECRET_KEY', 'GOOGLE_CLIENT_ID', 'GOOGLE_CLIENT_SECRET', 'GOOGLE_REDIRECT_URI',
            'ODOO_TENANT_BASE_URL', 'ODOO_TENANT_REGISTRATION_URL', 'ODOO_TENANT_REGISTRATION_KEY',
            'TURNSTILE_SITE_KEY', 'TURNSTILE_SECRET_KEY', 'FIREBASE_CREDENTIALS',
            'TELEGRAM_BOT_TOKEN', 'TELEGRAM_CHAT_ID', 'TELEGRAM_REGISTRATION_CHAT_ID']
    copied = {key: values[key] for key in keys if values.get(key)}
    credentials = copied.get('FIREBASE_CREDENTIALS')
    if credentials and not credentials.lstrip().startswith('{') and not Path(credentials).is_absolute():
        copied['FIREBASE_CREDENTIALS'] = str((backend.parent / credentials).resolve())
    copied.update(SESSION_SECRET=secrets.token_urlsafe(48), FRONTEND_URL='http://localhost:5173',
                  BIND_ADDRESS='127.0.0.1:8088', TURNSTILE_BYPASS='false')
    with target.open('x') as output:
        for key, value in copied.items():
            escaped = value.replace('\\', '\\\\').replace('"', '\\"').replace('$', '\\$').replace('\n', '\\n')
            output.write(f'{key}="{escaped}"\n')
    target.chmod(0o600)
    print('Created backend/.env using existing DB and integration configuration. No database changes.')

if __name__ == '__main__':
    main()

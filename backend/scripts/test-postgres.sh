#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Refuse user-provided URLs. This script exclusively targets the disposable cluster.
export TEST_DATABASE_URL="postgresql://$(id -un)@127.0.0.1:55438/o4u_migration_test"
psql "postgresql://$(id -un)@127.0.0.1:55438/postgres" -v ON_ERROR_STOP=1 -c 'DROP DATABASE IF EXISTS o4u_migration_test' -c 'CREATE DATABASE o4u_migration_test'
psql "$TEST_DATABASE_URL" -v ON_ERROR_STOP=1 -f tests/fixtures/legacy-schema.sql >/dev/null
RUSTC_WRAPPER= cargo test --workspace --test compatibility -- --ignored

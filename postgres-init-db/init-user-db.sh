#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER rust_user WITH PASSWORD 'qwerty88';
    CREATE DATABASE rust_db;
    GRANT ALL PRIVILEGES ON DATABASE rust_db TO rust_user;
EOSQL

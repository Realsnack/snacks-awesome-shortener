#!/bin/bash

#DATABASE_URL=postgres://postgres:postgrespwd@localhost:5432/postgres

if [ -z "$DATABASE_URL" ]; then
  echo "ERROR: No database URL set, please verify your settings"
  exit 1
fi

DB_URL_CHECK=$(echo "$DATABASE_URL" | grep -Eo 'postgres://.*:.*@.*:([0-9]{0,})/.*' | wc -l)

if [ "$DB_URL_CHECK" -eq 0 ]; then
  echo "Connection string '$DATABASE_URL' doesn't seem to be valid, please check"
  exit 2
fi

sleep 5
psql "$DATABASE_URL" <<EOF
  SELECT 1;
EOF

if [ "$?" -ne 0 ]; then
  echo "postgresql doesn't seem to be up, waiting 5 seconds"
  sleep 5
fi

INIT_SQL="CREATE ROLE sas_app WITH
LOGIN
NOSUPERUSER
INHERIT
NOCREATEDB
NOCREATEROLE
NOREPLICATION
NOBYPASSRLS
PASSWORD 'temppass';

CREATE DATABASE sas_db
WITH
OWNER = sas_app
ENCODING = 'UTF8'
TABLESPACE = pg_default
CONNECTION LIMIT = -1
IS_TEMPLATE = False;

\c sas_db

CREATE SCHEMA IF NOT EXISTS sas
AUTHORIZATION sas_app;

ALTER USER sas_app SET search_path TO sas,public;

GRANT ALL ON SCHEMA sas TO sas_app;

GRANT ALL ON SCHEMA sas TO postgres;"

echo "Initializing DB for Snack's Awesome Shortener"

psql "$DATABASE_URL" << EOF
  $INIT_SQL
EOF

exit 0

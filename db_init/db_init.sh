#!/bin/bash

DATABASE_URL=postgres://postgres:postgrespwd@localhost:5432/postgres

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

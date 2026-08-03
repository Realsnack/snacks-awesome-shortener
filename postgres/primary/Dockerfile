FROM registry.msvacina.cz/postgresql:latest

COPY pg_hba.conf /opt/bitnami/postgesql/conf/pg_hba.conf
COPY postgresql.conf /opt/bitnami/postgesql/conf/postgresql.conf
COPY repl_init.sql /docker-entrypoint-initdb.d/init.sql

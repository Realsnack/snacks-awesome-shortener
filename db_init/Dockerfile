FROM bitnami/minideb:latest

RUN apt update -y && \
    apt install postgresql-client -y

COPY db_init.sh .
RUN chmod +x db_init.sh

ENTRYPOINT [ "./db_init.sh" ]

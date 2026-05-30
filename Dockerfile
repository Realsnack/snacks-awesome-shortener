# syntax=docker/dockerfile:1.7

ARG RUST_VERSION=1.94

FROM rust:${RUST_VERSION}-slim-bookworm AS base

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    perl \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef

FROM base AS planner

COPY Cargo.toml Cargo.lock ./

COPY api_gateway/Cargo.toml api_gateway/Cargo.toml
COPY api_gateway/src api_gateway/src

COPY common/Cargo.toml common/Cargo.toml
COPY common/src common/src

COPY common_derive/Cargo.toml common_derive/Cargo.toml
COPY common_derive/src common_derive/src

COPY data_persistor/Cargo.toml data_persistor/Cargo.toml
COPY data_persistor/src data_persistor/src

COPY shorts_service/Cargo.toml shorts_service/Cargo.toml
COPY shorts_service/src shorts_service/src

COPY test_tool/Cargo.toml test_tool/Cargo.toml
COPY test_tool/src test_tool/src

# COPY health_service/Cargo.toml health_service/Cargo.toml
# COPY health_service/src health_service/src

RUN cargo chef prepare --recipe-path recipe.json

FROM base AS cacher

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook \
    --release \
    --recipe-path recipe.json

FROM base AS builder

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

COPY . .

ARG SERVICE_NAME

RUN cargo build \
    --release \
    -p ${SERVICE_NAME}

RUN cp target/release/${SERVICE_NAME} /app/app

FROM bitnami/minideb:latest AS runtime

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/app /usr/local/bin/app

ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/app"]

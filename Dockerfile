FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./

COPY api_gateway/Cargo.toml api_gateway/Cargo.toml
COPY api_gateway/src api_gateway/src

COPY common/Cargo.toml common/Cargo.toml
COPY common/src common/src

COPY data_persistor/Cargo.toml data_persistor/Cargo.toml
COPY data_persistor/src data_persistor/src

#COPY health_service/Cargo.toml health_service/Cargo.toml
#COPY health_service/src health_service/src

COPY shorts_service/Cargo.toml shorts_service/Cargo.toml
COPY shorts_service/src shorts_service/src

COPY test_tool/Cargo.toml test_tool/Cargo.toml
COPY test_tool/src test_tool/src

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ARG SERVICE_NAME
RUN cargo build --release -p ${SERVICE_NAME}
RUN cp target/release/${SERVICE_NAME} /app/app

FROM bitnami/minideb:latest AS runtime
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]

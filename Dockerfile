FROM lukemathwalker/cargo-chef:latest-rust-1-slim-buster AS chef
WORKDIR /app/
RUN apt update && apt install -y libssl-dev pkg-config

FROM chef AS planner

COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY src ./src
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

FROM debian:buster-slim
RUN apt update  \
    && apt install -y openssl ca-certificates
RUN update-ca-certificates

COPY --from=builder /app/target/release/cached-eth-rpc /app/cached-eth-rpc
COPY ./docker-entrypoint.sh /app/docker-entrypoint.sh

ENV ENDPOINTS="eth-chain=https://rpc.ankr.com/eth,bsc-chain=https://rpc.ankr.com/bsc"

EXPOSE 8124
CMD ["/app/docker-entrypoint.sh", "/app/cached-eth-rpc"]

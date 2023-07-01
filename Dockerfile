FROM rust:1 as builder
RUN apt update && apt install -y libssl-dev

WORKDIR /app/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
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

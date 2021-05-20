FROM rust:1.52.1 as builder

RUN apt-get update && apt-get install -y musl-tools

COPY /Cargo.toml /Cargo.lock /app/
COPY /src /app/src

RUN rustup target add x86_64-unknown-linux-musl && cargo build --release --manifest-path /app/Cargo.toml --target=x86_64-unknown-linux-musl

FROM alpine

ENTRYPOINT ["/app/graphql"]

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/graphql /app/graphql
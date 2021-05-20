FROM rust:1.52.1 as builder

RUN apt-get update && apt-get install -y musl-tools

COPY /Cargo.toml /Cargo.lock /app/
COPY /src /app/src

#ENV RUSTFLAGS="-C link_arg=-lgcc -C link_arg=-specs -C link_arg=/usr/lib/aarch64-linux-musl/musl-gcc.specs" 
#RUN rustup target add x86_64-unknown-linux-musl && cargo build --release --manifest-path /app/Cargo.toml --target=x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-musl && cargo build --release --manifest-path /app/Cargo.toml --target aarch64-unknown-linux-musl

FROM alpine

ENTRYPOINT ["/app/graphql"]

#COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/graphql /app/graphql
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/graphql /app/graphql
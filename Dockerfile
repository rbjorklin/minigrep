FROM rust:slim AS rust-compiler

WORKDIR /opt/build
ADD Cargo.lock /opt/build/Cargo.lock
ADD Cargo.toml /opt/build/Cargo.toml
ADD src/ /opt/build/src

RUN cargo build --release

# Resulting container starts here
FROM debian:buster-slim

COPY --from=rust-compiler /opt/build/target/release/minigrep /usr/local/bin/minigrep

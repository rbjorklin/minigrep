FROM rust:slim

ADD target/release/minigrep /usr/local/bin/minigrep

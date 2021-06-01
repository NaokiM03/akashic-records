FROM rust:1.52.1-slim-buster

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-watch

FROM rust:latest

RUN apt-get update && apt-get install -y \
  libssl-dev pkg-config \
  && rustup component add clippy rustfmt


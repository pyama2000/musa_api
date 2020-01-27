FROM rust:latest

WORKDIR /musa_diesel

RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo install cargo-watch && \
    rustup component add clippy

CMD ["cargo", "watch", "-x", "clippy", "-x", "run"]

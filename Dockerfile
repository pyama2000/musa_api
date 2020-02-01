FROM rust:latest

WORKDIR /musa_diesel

ENV USER pyama2000

EXPOSE 8000 8000

RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo install cargo-watch && \
    rustup component add clippy && \
    cargo init

COPY Cargo.* /musa_diesel/

RUN cargo run

CMD ["cargo", "watch", "-x", "clippy", "-x", "test", "-x", "run"]

FROM rust:latest

WORKDIR /musa_diesel

# RUN apt update -y && apt upgrade -y && \
#     apt install -y mysql-client && \
#     cargo install diesel_cli --no-default-features --features mysql

RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]

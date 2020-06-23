FROM rust:1 as builder
WORKDIR /usr/src/musa_api
COPY Cargo.toml Cargo.toml
RUN mkdir src && ¥
    echo "fn main() {println!(¥"if you see this, the build broke¥")}" > src/main.rs
RUN cargo build --release
RUN rm -rf target/release/deps/musa_api*
COPY . .
RUN cargo build --release
RUN cargo install --path .

FROM debian:buster-slim
EXPOSE 8000 8000
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/musa_api /usr/local/bin/musa_api

CMD ["musa_api"]

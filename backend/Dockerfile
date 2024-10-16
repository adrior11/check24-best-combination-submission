FROM rust:1.80.1 as builder

RUN apt-get update && apt-get install -y musl-tools

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend .

EXPOSE 8000

CMD ["./backend"]


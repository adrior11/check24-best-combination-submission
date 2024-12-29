# Use the latest Rust image as a build stage
FROM rust:latest AS builder 

ENV CARGO_TERM_COLOR=always
RUN apt-get update && apt-get install -y protobuf-compiler cmake

WORKDIR /usr/src/docker-build

# Create empty project for caching dependencies
RUN USER=root cargo init
COPY apps/libs ../libs
COPY apps/Cargo.lock apps/gateway/Cargo.toml ./

# Cache dependencies
RUN cargo install --path . --locked

COPY apps/gateway/ ./
RUN touch src/main.rs
RUN cargo install --path . --locked

FROM debian:bookworm-slim 
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=builder /usr/local/cargo/bin/gateway-service /bin/
RUN update-ca-certificates
CMD ["gateway-service"]

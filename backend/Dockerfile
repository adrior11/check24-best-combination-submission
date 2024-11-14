# Use the latest Rust image as a build stage
FROM rust:latest AS builder 

WORKDIR /backend

# Copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock 

# Copy source tree
COPY ./src ./src 

# Build for release
RUN cargo build --release 

# Use the official Debian image as a base
FROM debian:bookworm-slim 

# Copy build artefact from the build stage
COPY --from=builder /backend/target/release/backend /usr/local/bin/backend

# Run the backend on container startup and expose the port
EXPOSE 8000 
CMD ["backend"]


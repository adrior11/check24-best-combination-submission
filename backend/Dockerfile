# Use the latest Rust image as a build stage
FROM rust:latest AS builder 

# Create a new empty shell project
RUN USER=root cargo new --bin backend 
WORKDIR /backend

# Copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock 

# Cache dependencies 
RUN cargo build --release

# Remove dummy source file created by cargo new
RUN rm src/*.rs 

# Copy source tree
COPY ./src ./src 

# Build for release
RUN cargo build --release 

# Use the official Debian image as a base
FROM debian:bookworm-slim 

# Copy build artefact from the build stage
COPY --from=builder /backend/target/release/backend /usr/local/bin/backend

# Run the backend on container startup and expose the port
EXPOSE 3000 
CMD ["backend"]

# FROM ekidd/rust-musl-builder:stable AS builder
# ADD --chown=rust:rust . ./
# WORKDIR /app
# COPY Cargo.toml Cargo.lock ./
# Help cache dependencies so that they don't need to be rebuilt unless changed
# RUN cargo fetch
# COPY src ./src
# RUN cargo clean
# Build the application using the musl target
# RUN cargo build --release
# Use a minimal Alpine image for the final stage to keep the image size small
# FROM alpine:latest
# WORKDIR /app
# Copy the built binary from the builder stage
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend .
# COPY --from=builder \
    # /home/rust/src/target/x84_64-unknown-linux-musl/release/backend \
    # /usr/local/bin/
# EXPOSE 3000
# CMD /usr/local/bin/backend
# CMD ["./backend"]

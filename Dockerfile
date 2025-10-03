# --- Stage 1: The Builder ---
# We use the official Rust image as a temporary build environment.
# Using a specific version ensures your builds are repeatable.
FROM rust:1.79-slim-bookworm AS builder

# Set the working directory inside the container.
WORKDIR /usr/src/app

# Copy the source code into the container.
COPY . .

# Build the application in release mode for performance.
# This creates a statically linked binary if possible, which is great for small final images.
RUN cargo build --release

# --- Stage 2: The Final Image ---
# We use a minimal Debian image for the final container.
# This makes the image small and reduces the attack surface.
FROM debian:bookworm-slim AS final

# Set the working directory.
WORKDIR /usr/src/app

# Copy the compiled binary from the 'builder' stage.
# IMPORTANT: Change 'furrk-rust-bot' if your binary has a different name!
COPY --from=builder /usr/src/app/target/release/furrk-rust-bot .

# This is the command that will be run when the container starts.
CMD ["./furrk-rust-bot"]

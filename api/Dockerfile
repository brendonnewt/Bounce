# Use official Rust image as a build environment
FROM rust:1.83 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the entire project into the container
COPY . .

# Build the final application binary
RUN cargo build --release

# Use a minimal base image for the final image
FROM debian:bookworm-slim

# Install necessary dependencies (for SQLite support)
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled application binary from the builder stage
COPY --from=builder /app/target/release/api /usr/local/bin/api

# Set environment variable for the database location
ENV DATABASE_URL="sqlite:/app/bounce.db"

# Expose the port your application is listening on
EXPOSE 8080

# Run the application when the container starts
CMD ["api"]
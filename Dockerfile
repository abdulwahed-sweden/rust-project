# Build stage
FROM rust:1.82-slim AS builder

WORKDIR /app

# Install build dependencies  
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS final

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rust-project ./rust-project

# Create non-root user
RUN useradd -r -s /bin/false rustuser
RUN chown rustuser:rustuser ./rust-project
USER rustuser

EXPOSE 8001

CMD ["./rust-project"]
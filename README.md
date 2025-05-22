# Rust Docker Project Setup

Complete commands to set up a Rust web application with Docker from scratch.

## Initial Setup

```bash
# Create and clone repository
git clone https://github.com/username/rust-project.git
cd rust-project

# Initialize Rust project
cargo init --name rust-project

# Initialize Docker configuration
docker init
# Select: Rust, version 1.82, port 8001

# Generate lockfile
cargo generate-lockfile
```

## Update Cargo.toml

```toml
[package]
name = "rust-project"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
warp = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Create Web Server (src/main.rs)

```rust
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[tokio::main]
async fn main() {
    // GET /
    let hello = warp::path::end()
        .map(|| {
            let response = ApiResponse {
                message: "Hello from Rust Docker container!".to_string(),
                status: "success".to_string(),
            };
            warp::reply::json(&response)
        });

    // GET /health
    let health = warp::path("health")
        .map(|| {
            let response = ApiResponse {
                message: "Service is healthy".to_string(),
                status: "ok".to_string(),
            };
            warp::reply::json(&response)
        });

    // GET /api/info
    let info = warp::path!("api" / "info")
        .map(|| {
            let response = serde_json::json!({
                "service": "rust-project",
                "version": "0.1.0",
                "description": "Rust web service running in Docker",
                "author": "Your Name",
                "port": 8001
            });
            warp::reply::json(&response)
        });

    let routes = hello
        .or(health)
        .or(info)
        .with(warp::cors().allow_any_origin());

    println!("🚀 Server starting on http://0.0.0.0:8001");
    warp::serve(routes).bind(([0, 0, 0, 0], 8001)).await;
}
```

## Dockerfile

```dockerfile
# Build stage
FROM rust:1.82-slim AS builder

WORKDIR /app

# Install build dependencies  
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage - use compatible GLIBC version
FROM debian:bookworm-slim AS final

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
```

## .dockerignore

```text
# Rust
target/
Cargo.lock

# Git
.git
.gitignore

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Docker
Dockerfile*
docker-compose*
.dockerignore

# Documentation
README.md
*.md

# CI/CD
.github/
```

## GitHub Actions (.github/workflows/docker-build.yml)

```yaml
name: Docker Build and Push

on:
  push:
    branches: [ main, develop ]
    tags: [ 'v*' ]
  pull_request:
    branches: [ main ]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
    - name: Checkout repository
      uses: actions/checkout@v4

    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v3

    - name: Log in to Container Registry
      if: github.event_name != 'pull_request'
      uses: docker/login-action@v3
      with:
        registry: ${{ env.REGISTRY }}
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}

    - name: Extract metadata
      id: meta
      uses: docker/metadata-action@v5
      with:
        images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
        tags: |
          type=ref,event=branch
          type=ref,event=pr
          type=semver,pattern={{version}}
          type=semver,pattern={{major}}.{{minor}}
          type=raw,value=latest,enable={{is_default_branch}}

    - name: Build and push Docker image
      uses: docker/build-push-action@v5
      with:
        context: .
        platforms: linux/amd64,linux/arm64
        push: ${{ github.event_name != 'pull_request' }}
        tags: ${{ steps.meta.outputs.tags }}
        labels: ${{ steps.meta.outputs.labels }}
        cache-from: type=gha
        cache-to: type=gha,mode=max
```

## Build and Run Commands

```bash
# Update dependencies and regenerate lockfile
rm Cargo.lock
cargo update
cargo generate-lockfile

# Build and run with Docker
docker compose up --build

# Test endpoints
curl http://localhost:8001/
curl http://localhost:8001/health
curl http://localhost:8001/api/info

# Run in background
docker compose up --build -d

# View logs
docker compose logs

# Stop containers
docker compose down

# Clean Docker system
docker system prune -af
```

## Git Commands

```bash
# Add all files
git add .

# Commit changes
git commit -m "Add Docker support and web server

- Add Docker configuration (Dockerfile, compose.yaml, .dockerignore)
- Implement simple web API with warp framework
- Add health check and info endpoints
- Configure for port 8001"

# Push to GitHub
git push origin main

# Create and push tag
git tag v0.1.0
git push origin v0.1.0
```

## Production Deployment

```bash
# Pull and run from GitHub Container Registry
docker run -p 8001:8001 ghcr.io/username/rust-project:latest

# Or use docker-compose in production
docker-compose -f docker-compose.prod.yml up -d
```

## Troubleshooting

```bash
# Fix GLIBC issues - use bookworm-slim instead of bullseye-slim
# Fix Rust version compatibility - use rust:1.82 or newer
# Fix cargo lock issues - delete and regenerate Cargo.lock
# Fix Docker cache - run docker system prune -af
```
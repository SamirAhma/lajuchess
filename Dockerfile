# --- Stage 1: Frontend Build ---
FROM node:22-slim AS frontend-builder
WORKDIR /app/frontend

# Copy frontend manifests
COPY frontend/package*.json ./
RUN npm ci

# Copy frontend source
COPY frontend/ ./

# Copy types from rust (assuming they are already generated or will be generated)
# Note: In a fully automated Docker build, we might need typeshare-cli here too 
# if we want to guarantee types are fresh. For now, we assume frontend/src/lib/types.ts is committed or copied.
RUN npm run build

# --- Stage 2: Rust Builder ---
FROM rust:1.91-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# --- Stage 3: Runtime ---
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/lajuchess /app/lajuchess

# Copy static files from frontend builder
COPY --from=frontend-builder /app/public ./public

# Expose port
EXPOSE 3000

# Run the application
CMD ["./lajuchess"]

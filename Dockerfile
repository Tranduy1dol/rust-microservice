# Builder stage
FROM rust:latest AS builder

WORKDIR /app
COPY . .

# Build the application
# We need to build the specific binary 'app'
RUN cargo build --release --bin app

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (if any, e.g., OpenSSL)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/app /usr/local/bin/app

# Copy configuration
COPY --from=builder /app/config ./config

# Expose port
EXPOSE 3000

# Run the application
CMD ["app"]

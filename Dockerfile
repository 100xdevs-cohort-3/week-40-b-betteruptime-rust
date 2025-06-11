FROM rustlang/rust:nightly as builder

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

# Copy workspace Cargo files (from root)
COPY Cargo.toml ./
COPY Cargo.lock ./

# Copy API source code and Cargo.toml
COPY api/src ./api/src
COPY api/Cargo.toml ./api/

# Copy store directory (contains diesel.toml and migrations)
COPY store ./store

# Build the application (specify the api package)
RUN cargo build --release --package api

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/api ./

# Copy Diesel CLI if you need it at runtime
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy migrations and diesel.toml if needed at runtime
COPY --from=builder /app/store ./store

EXPOSE 8080
CMD ["./api"]

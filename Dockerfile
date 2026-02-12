# ---------- builder ----------
# Use a recent Rust so Cargo understands lockfile v4
FROM rust:1.89 AS builder

WORKDIR /app

# Cache dependencies (faster rebuilds)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && printf "fn main() {}\n" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source and build
COPY . .
RUN cargo build --release

# ---------- runtime ----------
FROM debian:bookworm-slim

WORKDIR /app

# TLS/CA certs for HTTPS + Postgres connections
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/acidcomics /usr/local/bin/acidcomics

EXPOSE 4000
ENV RUST_LOG=info

CMD ["acidcomics"]

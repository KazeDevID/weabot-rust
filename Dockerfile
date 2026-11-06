# Multi-stage Docker build for WeaBot-Rust
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /usr/src/weabot

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    git \
    clang \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/weabot/target/release/weabot-rust /app/weabot-rust
COPY .env.example /app/.env.example

RUN mkdir -p /app/sessions

VOLUME ["/app/sessions"]

CMD ["/app/weabot-rust"]

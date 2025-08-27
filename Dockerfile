# Stage 1: Build
FROM rust:1.89-trixie as builder

RUN apt-get update && apt-get install -y \
    libdbus-1-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/ruuvi-rust-mqtt

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Stage 2: Runtime
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    libbluetooth-dev \
    libdbus-1-3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/ruuvi-rust-mqtt/target/release/ruuvi-rust-mqtt .
RUN chmod +x /app/ruuvi-rust-mqtt

ENTRYPOINT ["./ruuvi-rust-mqtt"]
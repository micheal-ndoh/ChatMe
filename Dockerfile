FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.lock Cargo.toml /app/
COPY . .

RUN cargo build --release --bin ChatMe 

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

# Ensure the binary exists at the correct path
COPY --from=builder /app/target/release/ChatMe /usr/local/bin/ChatMe

CMD ["ChatMe"]
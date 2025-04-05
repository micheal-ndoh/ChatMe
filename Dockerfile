FROM rust:latest AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.lock Cargo.toml /app/
COPY . .

RUN cargo build --release --locked --bin ChatMe

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ChatMe /usr/local/bin/ChatMe

CMD ["/usr/local/bin/ChatMe"]
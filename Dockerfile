
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .
RUN cargo build --release --locked --bin ChatMe

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ChatMe /usr/local/bin/ChatMe

CMD ["/usr/local/bin/ChatMe"]
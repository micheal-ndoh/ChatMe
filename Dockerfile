FROM rust:1.73 AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .
COPY Cargo.lock Cargo.toml /app/

RUN cargo build --release --bin server --bin client

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/server /usr/local/bin/
COPY --from=builder /app/target/release/client /usr/local/bin/

CMD ["server"]

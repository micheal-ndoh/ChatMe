FROM rust:1.73-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .
COPY Cargo.lock Cargo.toml /app/

RUN cargo build --release
RUN cargo build --release --bin server
RUN cargo build --release --bin client

FROM alpine:3.19

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/ /usr/local/bin
COPY --from=builder /app/target/release/server /usr/local/bin/
COPY --from=builder /app/target/release/client /usr/local/bin/

CMD ["server"]

FROM rustlang/rust:nightly AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/backend ./backend

EXPOSE 8000
CMD ["cargo", "watch", "-x", "run"]
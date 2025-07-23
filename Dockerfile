# backend/Dockerfile
FROM rust:1.77 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

# --- producción ---
FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/backend ./backend

EXPOSE 8000
CMD ["./backend"]

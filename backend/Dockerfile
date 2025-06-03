# Dockerfile for Rust/Axum backend
FROM rust:1.87 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend
COPY .env.example .env
EXPOSE 8080
CMD ["/app/backend"]

# syntax=docker/dockerfile:1

# Etapa de build: compila el binario Rust
FROM rust:1.76-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p coordinator

# Etapa runtime mínima: solo certificados + binario
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/coordinator /usr/local/bin/coordinator

# Comando por defecto 
CMD ["/usr/local/bin/coordinator"]

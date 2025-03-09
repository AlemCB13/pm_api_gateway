FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 curl iputils-ping && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pm_api_gateway /usr/local/bin/pm_api_gateway

CMD ["pm_api_gateway"]


# # Etapa 1: Construcción
# FROM rust:latest AS builder
# WORKDIR /app
# COPY Cargo.toml Cargo.lock ./
# RUN cargo fetch               
# COPY src/ /app/src/                
# RUN cargo build --release

# # Etapa 2: Ejecución
# FROM ubuntu:22.04
# WORKDIR /app

# # Instalar dependencias necesarias
# RUN apt-get update && apt-get install -y \
#     libssl-dev \
#     ca-certificates \
#     && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /app/target/release/pm_api_gateway /usr/local/bin/pm_api_gateway

# # Exponer el puerto
# EXPOSE 8080

# # Comando de inicio
# CMD ["/usr/local/bin/pm_api_gateway"]





# # Etapa 1: Construcción
# FROM rust:latest as builder
# WORKDIR /app
# COPY . .
# RUN cargo build --release

# # Etapa 2: Ejecución
# FROM debian:buster-slim
# WORKDIR /app
# COPY --from=builder /app/target/release/pm_api_gateway /usr/local/bin/pm_api_gateway

# # Exponer el puerto del servicio
# EXPOSE 8080

# # Comando de inicio
# CMD ["pm_api_gateway"]

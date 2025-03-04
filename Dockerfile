# Usar una imagen base de Rust
FROM rust:1.60 as builder

# Establecer el directorio de trabajo
WORKDIR /app

# Copiar los archivos del proyecto
COPY . .

# Construir el proyecto
RUN cargo build --release

# Usar una imagen ligera para la ejecución
FROM debian:buster-slim

# Copiar el binario construido
COPY --from=builder /app/target/release/pm_api_gateway /usr/local/bin/pm_api_gateway

# Exponer el puerto 8080
EXPOSE 8080

# Comando para ejecutar el API Gateway
CMD ["pm_api_gateway"]
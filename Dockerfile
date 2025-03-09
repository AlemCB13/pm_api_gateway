FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 curl iputils-ping && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pm_api_gateway /usr/local/bin/pm_api_gateway

CMD ["pm_api_gateway"]



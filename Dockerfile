FROM rust:1.75.0-buster as builder
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:buster-slim
COPY --from=builder ./target/x86_64-unknown-linux-gnu/release/jumprankingsapi ./target/release/jumprankingsapi
WORKDIR /app
COPY .env /app/.env
RUN apt-get update && apt install -y openssl
EXPOSE 8080
ENTRYPOINT ["/target/release/jumprankingsapi"]
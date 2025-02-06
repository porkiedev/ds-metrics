FROM rust:latest AS builder
WORKDIR /app
COPY ../.. .
RUN apt update && apt install -y protobuf-compiler
RUN cargo build --bin server_api --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder --chown=nonroot /app/target/release/server_api /usr/local/bin/
WORKDIR /app
USER nonroot
ENTRYPOINT ["server_api"]
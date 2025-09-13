# syntax=docker/dockerfile:1
FROM rust:1.89 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/dmnapi /app/dmnapi
COPY --from=builder /app/target/release/. /app/target/release/.
CMD ["/app/dmnapi"]

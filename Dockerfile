FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/pentester .
COPY --from=builder /app/static ./static
EXPOSE 3000
CMD ["./pentester"]

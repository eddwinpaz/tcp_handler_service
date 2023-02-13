FROM rust as builder

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/release/tcp_handler_service /app/tcp_handler_service

WORKDIR /app

EXPOSE 7011

CMD ["./tcp_handler_service"]

FROM rust:latest as builder
WORKDIR /tmp
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY src src
RUN cargo build --release --bin=test_project

FROM debian:stretch-slim
RUN apt-get update -y && apt-get install -y libpq-dev openssl libssl1.0-dev ca-certificates
COPY --from=builder /tmp/target/release/test_project .
CMD ["./test_project"]
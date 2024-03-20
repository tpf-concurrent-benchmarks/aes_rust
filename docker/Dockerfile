FROM rust:slim AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /opt/app
COPY Cargo.toml Cargo.lock ./
COPY src /opt/app/src

RUN cargo build --release

RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch

WORKDIR /opt/app

COPY --from=builder /opt/app/target/x86_64-unknown-linux-musl/release/aes_rust  /opt/app/aes_rust

CMD ["/opt/app/aes_rust"]
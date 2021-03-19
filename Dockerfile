FROM rust as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-build-deps

RUN cd /tmp && USER=root cargo new --bin onion-vanity
WORKDIR /tmp/onion-vanity
COPY Cargo.toml Cargo.lock ./

RUN cargo build-deps --release --target=x86_64-unknown-linux-musl


COPY src /tmp/onion-vanity/src
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine
COPY --from=builder /tmp/onion-vanity/target/x86_64-unknown-linux-musl/release/onion-vanity /onion-vanity

ENTRYPOINT ["/onion-vanity"]
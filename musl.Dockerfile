FROM alpine:edge AS build

WORKDIR /app
RUN apk --no-cache add curl cargo g++ openssl openssl-dev clang jq ca-certificates bash linux-headers clang16-libclang

ENV OPENSSL_STATIC=yes \
  PKG_CONFIG_ALLOW_CROSS=true \
  PKG_CONFIG_ALL_STATIC=true \
  PATH="/root/.cargo/bin:${PATH}" \
  RUSTFLAGS="-C target-feature=+crt-static"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY cloudflare-0.10.1 cloudflare-0.10.1

RUN rustup target add x86_64-unknown-linux-musl
RUN CC=/usr/bin/gcc CXX=/usr/bin/g++ cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
WORKDIR /app
ENV PATH=/app:${PATH}

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/cflare /app

ENTRYPOINT ["/app/cflare"]

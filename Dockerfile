FROM --platform=amd64 rust:slim AS builder

WORKDIR /builder
COPY . /builder/
RUN apt update && apt install -y libc6 musl-tools gcc libssl-dev pkg-config
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM --platform=amd64 alpine

ENV DB_SERVER 'localhost'
ENV DB_PORT 3306
ENV DB_USER user
ENV DB_PASSWD 0000
ENV DB_DATABASE test
ENV USE_SSL false
ENV APP_PORT 8080

WORKDIR /server
# COPY cert /server/cert
COPY --from=builder /builder/target/x86_64-unknown-linux-musl/release/code_mmunity_server /server/
CMD ./code_mmunity_server

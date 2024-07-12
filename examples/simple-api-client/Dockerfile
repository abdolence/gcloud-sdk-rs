FROM rust:1 as builder

WORKDIR /opt/sources

COPY src src
COPY Cargo.toml ./

RUN cargo build --release --bin simple-api-client-example


FROM gcr.io/distroless/cc-debian12

COPY --from=builder /usr/lib/x86_64-linux-gnu/libcbor.* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypt* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libgmp* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libgnutls* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libicu* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl* /usr/lib/x86_64-linux-gnu/

COPY --from=builder /opt/sources/target/release/simple-api-client-example /opt/service/simple-api-client-example

WORKDIR /opt/service

CMD ["./simple-api-client-example"]

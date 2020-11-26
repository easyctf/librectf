FROM rust:1.48
RUN rustup target add x86_64-unknown-linux-musl
COPY . /app
WORKDIR /app
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.12
RUN mkdir /app
COPY --from=0 /app/target/x86_64-unknown-linux-musl/release/filestore /app
ENV FILESTORE_PORT=8000
EXPOSE 8000
ENTRYPOINT ["/app/filestore"]


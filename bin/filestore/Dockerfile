# Build stage

FROM rust:1.30.0
RUN mkdir /build
COPY . /build
WORKDIR /build
RUN cargo build --release

# Run stage

FROM alpine:latest

RUN apk add ca-certificates
WORKDIR /root
COPY --from=0 /build/target/release/filestore .
CMD ["./filestore"]

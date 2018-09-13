FROM ekidd/rust-musl-builder:nightly

COPY . ./
RUN sudo chown -R rust:rust .

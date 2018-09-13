FROM ekidd/rust-musl-builder

RUN rustup default nightly

COPY . ./
RUN sudo chown -R rust:rust .

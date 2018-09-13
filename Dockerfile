# bleeeeeeeeeding edge
FROM failedxyz/rust-musl-builder

RUN sudo apt-get update -y && sudo apt-get install -y libmysqlclient-dev

COPY . ./
RUN sudo chown -R rust:rust .

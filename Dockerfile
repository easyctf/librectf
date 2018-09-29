# bleeeeeeeeeding edge
FROM failedxyz/rust-musl-builder
RUN sudo apt-get update -y && sudo apt-get install -y cmake wget libncurses5-dev

RUN mkdir -p /opt
RUN sudo chown -R rust:rust /opt
WORKDIR /opt

# install boost
RUN wget https://dl.bintray.com/boostorg/release/1.67.0/source/boost_1_67_0.tar.gz
RUN tar xvf boost_1_67_0.tar.gz
WORKDIR /opt/boost_1_67_0
RUN sh /opt/boost_1_67_0/bootstrap.sh
RUN sudo /opt/boost_1_67_0/b2 install link=static -j4

# install mysql
RUN wget http://mysql.mirrors.hoobly.com/Downloads/MySQL-8.0/mysql-8.0.12.tar.gz
RUN tar xvf mysql-8.0.12.tar.gz -C /opt
RUN mkdir -p /opt/mysql-8.0.12/build
WORKDIR /opt/mysql-8.0.12/build
RUN cmake -DWITHOUT_SERVER=1 ..
RUN sudo make -j4 install

WORKDIR /home/rust/src
USER root
ENV RUSTUP_TOOLCHAIN=nightly
ENV RUSTUP_HOME=/home/rust/.multirust
ENV CARGO_HOME=/home/rust/.cargo
COPY . ./

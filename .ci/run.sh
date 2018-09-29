#!/bin/bash
#
# Modified version of build-release.sh
# Original comments left below.
#
# Usage: ./build-release <PROJECT> ${TRAVIS_TAG}-${TRAVIS_OS_NAME}
#
# The latest version of this script is available at
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release
#
# Called by `.travis.yml` to build release binaries.  We use
# ekidd/rust-musl-builder to make the Linux binaries so that we can run
# them unchanged on any distro, including tiny distros like Alpine (which
# is heavily used for Docker containers).  Other platforms get regular
# binaries, which will generally be dynamically linked against libc.
#
# If you have a platform which supports static linking of libc, and this
# would be generally useful, please feel free to submit patches.

set -euo pipefail

VOLUMES="-v $2-target:/home/rust/src/target -v $2-registry:/home/rust/.cargo/registry -v $2-git:/home/rust/.cargo/git"

case $1 in
    check)
        echo "Building static binaries using rust-musl-builder"
        docker build --build-arg TOOLCHAIN="nightly" -f Dockerfile -t build-"$2"-image .
        docker rm -f build-"$2" || true
        docker run -it --name build-"$2" $VOLUMES build-"$2"-image bash -c 'cargo build --all'
        docker rm build-"$2"
        docker rmi build-"$2"-image
        ;;
    build)
        echo "Building static binaries using rust-musl-builder"
        docker build --build-arg TOOLCHAIN="nightly" -t build-"$2"-image .
        docker rm -f build-"$2" || true
        docker run -it --name build-"$2" $VOLUMES build-"$2"-image bash -c 'cargo build --release --all'
        docker cp build-"$2":/home/rust/src/target/x86_64-unknown-linux-musl/release/"$2" "$2"
        docker rm build-"$2"
        docker rmi build-"$2"-image
        zip "$2"-"$3".zip "$2"
        ;;
esac

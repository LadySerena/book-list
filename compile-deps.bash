#!/usr/bin/env sh

set -xe

platform=$1

case $platform in
  linux/amd64)
    cargo build --locked --release --target=x86_64-unknown-linux-musl;;

  linux/arm64)
    cargo build --locked --release --target=aarch64-unknown-linux-musl;;
esac

echo "finished"

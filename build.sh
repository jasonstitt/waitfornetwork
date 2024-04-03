#!/usr/bin/env -S bash -xe

declare -A targets
targets=(['linux/arm64']='aarch64' ['linux/amd64']='x86_64')
echo $TARGETPLATFORM
export RUSTTARGET=${targets[${TARGETPLATFORM}]}-unknown-linux-musl
rustup target add $RUSTTARGET
cargo install --target $RUSTTARGET --path .

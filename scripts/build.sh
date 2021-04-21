#!/usr/bin/env bash

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.
export RUST_BACKTRACE=1  #  Show Rust errors.

pushd bl602-macros
cargo build
popd

pushd bl602-sdk
cargo build \
    --target riscv32imacf-unknown-none-elf.json \
    -Z build-std=core
popd

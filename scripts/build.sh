#!/usr/bin/env bash

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.
export RUST_BACKTRACE=1  #  Show Rust errors.

rust_build_options=" \
    --target riscv32imacf-unknown-none-elf.json \
    -Z build-std=core \
"

pushd bl602-macros
cargo build
popd

pushd bl602-sdk
cargo rustc \
    $rust_build_options \
    -- -Z unstable-options \
    --pretty expanded \
    > ../logs/sdk-expanded.rs

cargo build \
    $rust_build_options
popd

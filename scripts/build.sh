#!/usr/bin/env bash

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.
export RUST_BACKTRACE=1  #  Show Rust errors.

rust_build_options=" \
    --target riscv32imacf-unknown-none-elf.json \
    -Z build-std=core \
"

#  Build the macros
pushd bl602-macros
cargo build
popd

#  Copy the temp doclinks
cp bl602-sdk/doclinks.md .

pushd bl602-sdk

#  Expand the safe wrapper macros
cargo rustc \
    $rust_build_options \
    -- \
    -Z unstable-options \
    --pretty expanded \
    > ../logs/sdk-expanded.rs

#  Build the wrappers
cargo build \
    $rust_build_options

#  Generate the docs for inspection
cargo doc \
    $rust_build_options

popd

#  Remove the temp doclinks
echo 'Moved to [`bl602-sdk/doclinks.md`](bl602-sdk/doclinks.md)' >doclinks.md

#  Copy the docs
cp -r target/riscv32imacf-unknown-none-elf/doc/bl602_sdk docs
cp -r target/riscv32imacf-unknown-none-elf/doc/src docs

#  To publish:
#  pushd bl602-macros ; cargo publish ; popd
#  pushd bl602-sdk ; cargo publish --target ./riscv32imacf-unknown-none-elf.json -Z build-std=core ; popd

#!/usr/bin/env bash
#
# helper script designed to compile each target architecture
# this library is designed for.

targets=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu")

for target in ${targets[@]}
do
    cargo build --release --target=$target
done

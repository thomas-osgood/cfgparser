#!/usr/bin/env bash
#
# helper script designed to compile each target architecture
# this library is designed for.

targets=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu")

for target in ${targets[@]}
do
    cargo build --release --target=$target 2>/dev/null
    lastexitcode=$?
    if [ $lastexitcode -eq 0 ]; then
        echo "[+] $target compilation success"
    else
        echo "[-] compilation failed with code $lastexitcode. make sure $target is marked as \"installed\" in \"rustup target list\""
    fi
done

#!/usr/bin/env bash
#
# helper script designed to compile each target architecture
# this library is designed for.

# auto-detect installed architectures
readarray -t targets <<< $(rustup target list | grep "(installed)")

for idx in ${!targets[@]}
do
    # remove " (installed)" from target
    target=$(echo "${targets[$idx]}" | sed 's/ (installed)//')

    cargo build --release --target=$target 2>/dev/null
    lastexitcode=$?
    if [ $lastexitcode -eq 0 ]; then
        echo "[+] $target compilation success"
    else
        echo "[-] compilation failed with code $lastexitcode. make sure $target is marked as \"installed\" in \"rustup target list\""
    fi
done

#!/usr/bin/env bash

# confirm linux compilation available
rustup target add x86_64-unknown-linux-gnu
# add support for windows compilation
rustup target add x86_64-pc-windows-gnu
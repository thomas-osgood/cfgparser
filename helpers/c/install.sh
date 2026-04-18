#!/usr/bin/env bash

# this script is designed to install the cfgparser library
# onto a linux machine and should be run as sudo or with
# elevated permissions.

BINARY_DIR=/usr/local/bin
HEADER_DIR=/usr/local/include

BINARY_FILE=./cfgparser.so
HEADER_FILE=./cfgparser.h

# step 1: copy the binary file to /usr/local/bin
cp $BINARY_FILE $BINARY_DIR
LASTEXITCODE=$?
if [[ $LASTEXITCODE -ne 0 ]]; then
    echo "[-] unable to copy shared object to $BINARY_DIR. exit code: $LASTEXITCODE" >&2
    exit $LASTEXITCODE
fi
echo "[+] $BINARYFILE successfully copied to $BINARY_DIR"

# step 2: copy the header file to /usr/local/include
cp $HEADER_FILE $HEADER_DIR
LASTEXITCODE=$?
if [[ $LASTEXITCODE -ne 0 ]]; then
    echo "[-] unable to copy shared object to $HEADER_DIR. exit code: $LASTEXITCODE" >&2
    exit $LASTEXITCODE
fi
echo "[+] $HEADER_ILE successfully copied to $HEADER_DIR"

#!/usr/bin/env bash

# this script is designed to install the cfgparser library
# onto a linux machine and should be run as sudo or with
# elevated permissions.

LIB_DIR=/usr/local/lib
HEADER_DIR=/usr/local/include

LIB_FILE=./libcfgparser_core.so
HEADER_FILE=./cfgparser.h

# step 1: copy the binary file to /usr/local/bin
cp $LIB_FILE $LIB_DIR
LASTEXITCODE=$?
if [[ $LASTEXITCODE -ne 0 ]]; then
    echo "[-] unable to copy shared object to $LIB_DIR. exit code: $LASTEXITCODE" >&2
    exit $LASTEXITCODE
fi
echo "[+] $BINARYFILE successfully copied to $LIB_DIR"

# step 2: copy the header file to /usr/local/include
cp $HEADER_FILE $HEADER_DIR
LASTEXITCODE=$?
if [[ $LASTEXITCODE -ne 0 ]]; then
    echo "[-] unable to copy shared object to $HEADER_DIR. exit code: $LASTEXITCODE" >&2
    exit $LASTEXITCODE
fi
echo "[+] $HEADER_ILE successfully copied to $HEADER_DIR"

# notify the linker of the new library.
ldconfig 

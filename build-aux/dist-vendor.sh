#!/bin/sh
set -eu
export DIST="$1"
export SOURCE_ROOT="$2"

cd "${SOURCE_ROOT}"
mkdir "${DIST}"/.cargo
cargo vendor > "${DIST}/.cargo/config"
sed -i 's/^directory = ".*"/directory = "vendor"/g' "${DIST}/.cargo/config"
mv vendor "${DIST}"

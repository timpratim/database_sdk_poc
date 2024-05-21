#!/bin/bash
set -euxo pipefail

SCRIPT_DIR="${SCRIPT_DIR:-$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )}"
ROOT_DIR="$SCRIPT_DIR"

BINDINGS_DIR="$ROOT_DIR/greet"
BINARIES_DIR="$ROOT_DIR/target/aarch64-apple-darwin/debug"

# pushd is used to change the directory to the specified directory
# pushd $BINDINGS_DIR
LD_LIBRARY_PATH="${LD_LIBRARY_PATH:-}:$BINARIES_DIR" \
	CGO_LDFLAGS="-ldatabase_sdk_poc -L$BINARIES_DIR -lm -ldl" \
	CGO_ENABLED=1 \
	go run main.go
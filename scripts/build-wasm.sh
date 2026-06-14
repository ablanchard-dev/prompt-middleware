#!/usr/bin/env sh
# Build the WebAssembly bundle for prompt-engine and generate the JS bindings.
#
# Prerequisites (one-time):
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli
#
# Usage:
#   sh scripts/build-wasm.sh [target] [out-dir]
#     target  : nodejs (default) | web | bundler
#     out-dir : output directory (default: pkg)
set -eu

TARGET="${1:-nodejs}"
OUT_DIR="${2:-pkg}"
WASM="target/wasm32-unknown-unknown/release/prompt_engine_wasm.wasm"

cargo build --release --target wasm32-unknown-unknown -p prompt-engine-wasm
wasm-bindgen --target "$TARGET" --out-dir "$OUT_DIR" "$WASM"

echo "WASM bundle written to $OUT_DIR/ ($TARGET target)"

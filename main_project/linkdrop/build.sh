#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/linkdrop.wasm ./out/
wasm-opt -Os -o out/main.wasm out/linkdrop.wasm
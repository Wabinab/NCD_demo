#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/linkdrop.wasm ./res/
wasm-opt -Os -o res/output_s.wasm res/linkdrop.wasm
wasm-opt -Oz -o res/output.wasm res/linkdrop.wasm
ls res -lh
gzip -9 output_s.wasm | wc -c 
#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/tipping.wasm ./res/
wasm-opt -Os -o res/output_s.wasm res/tipping.wasm
ls res -lh
gzip -9 output_s.wasm | wc -c 
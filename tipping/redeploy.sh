#!/bin/bash

bash build.sh
export CONTRACT=tipping.dapplet_temp.testnet

near delete $CONTRACT dapplet_temp.testnet
near create-account $CONTRACT --masterAccount dapplet_temp.testnet --initialBalance 4.5

near deploy --accountId $CONTRACT --wasmFile res/output_s.wasm

near call $CONTRACT new '{"owner_id": "'$CONTRACT'"}' --accountId $CONTRACT
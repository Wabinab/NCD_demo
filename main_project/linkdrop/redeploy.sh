#!/bin/bash

near delete linkdrop.dapplet_temp.testnet dapplet_temp.testnet
near create-account linkdrop.dapplet_temp.testnet --masterAccount dapplet_temp.testnet --initialBalance 4.5

near deploy --accountId linkdrop.dapplet_temp.testnet --wasmFile out/main.wasm
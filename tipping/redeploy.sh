#!/bin/bash

near delete tipping.dapplet_temp.testnet dapplet_temp.testnet
near create-account tipping.dapplet_temp.testnet --masterAccount dapplet_temp.testnet --initialBalance 4.5

near deploy --accountId tipping.dapplet_temp.testnet --wasmFile out/main.wasm
#!/bin/bash
set -e

# This flags are needed to reduce size of compiled wasm file
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cd ..
cp target/wasm32-unknown-unknown/release/near_contract.wasm ./res/near_contract_prod.wasm

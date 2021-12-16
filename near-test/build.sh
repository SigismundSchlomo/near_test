#!/bin/bash
set -e
if
  [[ $1 = "--prod_address" ]]
then
  export TOKEN_ADDRESS="" # Write prod address.
else
  export TOKEN_ADDRESS="token.kuznietsov.testnet"
fi
# This flags are needed to reduce size of compiled wasm file
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cd ..
cp target/wasm32-unknown-unknown/release/near_test.wasm ./res/near_test.wasm

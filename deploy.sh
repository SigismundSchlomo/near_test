##!/bin/bash

# Example of deploy script

#
#if
#  [[ $2 = "--init" ]]
#then
#  near deploy --accountId $1 --wasmFile out/contract.wasm --initFunction initialize --initArgs '{"owner_id":"'"$1"'","total_supply":"44000000000000000000000000000000","metadata":{"spec":"1.0.0","name":"opn","symbol":"OPN","icon":null,"reference":null,"reference_hash":null,"decimals":24}}'
#elif
#  [[ $2 = "--reinit" ]]
#then
#  near deploy --accountId $1 --wasmFile out/contract.wasm --initFunction reinitialize --initArgs '{"owner_id":"'"$1"'","total_supply":"44000000000000000000000000000000","metadata":{"spec":"1.0.0","name":"opn","symbol":"OPN","icon":null,"reference":null,"reference_hash":null,"decimals":24}}'
#else
#  near deploy --accountId $1 --wasmFile ./out/contract.wasm
#fi
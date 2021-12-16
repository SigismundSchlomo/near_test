#!/bin/bash
set -e

cd near-test
./build.sh
cd ..
near dev-deploy res/near_test.wasm
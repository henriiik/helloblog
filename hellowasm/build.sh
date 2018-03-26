#!/bin/bash

set -e

cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/hellowasm.wasm -o ../static/hellowasm.gc.wasm

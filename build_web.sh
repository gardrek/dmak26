#!/bin/sh

cargo build --release --target wasm32-unknown-unknown --bin client --features client

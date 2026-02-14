#!/bin/sh

cargo build --release --bin client --features client
cargo build --release --bin server --features server

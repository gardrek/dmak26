#!/bin/sh

cargo build --bin client --features client
cargo build --bin server --features server

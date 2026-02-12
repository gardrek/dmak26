#!/bin/sh

rm pkg/*
echo $(date -u +%F_%T) > pkg/date_built.txt
~/.cargo/bin/wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/client.wasm && \
zip -r - index.html pkg/ assets/ > ../dmak26.zip

#!/bin/sh

rm pkg/*
TIMESTAMP=$(date -u +%F_%T)
echo $TIMESTAMP > pkg/date_built.txt
~/.cargo/bin/wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/client.wasm && \
    zip -r - index.html pkg/ assets/ > ../dmak26.zip && \
    cp ../dmak26.zip ../dmak26_$TIMESTAMP.zip

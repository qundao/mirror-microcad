#!/bin/bash
cargo run --bin update_copyright -- . \
    -S rs \
    -S pest \
    -S slint \
    -S wgsl \
    -S µcad \
    -H toml \
    -e "./target/*" \
    -e "./tests/*.µcad" \
    -e "./crates/cli/examples/*.µcad" \
    -e "./thirdparty/*"

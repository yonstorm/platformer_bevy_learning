#!/bin/sh
echo $1
RUST_LOG=warn,bevy_platformer=debug cargo run --features $1 bevy/dynamic_linking

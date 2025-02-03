#!/bin/bash

echo "Building Linux MUSL"

cargo build --release --target x86_64-unknown-linux-musl

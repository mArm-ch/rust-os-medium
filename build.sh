#!/bin/bash

# - Cleaning cargo files
cargo clean

# - Setting manifest dir
export CARGO_MANIFEST_DIR=$(pwd)

# - Building with configurated target in .cargo/config.toml
cargo build -Zbuild-std=core,alloc --no-default-features

# - Building the bootable image
cargo bootimage
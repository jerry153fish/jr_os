#!/usr/bin/env bash

set -xe

export PATH="$HOME/.cargo/bin:$PATH"
rustup component add llvm-tools-preview
rustup component add rust-src


cargo test
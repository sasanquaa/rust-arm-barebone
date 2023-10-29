#/bin/bash

cargo build &&
qemu-system-aarch64 -nographic -M virt -cpu cortex-a72 -s -S -kernel ./target/aarch64-unknown-none/debug/rust-arm-barebone
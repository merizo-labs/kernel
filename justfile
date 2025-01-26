build:
    cargo build
    cargo objcopy --bin merizo-kernel -- -O binary target/x86_64-unknown-none/debug/merizo-kernel.elf

build-release:
    cargo build --release
    cargo objcopy --bin merizo-kernel --release -- -O binary target/x86_64-unknown-none/release/merizo-kernel.elf
    
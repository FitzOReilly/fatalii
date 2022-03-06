release_profile := "--release"

build-all: build-linux build-windows

build-linux:
    cargo build {{release_profile}} --target x86_64-unknown-linux-musl

build-windows:
    cross build {{release_profile}} --target x86_64-pc-windows-gnu

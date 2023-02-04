default:
    just --list

build-debug:
    cargo build

build-release:
    cargo build --release

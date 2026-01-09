mod eml-client

default:
    just --list

build-debug:
    just eml-client build-debug
    cargo build --workspace --exclude=amail

build-release:
    just eml-client build-release
    cargo build --workspace --exclude=amail --release

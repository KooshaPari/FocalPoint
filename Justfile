# FocalPoint Justfile
set shell := ["bash", "-cu"]

default:
    @just --list

build:
    cargo build --workspace

test:
    cargo test --workspace

lint:
    cargo fmt -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo fmt

audit:
    cargo deny check

ci: build test lint audit

clean:
    cargo clean

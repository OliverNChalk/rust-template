#!/bin/sh

set -eu

USE_NIX=false
for arg in "$@"; do
    case "$arg" in
        -n|--nix) USE_NIX=true ;;
        *) echo "Unknown option: $arg" >&2; exit 1 ;;
    esac
done

if [ "$USE_NIX" = true ]; then
    cargo() { nix develop --command cargo "$@"; }
    treefmt() { nix develop --command treefmt "$@"; }
fi

set -x

cargo clippy --all-features --all-targets --tests -- --deny warnings
cargo test
cargo machete
treefmt --fail-on-change

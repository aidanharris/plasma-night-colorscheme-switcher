#!/bin/sh
cd "${BASH_SOURCE%/*}" || exit 1
cargo update
pkill -f target/release/plasma-night-colorscheme-switcher
exec cargo run --release -- "$@"

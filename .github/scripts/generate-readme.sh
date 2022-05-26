set -e

cargo install cargo-doc2readme

CARGO_BUILD_TARGET=x86_64-pc-windows-gnu cargo doc2readme --expand-macros --out README.md

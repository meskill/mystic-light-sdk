set -e

cargo install cargo-edit cargo-readme

cargo set-version $1
cargo readme > README.md
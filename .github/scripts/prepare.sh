set -e

cargo install cargo-edit cargo-readme

cargo set-version $1
cargo readme > README.md
cargo check --target=x86_64-pc-windows-gnu # update Cargo.lock

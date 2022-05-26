set -e

cargo install cargo-edit

cargo set-version $1

cargo check --target=x86_64-pc-windows-gnu # update Cargo.lock

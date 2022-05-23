set -e

cargo install cargo-edit # cargo-doc2readme

cargo set-version $1
# use [doc2readme](https://github.com/msrd0/cargo-doc2readme) when new version will be released
# cargo doc2readme --expand-macros --out README.md
cargo check --target=x86_64-pc-windows-gnu # update Cargo.lock

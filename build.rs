use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=sdk");

    let current_dir = env::current_dir()?;
    let out_dir = env::var("OUT_DIR").unwrap();

    let from_path = current_dir.join("sdk");

    let dest_path = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("sdk");

    if !dest_path.exists() {
        copy_dir::copy_dir(from_path, dest_path)?;
    }

    Ok(())
}

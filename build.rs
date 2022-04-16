use std::env;
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let target = env::var("TARGET").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(target)
        .join(build_type);
    return PathBuf::from(path);
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=sdk");

    let current_dir = env::current_dir()?;
    let out_dir = get_output_path();
    let from_path = current_dir.join("sdk");
    let dest_path = Path::new(&out_dir).join("sdk");

    if !dest_path.exists() {
        copy_dir::copy_dir(from_path, dest_path)?;
    }

    Ok(())
}

use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=sdk");
    println!("cargo:rerun-if-env-changed=MYSTIC_LIGHT_SDK_PATH");

    let sdk_path_env = env::var("MYSTIC_LIGHT_SDK_PATH");

    if let Ok(sdk_path) = sdk_path_env {
        let current_dir = env::current_dir()?;
        let out_dir = env::var("OUT_DIR").unwrap();

        let from_path = current_dir.join(sdk_path);

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
    }

    Ok(())
}

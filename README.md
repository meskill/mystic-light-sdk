# mystic_light_sdk

Rust SDK wrapper for the [Mystic Light SDK](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download)

## Requirements

1. Any MSI device with RGB support
1. Only Windows 7+
1. Dragon Center or Msi Center installed and running. You can download it [here](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download)
1. Admin rights to run program with the `mystic_light_sdk`

## Examples

```rust
use mystic_light_sdk::{Color, CommonError, DeviceLedState, MysticLightSDK};
use std::thread;
use std::time::Duration;

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
    "sdk/MysticLight_SDK_x64.dll"
} else {
    "sdk/MysticLight_SDK.dll"
};

fn main() -> Result<(), CommonError> {
    let sdk = MysticLightSDK::new(LIB_PATH)?;

    let devices = sdk.get_devices()?;

    println!("{:#?}", devices);

    let mut keyboard_leds = devices[2].leds()?;

    println!("{:#?}", keyboard_leds);

    let state = keyboard_leds[0].get_state()?.to_owned();

    println!("Current device state: {:#?}", state);

    println!("Disable lightning!");

    let new_state = DeviceLedState {
        color: Color {
            red: 0,
            green: 0,
            blue: 0,
        },
        style: String::from("NoAnimation"),
        ..state.clone()
    };

    keyboard_leds[0].set_state(&new_state)?;

    thread::sleep(Duration::from_secs(5));

    println!("Enable lightning");

    keyboard_leds[0].set_state(&state)?;

    Ok(())
}
```

### Pass right dll file

It depends on the os architecture you are building the program to and the os architecture for the end users.

Currently, most of the PC's are 64 bit architecture so you may just use MysticLight_SDK_x64.dll

Or if you are targetting both architecture you may use code below

```rust
const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
    "sdk/MysticLight_SDK_x64.dll" // path to the dll file that must be available in runtime
} else {
    "sdk/MysticLight_SDK.dll"
};
```

### Copy dll files to the output dir

As sdk dll is required in runtime you must provide these files somehow in the runtime.

You may use build script below in order to copy sdk files to the output dir. In this case dll files must reside in the `<path-to-your-project>/sdk` directory

```rust
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
```

## Panics

- in case of any problems with conversion from and into WinApi types

## Troubleshooting

### Timeout error on initialization

Make sure you have been fulfilled [requirements](#requirements) and you running the result program with the admin rights

### NotSupported error when trying to set color

Some of the device's styles do not support colors. In this case this kind of error will be generated.


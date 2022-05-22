//! Rust SDK wrapper for the [Mystic Light SDK](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download)
//!
//! # Requirements
//!
//! 1. Any MSI device with RGB support
//! 1. Only Windows 7+
//! 1. Dragon Center or Msi Center installed and running. You can download it [here](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download)
//! 1. Admin rights to run program with the `mystic_light_sdk`
//!
//! # Examples
//!
//! ```ignore
#![doc = include_str!("../examples/disable_light_for_5_sec.rs")]
//! ```
//!
//! ## Pass right dll file
//!
//! It depends on the os architecture you are building the program to and the os architecture for the end users.
//!
//! Currently, most of the PC's are 64 bit architecture so you may just use MysticLight_SDK_x64.dll
//!
//! Or if you are targetting both architecture you may use code below
//!
//! ```
//! const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
//!     "sdk/MysticLight_SDK_x64.dll" // path to the dll file that must be available in runtime
//! } else {
//!     "sdk/MysticLight_SDK.dll"
//! };
//! ```
//!
//! ## Copy dll files to the output dir
//!
//! As sdk dll is required in runtime you must provide these files somehow in the runtime.
//!
//! You may use build script included in the library itself to copy directory with sdk to the output directory. To do so provide environment variable `MYSTIC_LIGHT_SDK_PATH`
//! with the path to directory with the sdk's dll relative to current working directory.
//!
//! # Panics
//!
//! - in case of any problems with conversion from and into WinApi types
//!
//! # Features
//!
//! ## serde
//!
//! Enables [serde](https://crates.io/crates/serde) serialization/deserialization for some of the sdk structs
//!
//! # Troubleshooting
//!
//! ## Timeout error on initialization
//!
//! Make sure you have been fulfilled [requirements](#requirements) and you running the result program with the admin rights
//!
//! ## NotSupported error when trying to set color
//!
//! Some of the device's styles do not support colors. In this case this kind of error will be generated.
//!
pub mod sdk;
pub mod winapi;

pub use sdk::*;

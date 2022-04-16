//! Rust SDK wrapper for the [Mystic Light SDK](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download)

pub mod color;
pub mod device;
pub mod error;
pub mod led;
pub mod sdk;
pub mod types;

pub use color::*;
pub use device::*;
pub use error::*;
pub use led::*;
pub use sdk::*;
pub use types::*;

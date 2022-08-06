#![cfg_attr(docsrs, feature(doc_cfg))]

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
//! with the path to directory with the sdk's dll relative to the crate root e.g. `MYSTIC_LIGHT_SDK_PATH=sdk` if you have dlls inside `<crate_root>/sdk` directory
//!
//! # Panics
//!
//! - in case of any problems with conversion from and into WinApi types
//!
//! # How does it work
//!
//! ## Parallelism
//!
//! Underlying C++ SDK doesn't support parallel access and trying to use sdk that way will lead to wrong data. To prevent such problems this wrapper wraps underlying library in Arc and Mutex.
//! Arc is used to share the same library instance across wrapper structs. Mutex is used to prevent parallel access to the underlying library.
//!
//! That all means you can safely use rust wrapper both in single-threaded and multi-threaded environments, but actual sdk calls will be executed in sequence anyway.
//!
//! # Usage
//!
//! ## logging
//!
//! Logging is implemented with library [`log`](https://docs.rs/log/0.4.17/log/index.html) - to enable actual logging just pick one of the logger
//! implementation from the [list](https://docs.rs/log/0.4.17/log/index.html#available-logging-implementations) and activate log for the module `mystic_light` e.g. for `env_logger` pass `RUST_LOG=mystic_light_sdk`
//!
//! # Features
//!
//! ## serde
//!
//! Enables [serde](https://crates.io/crates/serde) serialization/deserialization for some of the sdk structs
//!
//! ## async-graphql
//!
//! Enables [async-graphql](https://crates.io/crates/async-graphql) support for sdk entities
//!
//! When this feature is enabled you can use [MysticLightGraphqlQuery] as async_graphql::Query and [MysticLightGraphqlMutation] as async_graphql::Mutation
//!
//! ```
//! use async_graphql::{EmptySubscription, Schema};
//! use mystic_light_sdk::{build_graphql_schema, MysticLightSDK, MysticLightGraphqlMutation, MysticLightGraphqlQuery};
//!
//! pub type MysticLightSchema = Schema<MysticLightGraphqlQuery, MysticLightGraphqlMutation, EmptySubscription>;
//!
//! pub fn create_qraphql_schema(sdk: MysticLightSDK) -> MysticLightSchema {
//!     let (query, mutation) = build_graphql_schema(sdk);
//!
//!     Schema::build(query, mutation, EmptySubscription).finish()
//! }
//!
//! ```
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

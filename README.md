# mystic_light_sdk ![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue) [![mystic_light_sdk on crates.io](https://img.shields.io/crates/v/mystic_light_sdk)](https://crates.io/crates/mystic_light_sdk) [![mystic_light_sdk on docs.rs](https://docs.rs/mystic_light_sdk/badge.svg)](https://docs.rs/mystic_light_sdk) [![Source Code Repository](https://img.shields.io/badge/Code-On%20github.com-blue)](https://github.com/meskill/mystic-light-sdk) [![mystic_light_sdk on deps.rs](https://deps.rs/repo/github/meskill/mystic-light-sdk/status.svg)](https://deps.rs/repo/github/meskill/mystic-light-sdk)

Rust SDK wrapper for the [Mystic Light SDK][__link0]


## Requirements

 1. Any MSI device with RGB support
 1. Only Windows 7+
 1. Dragon Center or Msi Center installed and running. You can download it [here][__link1]
 1. Admin rights to run program with the `mystic_light_sdk`


## Examples


```rust
use mystic_light_sdk::{Color, CommonError, DeviceLedState, MysticLightSDK};
use std::thread;
use std::time::Duration;

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
   "../sdk/MysticLight_SDK_x64.dll"
} else {
   "../sdk/MysticLight_SDK.dll"
};

fn main() -> Result<(), CommonError> {
   let sdk = MysticLightSDK::new(LIB_PATH)?;

   let devices = sdk.get_devices()?;

   println!("{:#?}", devices);

   println!("Second Device name is {}", devices[2].name());

   let mut keyboard_leds = devices[2].leds()?;

   println!("{:#?}", keyboard_leds);

   println!(
       "First led has name: {} with max_bright: {} and max_speed: {}",
       keyboard_leds[0].name(),
       keyboard_leds[0].max_bright(),
       keyboard_leds[0].max_speed()
   );

   let state = keyboard_leds[0].get_state()?;

   println!("Current device state: {:#?}", state);

   println!("Disable lightning!");

   let new_state = DeviceLedState {
       color: Color {
           red: 0,
           green: 0,
           blue: 0,
       },
       style: String::from("NoAnimation"),
       ..state
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

Currently, most of the PC???s are 64 bit architecture so you may just use MysticLight_SDK_x64.dll

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

You may use build script included in the library itself to copy directory with sdk to the output directory. To do so provide environment variable `MYSTIC_LIGHT_SDK_PATH` with the path to directory with the sdk???s dll relative to the crate root e.g. `MYSTIC_LIGHT_SDK_PATH=sdk` if you have dlls inside `<crate_root>/sdk` directory


## Panics

 - in case of any problems with conversion from and into WinApi types


## How does it work


### Parallelism

Underlying C++ SDK doesn???t support parallel access and trying to use sdk that way will lead to wrong data. To prevent such problems this wrapper wraps underlying library in Arc and Mutex. Arc is used to share the same library instance across wrapper structs. Mutex is used to prevent parallel access to the underlying library.

That all means you can safely use rust wrapper both in single-threaded and multi-threaded environments, but actual sdk calls will be executed in sequence anyway.


## Usage


### logging

Logging is implemented with library [`log`][__link2] - to enable actual logging just pick one of the logger implementation from the [list][__link3] and activate log for the module `mystic_light` e.g. for `env_logger` pass `RUST_LOG=mystic_light_sdk`


## Features


### serde

Enables [serde][__link4] serialization/deserialization for some of the sdk structs


### async-graphql

Enables [async-graphql][__link5] support for sdk entities

When this feature is enabled you can use [MysticLightSDK][__link6] as async_graphql::Query and [MysticLightSDKMutation][__link7] as async_graphql::Mutation


```rust
use async_graphql::{EmptySubscription, Schema};
use mystic_light_sdk::{MysticLightSDK, MysticLightSDKMutation};

pub type MysticLightSchema = Schema<MysticLightSDK, MysticLightSDKMutation, EmptySubscription>;

pub fn create_qraphql_schema(sdk: MysticLightSDK) -> MysticLightSchema {
    let mutation = MysticLightSDKMutation(sdk.clone());

    Schema::build(sdk, mutation, EmptySubscription).finish()
}

```


## Troubleshooting


### Timeout error on initialization

Make sure you have been fulfilled [requirements](#requirements) and you running the result program with the admin rights


### NotSupported error when trying to set color

Some of the device???s styles do not support colors. In this case this kind of error will be generated.


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG52uRQSwBdezG6GWW8ODAbr5G6KRmT_WpUB5G9hPmBcUiIp6YXKEG6X8erNhPunCG31Cv27-Bu8hG7nWaAb2Sc_TG4vh1fzzx_YPYWSCgm5NeXN0aWNMaWdodFNES_aCdk15c3RpY0xpZ2h0U0RLTXV0YXRpb272
 [__link0]: https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download
 [__link1]: https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download
 [__link2]: https://docs.rs/log/0.4.17/log/index.html
 [__link3]: https://docs.rs/log/0.4.17/log/index.html#available-logging-implementations
 [__link4]: https://crates.io/crates/serde
 [__link5]: https://crates.io/crates/async-graphql
 [__link6]: https://crates.io/crates/MysticLightSDK
 [__link7]: https://crates.io/crates/MysticLightSDKMutation

# mystic_light_sdk ![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue) [![mystic_light_sdk on crates.io](https://img.shields.io/crates/v/mystic_light_sdk)](https://crates.io/crates/mystic_light_sdk) [![mystic_light_sdk on docs.rs](https://docs.rs/mystic_light_sdk/badge.svg)](https://docs.rs/mystic_light_sdk) [![Source Code Repository](https://img.shields.io/badge/Code-On%20github.com-blue)](https://github.com/meskill/mystic-light-sdk) [![mystic_light_sdk on deps.rs](https://deps.rs/repo/github/meskill/mystic-light-sdk/status.svg)](https://deps.rs/repo/github/meskill/mystic-light-sdk)

Rust SDK wrapper for the [Mystic Light SDK][__link0]


## Requirements

 1. Any MSI device with RGB support
 2. Only Windows 7+
 3. Dragon Center or Msi Center installed and running. You can download it [here][__link1]
 4. Admin rights to run program with the `mystic_light_sdk`


## Examples


```rust
use mystic_light_sdk::{Color, CommonError, DeviceLedState, MysticLightSDK};
use std::thread;
use std::time::Duration;

use tracing::{info, warn, Level};
use tracing_subscriber::{fmt, fmt::format::FmtSpan};

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
   "../sdk/MysticLight_SDK_x64.dll"
} else {
   "../sdk/MysticLight_SDK.dll"
};

fn main() -> Result<(), CommonError> {
   fmt()
       .pretty()
       .with_max_level(Level::DEBUG)
       .with_span_events(FmtSpan::ACTIVE)
       .init();

   let sdk = MysticLightSDK::new(LIB_PATH)?;

   let devices: Vec<_> = sdk.devices_iter().collect();

   info!(?devices);

   info!(second_device_name = devices[2].name());

   let keyboard_leds: Vec<_> = devices[2].leds_iter().collect();

   info!(?keyboard_leds);

   info!(
       "First led has name: {} with max_bright: {} and max_speed: {}",
       keyboard_leds[0].name(),
       keyboard_leds[0].max_bright(),
       keyboard_leds[0].max_speed()
   );

   let state = keyboard_leds[0].get_state()?;

   info!("Current device state: {:#?}", state);

   warn!("Disable lightning!");

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

   warn!("Enable lightning");

   keyboard_leds[0].set_state(&state)?;

   Ok(())
}

```


### Pass right dll file

It depends on the os architecture you are building the program to and the os architecture for the end users.

Currently, most of the PC’s are 64 bit architecture so you may just use MysticLight_SDK_x64.dll

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

You may use build script included in the library itself to copy directory with sdk to the output directory. To do so provide environment variable `MYSTIC_LIGHT_SDK_PATH` with **absolute** path to directory with the sdk’s dll e.g. `MYSTIC_LIGHT_SDK_PATH=/workspaces/project/sdk`.


## Panics

 - in case of any problems with conversion from and into WinApi types


## How does it work


### Parallelism

Underlying C++ SDK doesn’t support parallel access and trying to use sdk that way will lead to wrong data. To prevent such problems this wrapper wraps underlying library in Arc and Mutex. Arc is used to share the same library instance across wrapper structs. Mutex is used to prevent parallel access to the underlying library.

That all means you can safely use rust wrapper both in single-threaded and multi-threaded environments, but actual sdk calls will be executed in sequence anyway.


## Usage


### tracing

Tracing is implemented with library [`tracing`][__link2] - to see tracing logs follow the [instructions of tracing crate][__link3].


## Features


### serde

Enables [serde][__link4] serialization/deserialization for some of the sdk structs


### async-graphql

Enables [async-graphql][__link5] support for sdk entities

When this feature is enabled you can use [MysticLightGraphqlQuery][__link6] as async_graphql::Query and [MysticLightGraphqlMutation][__link7] as async_graphql::Mutation


```rust
use async_graphql::{EmptySubscription, Schema};
use mystic_light_sdk::{build_graphql_schema, MysticLightSDK, MysticLightGraphqlMutation, MysticLightGraphqlQuery};

pub type MysticLightSchema = Schema<MysticLightGraphqlQuery, MysticLightGraphqlMutation, EmptySubscription>;

pub fn create_qraphql_schema(sdk: MysticLightSDK) -> MysticLightSchema {
    let (query, mutation) = build_graphql_schema(sdk);

    Schema::build(query, mutation, EmptySubscription).finish()
}

```


## Troubleshooting


### Timeout error on initialization

Make sure you have been fulfilled [requirements](#requirements) and you running the result program with the admin rights


### NotSupported error when trying to set color

Some of the device’s styles do not support colors. In this case this kind of error will be generated.


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG52uRQSwBdezG6GWW8ODAbr5G6KRmT_WpUB5G9hPmBcUiIp6YXKEGwZVjdq2ObFUG5iRsw2sZp7JGwtix7sRa4ihGxFVtj4lRduhYWSCgngaTXlzdGljTGlnaHRHcmFwaHFsTXV0YXRpb272gndNeXN0aWNMaWdodEdyYXBocWxRdWVyefY
 [__link0]: https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download
 [__link1]: https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download
 [__link2]: https://docs.rs/tracing/0.1.36/tracing/index.html
 [__link3]: https://docs.rs/tracing/0.1.36/tracing/index.html#in-executables
 [__link4]: https://crates.io/crates/serde
 [__link5]: https://crates.io/crates/async-graphql
 [__link6]: https://crates.io/crates/MysticLightGraphqlQuery
 [__link7]: https://crates.io/crates/MysticLightGraphqlMutation

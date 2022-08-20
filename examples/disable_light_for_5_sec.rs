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

use mystic_light_sdk::{CommonError, MysticLightSDK};
use tracing::{info, Level};
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
        .with_span_events(FmtSpan::FULL)
        .init();

    let sdk = MysticLightSDK::new(LIB_PATH)?;

    let devices = sdk.devices_iter();

    let leds = devices.map(|device| device.leds_iter());
    let states: Vec<Vec<_>> = leds
        .map(|led| {
            led.into_iter()
                .map(|led| led.get_state().unwrap())
                .collect()
        })
        .collect();

    info!(?states);

    Ok(())
}

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

    let mut keyboard_leds = devices[2].leds()?;

    println!("Second Device name is {}", devices[2].name());

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

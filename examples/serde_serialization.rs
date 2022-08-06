use mystic_light_sdk::MysticLightSDK;

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
    "../sdk/MysticLight_SDK_x64.dll"
} else {
    "../sdk/MysticLight_SDK.dll"
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = MysticLightSDK::new(LIB_PATH)?;

    let devices: Vec<_> = sdk.devices_iter().collect();

    println!("devices json: {}", serde_json::to_string_pretty(&devices)?);

    let keyboard_leds: Vec<_> = devices[2].leds_iter().collect();

    println!(
        "keyboard_leds json: {}",
        serde_json::to_string_pretty(&keyboard_leds)?
    );

    let state = keyboard_leds[0].get_state()?;

    println!(
        "Current device state json: {}",
        serde_json::to_string_pretty(&state)?
    );

    Ok(())
}

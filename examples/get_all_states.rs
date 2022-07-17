use mystic_light_sdk::{CommonError, MysticLightSDK};
use std::time::Instant;

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
    "../sdk/MysticLight_SDK_x64.dll"
} else {
    "../sdk/MysticLight_SDK.dll"
};

fn main() -> Result<(), CommonError> {
    let timer = Instant::now();
    let sdk = MysticLightSDK::new(LIB_PATH)?;

    println!("Init in {:?} secs", timer.elapsed().as_secs_f32());

    let devices = sdk.devices_iter();

    println!(
        "Getting devices for {:#?} secs",
        timer.elapsed().as_secs_f32()
    );

    let leds = devices.map(|device| device.leds_iter());

    println!("Getting leds for {:#?} secs", timer.elapsed().as_secs_f32());

    let states: Vec<Vec<_>> = leds
        .map(|led| {
            led.into_iter()
                .map(|led| led.get_state().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Getting states for {:#?} secs",
        timer.elapsed().as_secs_f32()
    );

    println!("States: {:#?}", states);

    Ok(())
}

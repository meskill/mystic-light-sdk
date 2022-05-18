use std::fmt::Debug;
use std::rc::Rc;

use super::led::DeviceLed;
use super::types::Result;
use libloading::Library;

/// Represents single hardware MysticLight Device
pub struct Device {
    library: Rc<Library>,
    name: String,
    led_count: u32,
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name)
            .field("led_count", &self.led_count)
            .finish()
    }
}

impl Device {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(library: Rc<Library>, name: String, led_count: u32) -> Self {
        Self {
            library,
            name,
            led_count,
        }
    }

    /// returns vec of device's leds
    pub fn leds(&self) -> Result<Vec<DeviceLed>> {
        let leds = (0..self.led_count)
            .into_iter()
            .map(|led_index| DeviceLed::new(Rc::clone(&self.library), &self.name, led_index))
            .collect::<Result<Vec<_>>>()?;

        Ok(leds)
    }
}

use std::{ptr::null_mut, rc::Rc};

use libloading::{Library, Symbol};

use crate::{winapi::FromSafeArray, DeviceTypes, LedCounts, MysticLightSdkResult};

use super::{device::Device, error::MysticLightSDKError, types::Result};

/// Rust Wrapper for the underlying Mystic Light SDK
pub struct MysticLightSDK {
    library: Rc<Library>,
}

impl MysticLightSDK {
    /// Parse the result of the underlying dll call and convert to the Rust's Result
    pub fn parse_result(
        result: MysticLightSdkResult,
    ) -> std::result::Result<(), MysticLightSDKError> {
        match result {
            0 => return Ok(()),
            -1 => Err(MysticLightSDKError::Generic),
            -2 => Err(MysticLightSDKError::Timeout),
            -3 => Err(MysticLightSDKError::NotImplemented),
            -4 => Err(MysticLightSDKError::NotInitialized),
            -101 => Err(MysticLightSDKError::InvalidArgument),
            -102 => Err(MysticLightSDKError::DeviceNotFound),
            -103 => Err(MysticLightSDKError::NotSupported),
            _ => Err(MysticLightSDKError::Unknown),
        }
    }

    /// Initialize MysticLight SDK with passed path to the dll file
    ///
    /// **You must pass valid dll based on the os architecture**
    pub fn new(lib_path: &str) -> Result<Self> {
        let library;

        unsafe {
            library = Library::new(lib_path)?;

            let initialize: Symbol<unsafe extern "C" fn() -> MysticLightSdkResult> =
                library.get(b"MLAPI_Initialize")?;

            MysticLightSDK::parse_result(initialize())?;
        }

        Ok(MysticLightSDK {
            library: Rc::new(library),
        })
    }

    /// Return list of the currently available devices
    pub fn get_devices(&self) -> Result<Vec<Device>> {
        let mut dev_type: DeviceTypes = null_mut();
        let mut led_count: LedCounts = null_mut();

        let dev_type_ptr: *mut DeviceTypes = &mut dev_type;
        let led_count_ptr: *mut LedCounts = &mut led_count;

        unsafe {
            let api_get_info: Symbol<
                unsafe extern "C" fn(
                    dev_type: *mut DeviceTypes,
                    led_count: *mut LedCounts,
                ) -> MysticLightSdkResult,
            > = self.library.get(b"MLAPI_GetDeviceInfo")?;

            MysticLightSDK::parse_result(api_get_info(dev_type_ptr, led_count_ptr))?
        }

        let devices: Vec<String> = Vec::from_safearray(dev_type);
        let leds: Vec<String> = Vec::from_safearray(led_count);

        Ok(devices
            .into_iter()
            .zip(leds)
            .map(|(device_name, led_count)| {
                let led_count: u32 = led_count.parse().expect("Cannot parse led count str");

                let device = Device::new(Rc::clone(&self.library), device_name, led_count);

                device
            })
            .collect())
    }
}

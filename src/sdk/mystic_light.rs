use std::{
    ptr::null_mut,
    sync::{Arc, Mutex},
};

use libloading::{Library, Symbol};

#[cfg(feature = "async-graphql")]
use crate::DeviceMutation;
use crate::{winapi::FromSafeArray, DeviceTypes, LedCounts, MysticLightSdkResult};

use super::{
    device::Device,
    error::MysticLightSDKError,
    types::{Filter, Result},
};

/// used for filtering devices.
/// Currently, supports only filtering by name
#[derive(Default)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
struct DeviceFilter {
    names: Option<Vec<String>>,
}

impl Filter<&str> for DeviceFilter {
    fn predicate(&self, device_name: &str) -> bool {
        match &self.names {
            Some(names) => {
                if names.is_empty() {
                    return true;
                }

                names.iter().any(|name| name == device_name)
            }
            None => true,
        }
    }
}

/// Rust Wrapper for the underlying Mystic Light SDK
#[derive(Clone)]
pub struct MysticLightSDK {
    library: Arc<Mutex<Library>>,
}

/// Rust Wrapper for the underlying Mystic Light SDK
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl MysticLightSDK {
    async fn devices(&self, #[graphql(default)] filter: DeviceFilter) -> Result<Vec<Device>> {
        self.get_devices_with_filter(filter)
    }
}

/// Mutation wrapper for sdk
#[cfg(feature = "async-graphql")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-graphql")))]
pub struct MysticLightSDKMutation(pub MysticLightSDK);

/// Mutation wrapper for sdk
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl MysticLightSDKMutation {
    async fn devices(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(default)] filter: DeviceFilter,
    ) -> Result<Vec<DeviceMutation>> {
        let devices = self.0.devices(ctx, filter).await?;

        Ok(devices.into_iter().map(DeviceMutation).collect())
    }
}

impl MysticLightSDK {
    /// Parse the result of the underlying dll call and convert to the Rust's Result
    pub fn parse_result(
        result: MysticLightSdkResult,
    ) -> std::result::Result<(), MysticLightSDKError> {
        match result {
            0 => Ok(()),
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
            library: Arc::new(Mutex::new(library)),
        })
    }

    pub fn get_devices(&self) -> Result<Vec<Device>> {
        self.get_devices_with_filter(DeviceFilter::default())
    }

    /// Return list of the currently available devices
    pub fn get_devices_with_filter<F>(&self, filter: F) -> Result<Vec<Device>>
    where
        F: for<'a> Filter<&'a str>,
    {
        let mut dev_type: DeviceTypes = null_mut();
        let mut led_count: LedCounts = null_mut();

        let dev_type_ptr: *mut DeviceTypes = &mut dev_type;
        let led_count_ptr: *mut LedCounts = &mut led_count;

        unsafe {
            let library = self.library.lock()?;

            let api_get_info: Symbol<
                unsafe extern "C" fn(
                    dev_type: *mut DeviceTypes,
                    led_count: *mut LedCounts,
                ) -> MysticLightSdkResult,
            > = library.get(b"MLAPI_GetDeviceInfo")?;

            MysticLightSDK::parse_result(api_get_info(dev_type_ptr, led_count_ptr))?
        }

        let devices: Vec<String> = Vec::from_safearray(dev_type);
        let leds: Vec<String> = Vec::from_safearray(led_count);

        Ok(devices
            .into_iter()
            .zip(leds)
            .filter(|(device_name, _)| filter.predicate(device_name))
            .map(|(device_name, led_count)| {
                let led_count: u32 = led_count.parse().expect("Cannot parse led count str");

                Device::new(Arc::clone(&self.library), device_name, led_count)
            })
            .collect())
    }
}

use std::{
    collections::HashMap,
    fmt::Debug,
    ptr::null_mut,
    sync::{Arc, Mutex},
};

#[cfg(feature = "async-graphql")]
use either::Either;
use libloading::{Library, Symbol};

use crate::{winapi::FromSafeArray, DeviceTypes, LedCounts, MysticLightSdkResult};
#[cfg(feature = "async-graphql")]
use crate::{DeviceMutation, SyncError};

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

impl Filter<&Device> for DeviceFilter {
    fn predicate(&self, device: &Device) -> bool {
        match &self.names {
            Some(names) => {
                if names.is_empty() {
                    return true;
                }

                names.iter().any(|name| name == device.name())
            }
            None => true,
        }
    }
}

#[cfg(feature = "async-graphql")]
fn filter_devices(
    devices: &HashMap<String, Device>,
    filter: DeviceFilter,
) -> Either<impl Iterator<Item = &Device>, impl Iterator<Item = &Device>> {
    match filter.names {
        Some(names) => Either::Left(
            names
                .into_iter()
                .filter_map(|led_name| devices.get(&led_name)),
        ),
        None => Either::Right(devices.values()),
    }
}

/// Rust Wrapper for the underlying Mystic Light SDK
pub struct MysticLightSDK {
    library: Arc<Mutex<Library>>,
    devices: HashMap<String, Device>,
    #[cfg(feature = "async-graphql")]
    lib_path: String,
}

impl Debug for MysticLightSDK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MysticLightSDK")
            .field("devices", &self.devices)
            .finish()
    }
}

/// Rust Wrapper for the underlying Mystic Light SDK
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl MysticLightSDK {
    /// returns Mystic Light devices
    async fn devices(&self, #[graphql(default)] filter: DeviceFilter) -> Vec<&Device> {
        filter_devices(&self.devices, filter).collect()
    }
}

/// Mutation wrapper for sdk
#[cfg(feature = "async-graphql")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-graphql")))]
pub struct MysticLightSDKMutation(pub Arc<MysticLightSDK>);

/// Mutation wrapper for sdk
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl MysticLightSDKMutation {
    /// returns Mystic Light devices wrapped in mutation wrapper
    async fn devices(
        &self,
        #[graphql(default)] filter: DeviceFilter,
    ) -> Result<Vec<DeviceMutation>> {
        let devices = filter_devices(&self.0.devices, filter)
            .map(DeviceMutation)
            .collect();

        Ok(devices)
    }
}

#[cfg(feature = "async-graphql")]
struct MysticLightGraphqlWrapper(pub Mutex<Arc<MysticLightSDK>>);

#[cfg(feature = "async-graphql")]
impl MysticLightGraphqlWrapper {
    fn sdk(&self) -> std::result::Result<Arc<MysticLightSDK>, SyncError> {
        let sdk = self.0.lock()?;

        Ok(Arc::clone(&sdk))
    }

    fn reload(&self) -> Result<()> {
        let mut sdk = self.0.lock()?;

        *sdk = Arc::new(MysticLightSDK::new(&sdk.lib_path)?);

        Ok(())
    }
}

/// Graphql query for MysticLightSDK
#[cfg(feature = "async-graphql")]
pub struct MysticLightGraphqlQuery(Arc<MysticLightGraphqlWrapper>);

/// Graphql query for MysticLightSDK
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl MysticLightGraphqlQuery {
    #[graphql(flatten)]
    async fn sdk(&self) -> std::result::Result<Arc<MysticLightSDK>, SyncError> {
        self.0.sdk()
    }
}

/// Graphql mutation for MysticLightSDK
#[cfg(feature = "async-graphql")]
pub struct MysticLightGraphqlMutation(Arc<MysticLightGraphqlWrapper>);

/// Graphql mutation for MysticLightSDK
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl MysticLightGraphqlMutation {
    #[graphql(flatten)]
    async fn sdk(&self) -> std::result::Result<MysticLightSDKMutation, SyncError> {
        let sdk = self.0.sdk()?;

        Ok(MysticLightSDKMutation(sdk))
    }

    /// Full reload of Mystic Light SDK to get most-fresh hardware data
    async fn reload(&self) -> Result<bool> {
        self.0.reload()?;

        Ok(true)
    }
}

#[cfg(feature = "async-graphql")]
pub fn build_graphql_schema(
    sdk: MysticLightSDK,
) -> (MysticLightGraphqlQuery, MysticLightGraphqlMutation) {
    let sdk = Arc::new(sdk);
    let wrapper = Arc::new(MysticLightGraphqlWrapper(Mutex::new(sdk)));

    (
        MysticLightGraphqlQuery(Arc::clone(&wrapper)),
        MysticLightGraphqlMutation(Arc::clone(&wrapper)),
    )
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
    #[tracing::instrument(level = "debug")]
    pub fn new(lib_path: &str) -> Result<Self> {
        let library;

        unsafe {
            library = Library::new(lib_path)?;

            let initialize: Symbol<unsafe extern "C" fn() -> MysticLightSdkResult> =
                library.get(b"MLAPI_Initialize")?;

            MysticLightSDK::parse_result(initialize())?;
        }

        let library = Arc::new(Mutex::new(library));
        let devices = Self::resolve_devices(&library)?;

        Ok(MysticLightSDK {
            library,
            devices,
            #[cfg(feature = "async-graphql")]
            lib_path: lib_path.to_owned(),
        })
    }

    /// returns Iterator iver Mystic Light devices
    pub fn devices_iter(&self) -> impl Iterator<Item = &Device> {
        self.devices.values()
    }

    /// reload cached devices info
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn reload(&mut self) -> Result<()> {
        self.devices = Self::resolve_devices(&self.library)?;

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn resolve_devices(library: &Arc<Mutex<Library>>) -> Result<HashMap<String, Device>> {
        let mut dev_type: DeviceTypes = null_mut();
        let mut led_count: LedCounts = null_mut();

        let dev_type_ptr: *mut DeviceTypes = &mut dev_type;
        let led_count_ptr: *mut LedCounts = &mut led_count;

        unsafe {
            let library = library.lock()?;

            let api_get_info: Symbol<
                unsafe extern "C" fn(
                    dev_type: *mut DeviceTypes,
                    led_count: *mut LedCounts,
                ) -> MysticLightSdkResult,
            > = library.get(b"MLAPI_GetDeviceInfo")?;

            MysticLightSDK::parse_result(api_get_info(dev_type_ptr, led_count_ptr))?
        }

        let devices_names: Vec<String> = Vec::from_safearray(dev_type);
        let leds: Vec<String> = Vec::from_safearray(led_count);

        let devices = devices_names
            .into_iter()
            .zip(leds)
            .map(|(device_name, led_count)| {
                let led_count: u32 = led_count.parse().expect("Cannot parse led count str");
                let device = Device::new(Arc::clone(library), device_name, led_count)?;

                Ok((device.name().to_owned(), device))
            })
            .collect::<Result<_>>()?;

        Ok(devices)
    }
}

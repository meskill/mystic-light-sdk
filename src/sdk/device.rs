use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[cfg(feature = "async-graphql")]
use crate::DeviceLedMutation;

use super::led::DeviceLed;
use super::types::{Filter, Result};
use libloading::Library;

/// used for filtering device's leds.
/// Currently, supports only filtering by name
#[derive(Default)]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
struct DeviceLedFilter {
    names: Option<Vec<String>>,
}

impl Filter<&DeviceLed> for DeviceLedFilter {
    fn predicate(&self, led: &DeviceLed) -> bool {
        match &self.names {
            Some(names) => {
                if names.is_empty() {
                    return true;
                }

                names.iter().any(|name| name == led.name())
            }
            None => true,
        }
    }
}

/// Represents single hardware MysticLight Device
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Device {
    name: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    library: Arc<Mutex<Library>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    led_count: u32,
}

/// Represents single hardware MysticLight Device
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl Device {
    #[graphql(name = "name")]
    async fn async_graphql_name(&self) -> &str {
        self.name()
    }

    #[graphql(name = "leds")]
    async fn async_graphql_leds(
        &self,
        #[graphql(default)] filter: DeviceLedFilter,
    ) -> Result<Vec<DeviceLed>> {
        self.leds_with_filter(filter)
    }
}

/// Mutation wrapper for a device
#[cfg(feature = "async-graphql")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-graphql")))]
pub struct DeviceMutation(pub Device);

/// Mutation wrapper for a device
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl DeviceMutation {
    async fn leds(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(default)] filter: DeviceLedFilter,
    ) -> Result<Vec<DeviceLedMutation>> {
        let leds = self.0.async_graphql_leds(ctx, filter).await?;

        Ok(leds.into_iter().map(DeviceLedMutation::new).collect())
    }
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

    pub(crate) fn new(library: Arc<Mutex<Library>>, name: String, led_count: u32) -> Self {
        Self {
            library,
            name,
            led_count,
        }
    }

    /// returns vec of device's leds
    pub fn leds(&self) -> Result<Vec<DeviceLed>> {
        self.leds_with_filter(DeviceLedFilter::default())
    }

    pub fn leds_with_filter<F>(&self, filter: F) -> Result<Vec<DeviceLed>>
    where
        F: for<'a> Filter<&'a DeviceLed>,
    {
        let leds = (0..self.led_count)
            .into_iter()
            .map(|led_index| DeviceLed::new(Arc::clone(&self.library), &self.name, led_index))
            .filter(|led| match led {
                Ok(led) => filter.predicate(led),
                Err(_) => true,
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(leds)
    }
}

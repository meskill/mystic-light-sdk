#[cfg(feature = "async-graphql")]
use either::Either;
use libloading::Library;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[cfg(feature = "async-graphql")]
use super::led::DeviceLedMutation;

use super::led::DeviceLed;
use super::types::{Filter, Result};

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

#[cfg(feature = "async-graphql")]
fn filter_leds(
    leds: &HashMap<String, DeviceLed>,
    filter: DeviceLedFilter,
) -> Either<impl Iterator<Item = &DeviceLed>, impl Iterator<Item = &DeviceLed>> {
    match filter.names {
        Some(names) => Either::Left(names.into_iter().filter_map(|led_name| leds.get(&led_name))),
        None => Either::Right(leds.values()),
    }
}

/// Represents single hardware MysticLight Device
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Device {
    name: String,

    pub(crate) leds: HashMap<String, DeviceLed>,

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

    /// returns device's leds
    async fn leds(&self, #[graphql(default)] filter: DeviceLedFilter) -> Vec<&DeviceLed> {
        filter_leds(&self.leds, filter).collect()
    }
}

/// Mutation wrapper for a device
#[cfg(feature = "async-graphql")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-graphql")))]
pub struct DeviceMutation<'a>(pub &'a Device);

/// Mutation wrapper for a device
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl<'a> DeviceMutation<'a> {
    /// returns device's leds wrapped in mutation wrapper
    async fn leds(&self, #[graphql(default)] filter: DeviceLedFilter) -> Vec<DeviceLedMutation> {
        filter_leds(&self.0.leds, filter)
            .map(DeviceLedMutation)
            .collect()
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

    #[tracing::instrument(level = "debug", skip(library))]
    pub(crate) fn new(library: Arc<Mutex<Library>>, name: String, led_count: u32) -> Result<Self> {
        let leds = Self::resolve_leds(&library, &name, led_count)?;

        Ok(Self {
            library,
            name,
            led_count,
            leds,
        })
    }

    /// returns iterator over device's leds
    pub fn leds_iter(&self) -> impl Iterator<Item = &DeviceLed> {
        self.leds.values()
    }

    /// reload cached leds info
    #[tracing::instrument(level = "debug", skip_all, fields(self.name = self.name))]
    pub fn reload(&mut self) -> Result<()> {
        self.leds = Self::resolve_leds(&self.library, &self.name, self.led_count)?;

        Ok(())
    }

    fn resolve_leds(
        library: &Arc<Mutex<Library>>,
        name: &str,
        led_count: u32,
    ) -> Result<HashMap<String, DeviceLed>> {
        let leds = (0..led_count)
            .into_iter()
            .map(|led_index| {
                let led = DeviceLed::new(Arc::clone(library), name, led_index)?;

                Ok((led.name().to_owned(), led))
            })
            .collect::<Result<_>>()?;

        Ok(leds)
    }
}

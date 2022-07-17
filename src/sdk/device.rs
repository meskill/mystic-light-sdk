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

    pub(crate) fn new(library: Arc<Mutex<Library>>, name: String, led_count: u32) -> Result<Self> {
        log::debug!(
            "fn:new call with args: name={}, led_count={}",
            name,
            led_count
        );

        let leds = Self::resolve_leds(&library, &name, led_count)?;

        Ok(Self {
            library,
            name,
            led_count,
            leds,
        })
    }

    pub fn leds_iter(&self) -> impl Iterator<Item = &DeviceLed> {
        self.leds.values()
    }

    pub fn reload(&mut self) -> Result<()> {
        log::debug!("fn:reload call");

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

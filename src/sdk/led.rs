use std::collections::HashSet;
use std::fmt::Debug;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

use libloading::{Library, Symbol};

use crate::winapi::{Bstr, FromSafeArray};
use crate::MysticLightSDK;

use super::color::Color;
use super::error::UsageError;
use super::types::{
    BrightLevel, ColorLevel, DeviceName, LedIndex, LedName, LedStyle, MysticLightSdkResult, Result,
    SpeedLevel,
};
use super::{CommonError, LedStyles, MysticLightSDKError};

/// Represents state of the single led
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::SimpleObject))]
pub struct DeviceLedState {
    /// current style of the led
    pub style: String,
    /// current color of the led (some of the styles do not support this, so there will be fake data in this case)
    pub color: Color,
    /// current brightness of the led (some of the styles do not support this, so there will be fake data in this case)
    pub bright: u32,
    /// current speed of the led (some of the styles do not support this, so there will be fake data in this case)
    pub speed: u32,
}

/// Represents state of the single led, but with optional fields
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct DeviceLedStateInput {
    /// current style of the led
    pub style: Option<String>,
    /// current color of the led (some of the styles do not support this, so there will be fake data in this case)
    pub color: Option<Color>,
    /// current brightness of the led (some of the styles do not support this, so there will be fake data in this case)
    pub bright: Option<u32>,
    /// current speed of the led (some of the styles do not support this, so there will be fake data in this case)
    pub speed: Option<u32>,
}

/// Represents single led of the device
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DeviceLed {
    // public fields
    name: String,
    supported_styles: HashSet<String>,
    max_bright: u32,
    max_speed: u32,

    // internal field that required to make api calls
    #[cfg_attr(feature = "serde", serde(skip))]
    library: Arc<Mutex<Library>>,

    #[cfg_attr(feature = "serde", serde(skip))]
    device_name: Bstr,

    #[cfg_attr(feature = "serde", serde(skip))]
    led_index: u32,
}

/// Represents single led of the device
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl DeviceLed {
    #[graphql(name = "name")]
    async fn async_graphql_name(&self) -> &str {
        self.name()
    }

    #[graphql(name = "supportedStyles")]
    async fn async_graphql_supported_styles(&self) -> &HashSet<String> {
        self.supported_styles()
    }

    #[graphql(name = "maxBright")]
    async fn async_graphql_max_bright(&self) -> u32 {
        self.max_bright()
    }

    #[graphql(name = "maxSpeed")]
    async fn async_graphql_max_speed(&self) -> u32 {
        self.max_speed()
    }

    #[graphql(name = "state")]
    async fn async_graphql_get_state(&self) -> Result<DeviceLedState> {
        self.get_state()
    }
}

/// Mutation wrapper for a device led
#[cfg(feature = "async-graphql")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-graphql")))]
pub struct DeviceLedMutation<'a>(pub &'a DeviceLed);

/// Mutation wrapper for a device led
#[cfg(feature = "async-graphql")]
#[async_graphql::Object]
impl<'a> DeviceLedMutation<'a> {
    /// updates state for the device led
    pub async fn set_state(&self, state: DeviceLedStateInput) -> Result<bool> {
        self.0.merge_with_state(&state)?;

        Ok(true)
    }
}

impl Debug for DeviceLed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceLed")
            .field("name", &self.name)
            .field("supported_styles", &self.supported_styles)
            .field("max_bright", &self.max_bright)
            .field("max_speed", &self.max_speed)
            .finish()
    }
}

impl DeviceLed {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn supported_styles(&self) -> &HashSet<String> {
        &self.supported_styles
    }

    pub fn max_bright(&self) -> u32 {
        self.max_bright
    }

    pub fn max_speed(&self) -> u32 {
        self.max_speed
    }

    #[tracing::instrument(level = "debug", skip(library))]
    pub(crate) fn new(
        library: Arc<Mutex<Library>>,
        device_name: &str,
        led_index: u32,
    ) -> Result<Self> {
        let get_led_info: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                led_name: *mut LedName,
                led_styles: *mut LedStyles,
            ) -> MysticLightSdkResult,
        >;
        let get_led_max_bright: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                max_bright: *mut BrightLevel,
            ) -> MysticLightSdkResult,
        >;
        let get_led_max_speed: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                max_speed: *mut SpeedLevel,
            ) -> MysticLightSdkResult,
        >;

        let library_instance = library.lock()?;

        unsafe {
            get_led_info = library_instance.get(b"MLAPI_GetLedInfo")?;
            get_led_max_bright = library_instance.get(b"MLAPI_GetLedMaxBright")?;
            get_led_max_speed = library_instance.get(b"MLAPI_GetLedMaxSpeed")?;
        }

        let device_name = Bstr::from(device_name);
        let mut led_name: LedName = null_mut();
        let mut led_styles: LedStyles = null_mut();
        let mut max_bright = 0u32;
        let mut max_speed = 0u32;

        unsafe {
            MysticLightSDK::parse_result(get_led_info(
                device_name.as_ptr(),
                led_index,
                &mut led_name,
                &mut led_styles,
            ))?;

            MysticLightSDK::parse_result(get_led_max_bright(
                device_name.as_ptr(),
                led_index,
                &mut max_bright,
            ))?;

            MysticLightSDK::parse_result(get_led_max_speed(
                device_name.as_ptr(),
                led_index,
                &mut max_speed,
            ))?;
        }

        let supported_styles = HashSet::from_safearray(led_styles);

        let name = Bstr::from(led_name).to_string();

        drop(library_instance);

        tracing::debug!(name, ?supported_styles, max_bright, max_speed);

        Ok(Self {
            library,
            device_name,
            led_index,
            name,
            supported_styles,
            max_bright,
            max_speed,
        })
    }

    /// Return state of the led
    #[tracing::instrument(level = "debug", skip(self), fields(self.name = self.name))]
    pub fn get_state(&self) -> Result<DeviceLedState> {
        let get_led_style: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                style: *mut LedStyle,
            ) -> MysticLightSdkResult,
        >;
        let get_led_color: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                red: *mut ColorLevel,
                green: *mut ColorLevel,
                blue: *mut ColorLevel,
            ) -> MysticLightSdkResult,
        >;
        let get_led_bright: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                bright: *mut BrightLevel,
            ) -> MysticLightSdkResult,
        >;
        let get_led_speed: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                speed: *mut SpeedLevel,
            ) -> MysticLightSdkResult,
        >;

        let library = self.library.lock()?;

        unsafe {
            get_led_style = library.get(b"MLAPI_GetLedStyle")?;
            get_led_color = library.get(b"MLAPI_GetLedColor")?;
            get_led_speed = library.get(b"MLAPI_GetLedSpeed")?;
            get_led_bright = library.get(b"MLAPI_GetLedBright")?;
        }

        let mut style: LedStyle = null_mut();
        let mut red = 0u32;
        let mut green = 0u32;
        let mut blue = 0u32;
        let mut speed = 0u32;
        let mut bright = 0u32;

        unsafe {
            MysticLightSDK::parse_result(get_led_style(
                self.device_name.as_ptr(),
                self.led_index,
                &mut style,
            ))?;

            MysticLightSDK::parse_result(get_led_color(
                self.device_name.as_ptr(),
                self.led_index,
                &mut red,
                &mut green,
                &mut blue,
            ))?;

            MysticLightSDK::parse_result(get_led_speed(
                self.device_name.as_ptr(),
                self.led_index,
                &mut speed,
            ))?;

            MysticLightSDK::parse_result(get_led_bright(
                self.device_name.as_ptr(),
                self.led_index,
                &mut bright,
            ))?;
        }

        let color = Color { red, green, blue };

        tracing::debug!(?color, bright, speed);

        Ok(DeviceLedState {
            style: Bstr::from(style).to_string(),
            color,
            bright,
            speed,
        })
    }

    /// Set led style
    #[tracing::instrument(level = "debug", skip(self), fields(self.name = self.name))]
    pub fn set_style(&self, style: &str) -> Result<()> {
        let set_led_style: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                style: LedStyle,
            ) -> MysticLightSdkResult,
        >;

        if !self.supported_styles.contains(style) {
            let mut iter = self.supported_styles.iter();
            let first = iter.next().unwrap().to_string();

            return Err(UsageError::NotSupportedStyle {
                style: style.to_string(),
                supported_styles: iter.fold(first, |acc, s| format!("{}, {}", acc, s)),
            }
            .into());
        }

        unsafe {
            let library = self.library.lock()?;

            set_led_style = library.get(b"MLAPI_SetLedStyle")?;

            let style: Bstr = style.into();

            MysticLightSDK::parse_result(set_led_style(
                self.device_name.as_ptr(),
                self.led_index,
                style.as_ptr(),
            ))?;
        }

        Ok(())
    }

    /// Set led color
    ///
    /// # Caveats
    ///
    /// Some of the styles do not support setting color for the led.
    /// In this case this method will return `Err(CommonError::MysticLightSDKError(Timeout))` as this error is returned by the underlying dll
    #[tracing::instrument(level = "debug", skip(self), fields(self.name = self.name))]
    pub fn set_color(&self, color: &Color) -> Result<()> {
        let set_led_color: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                r: ColorLevel,
                g: ColorLevel,
                b: ColorLevel,
            ) -> MysticLightSdkResult,
        >;

        unsafe {
            let library = self.library.lock()?;

            set_led_color = library.get(b"MLAPI_SetLedColor")?;

            let &Color { red, green, blue } = color;

            MysticLightSDK::parse_result(set_led_color(
                self.device_name.as_ptr(),
                self.led_index,
                red,
                green,
                blue,
            ))?;
        }

        Ok(())
    }

    /// Set led brightness
    ///
    /// # Caveats
    ///
    /// Some of the styles do not support setting color for the led.
    /// In this case this method will return `Err(CommonError::MysticLightSDKError(Timeout))` as this error is returned by the underlying dll
    #[tracing::instrument(level = "debug", skip(self), fields(self.name = self.name))]
    pub fn set_bright(&self, bright: BrightLevel) -> Result<()> {
        if bright > self.max_bright {
            return Err(UsageError::ExcessBrightLevel {
                level: bright,
                max_level: self.max_bright,
            }
            .into());
        }

        let set_led_bright: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                level: BrightLevel,
            ) -> MysticLightSdkResult,
        >;

        unsafe {
            let library = self.library.lock()?;

            set_led_bright = library.get(b"MLAPI_SetLedBright")?;

            MysticLightSDK::parse_result(set_led_bright(
                self.device_name.as_ptr(),
                self.led_index,
                bright,
            ))?;
        }

        Ok(())
    }

    /// Set led speed
    ///
    /// # Caveats
    ///
    /// Some of the styles do not support setting color for the led.
    /// In this case this method will return `Err(CommonError::MysticLightSDKError(Timeout))` as this error is returned by the underlying dll
    #[tracing::instrument(level = "debug", skip(self), fields(self.name = self.name))]
    pub fn set_speed(&self, speed: SpeedLevel) -> Result<()> {
        if speed > self.max_speed {
            return Err(UsageError::ExcessSpeedLevel {
                level: speed,
                max_level: self.max_bright,
            }
            .into());
        }

        let set_led_speed: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                speed: SpeedLevel,
            ) -> MysticLightSdkResult,
        >;

        unsafe {
            let library = self.library.lock()?;

            set_led_speed = library.get(b"MLAPI_SetLedSpeed")?;

            MysticLightSDK::parse_result(set_led_speed(
                self.device_name.as_ptr(),
                self.led_index,
                speed,
            ))?;
        }

        Ok(())
    }

    /// Set the whole state for the led
    ///
    /// # Caveats
    ///
    /// Some of the styles do not support setting color for the led.
    /// In this case this method will return `Err(CommonError::MysticLightSDKError(Timeout))` as this error is returned by the underlying dll
    #[tracing::instrument(level = "debug", skip(self), fields(self.name = self.name))]
    pub fn set_state(&self, state: &DeviceLedState) -> Result<()> {
        self.set_style(&state.style)?;
        self.set_bright(state.bright)?;
        self.set_speed(state.speed)?;
        match self.set_color(&state.color) {
            Ok(_) => (),
            Err(CommonError::SdkError {
                source: MysticLightSDKError::NotSupported,
            }) => (),
            error => return error,
        };

        Ok(())
    }

    /// Merge led current state with passed one i.e. applies only props that are Some() in passed argument
    pub fn merge_with_state(&self, state: &DeviceLedStateInput) -> Result<()> {
        if let Some(style) = &state.style {
            self.set_style(style)?;
        }

        if let Some(bright) = state.bright {
            self.set_bright(bright)?;
        }

        if let Some(speed) = state.speed {
            self.set_speed(speed)?;
        }

        if let Some(color) = &state.color {
            match self.set_color(color) {
                Ok(_) => (),
                Err(CommonError::SdkError {
                    source: MysticLightSDKError::NotSupported,
                }) => (),
                error => return error,
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "serde")]
    fn device_led_state_serialize_deserialize() {
        use super::DeviceLedState;
        use crate::Color;

        let device_led_state = DeviceLedState {
            bright: 10,
            speed: 5,
            color: Color {
                red: 10,
                green: 50,
                blue: 100,
            },
            style: String::from("led_style"),
        };

        let serialized_string = serde_json::to_string(&device_led_state).unwrap();

        assert_eq!(serialized_string, "{\"style\":\"led_style\",\"color\":{\"red\":10,\"green\":50,\"blue\":100},\"bright\":10,\"speed\":5}");

        assert_eq!(
            serde_json::from_str::<DeviceLedState>(&serialized_string).unwrap(),
            device_led_state
        );
    }
}

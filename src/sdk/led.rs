use std::collections::HashSet;
use std::fmt::Debug;
use std::ptr::null_mut;
use std::rc::Rc;

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

/// Represents single led of the device
pub struct DeviceLed {
    library: Rc<Library>,

    // internal field that required to make api calls
    device_name: Bstr,
    led_index: u32,

    // public fields
    name: String,
    supported_styles: HashSet<String>,
    max_bright: u32,
    max_speed: u32,
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

    pub fn new(library: Rc<Library>, device_name: &str, led_index: u32) -> Result<Self> {
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

        unsafe {
            get_led_info = library.get(b"MLAPI_GetLedInfo")?;
            get_led_max_bright = library.get(b"MLAPI_GetLedMaxBright")?;
            get_led_max_speed = library.get(b"MLAPI_GetLedMaxSpeed")?;
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

        unsafe {
            get_led_style = self.library.get(b"MLAPI_GetLedStyle")?;
            get_led_color = self.library.get(b"MLAPI_GetLedColor")?;
            get_led_speed = self.library.get(b"MLAPI_GetLedSpeed")?;
            get_led_bright = self.library.get(b"MLAPI_GetLedBright")?;
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

        Ok(DeviceLedState {
            style: Bstr::from(style).to_string(),
            color,
            bright,
            speed,
        })
    }

    /// Set led style
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
            set_led_style = self.library.get(b"MLAPI_SetLedStyle")?;

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
            set_led_color = self.library.get(b"MLAPI_SetLedColor")?;

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
    pub fn set_bright(&self, bright: BrightLevel) -> Result<()> {
        let set_led_bright: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                level: BrightLevel,
            ) -> MysticLightSdkResult,
        >;

        unsafe {
            set_led_bright = self.library.get(b"MLAPI_SetLedBright")?;

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
    pub fn set_speed(&self, speed: SpeedLevel) -> Result<()> {
        let set_led_speed: Symbol<
            unsafe extern "C" fn(
                device_name: DeviceName,
                led_index: LedIndex,
                speed: SpeedLevel,
            ) -> MysticLightSdkResult,
        >;

        unsafe {
            set_led_speed = self.library.get(b"MLAPI_SetLedSpeed")?;

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
    pub fn set_state(&mut self, state: &DeviceLedState) -> Result<()> {
        self.set_style(&state.style)?;
        match self.set_color(&state.color) {
            Ok(_) => (),
            Err(CommonError::SdkError {
                source: MysticLightSDKError::NotSupported,
            }) => (),
            error => return error,
        };
        self.set_bright(state.bright)?;
        self.set_speed(state.speed)?;

        Ok(())
    }
}

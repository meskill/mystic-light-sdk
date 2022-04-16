use libloading::Error as LibLoadingError;

use custom_error::custom_error;

custom_error! {
  /// Errors generated by the MysticLight SDK, see [SDK docs](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download)
pub MysticLightSDKError
  Generic = "Generic error",
  /// This error will raise in case of insufficient rights as well (run as an administrator)
  Timeout = "Request is timeout",
  NotImplemented = "MSI Application not found or installed version not supported",
  NotInitialized = "MSI Application was not initialized",
  InvalidArgument = "The parameter value is not valid",
  DeviceNotFound = "The device not found",
  NotSupported = "Requested feature is not supported in the selected LED",
  Unknown = "Unknown error",
}

custom_error! {
    /// Errors for bad sdk usage
pub UsageError
    /// Tried to set style that is not supported by current device
    NotSupportedStyle{style: String, supported_styles: String} = "{style} is not in the supported style list: {supported_styles}"
}

/// CommonError that may happen during usage of this library
#[derive(Debug)]
pub enum CommonError {
    SdkError(MysticLightSDKError),
    LibraryError(LibLoadingError),
    UsageError(UsageError),
}

impl From<MysticLightSDKError> for CommonError {
    fn from(error: MysticLightSDKError) -> Self {
        CommonError::SdkError(error)
    }
}

impl From<LibLoadingError> for CommonError {
    fn from(error: LibLoadingError) -> Self {
        CommonError::LibraryError(error)
    }
}

impl From<UsageError> for CommonError {
    fn from(error: UsageError) -> Self {
        CommonError::UsageError(error)
    }
}

use winapi::shared::minwindef::DWORD;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::SAFEARRAY;

use crate::CommonError;

/// Return type of the underlying functions in the mystic light dll
pub type MysticLightSdkResult = i32;
pub type Result<T> = std::result::Result<T, CommonError>;

// msi c++ types
pub type DeviceName = BSTR;
pub type DeviceTypes = *mut SAFEARRAY;
pub type LedCounts = *mut SAFEARRAY;
pub type LedIndex = DWORD;
pub type LedName = BSTR;
pub type LedStyle = BSTR;
pub type LedStyles = *mut SAFEARRAY;
pub type ColorLevel = DWORD;
pub type BrightLevel = DWORD;
pub type SpeedLevel = DWORD;

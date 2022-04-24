//! Conversions between WinApi types and Rust types
//!
//! # Uses
//!
//! - [winapi](https://docs.rs/winapi/latest/winapi/index.html) to get WinApi types
//! - [widestring](https://docs.rs/widestring/0.4.3/widestring/index.html) to get windows compatible UTF wide strings
//! - [oaidl](https://docs.rs/oaidl/latest/oaidl/index.html) to make conversion between widestring and WinAPI types
//!
use std::vec::IntoIter;

use oaidl::{BStringExt, Ptr, SafeArrayExt};
use widestring::U16String;
use winapi::{shared::wtypes::BSTR, um::oaidl::SAFEARRAY};

/// Wrapper for the BSTR.
///
/// # Features
///
/// - allocates bstr string from the
/// - frees memory after usage thanks to the Drop trait
pub struct Bstr {
    bstr: Ptr<u16>,
}

impl Bstr {
    /// Returns inner pointer to the BSTR string.
    ///
    /// This method should be used to get actually BSTR that most of the FFI expects
    pub fn as_ptr(&self) -> BSTR {
        self.bstr.as_ptr()
    }
}

impl Default for Bstr {
    /// Create new wrapper for the empty BSTR string
    ///
    /// # Panics
    ///
    /// - In case of error while allocating BSTR string
    fn default() -> Self {
        Bstr {
            bstr: U16String::new()
                .allocate_bstr()
                .expect("Cannot create BSTR"),
        }
    }
}

impl From<&str> for Bstr {
    /// Creates BSTR wrapper from the string
    ///
    /// # Panics
    ///
    /// - In case of any errors while converting to the BSTR
    fn from(s: &str) -> Self {
        Bstr {
            bstr: U16String::from_str(s)
                .consume_to_bstr()
                .expect("Cannot convert string to BSTR"),
        }
    }
}

impl From<&Bstr> for BSTR {
    fn from(bstr: &Bstr) -> Self {
        bstr.bstr.as_ptr()
    }
}

impl From<BSTR> for Bstr {
    fn from(bstr: BSTR) -> Self {
        Self {
            bstr: U16String::from_bstr(bstr)
                .consume_to_bstr()
                .expect("Cannot convert string to BSTR"),
        }
    }
}

impl Drop for Bstr {
    fn drop(&mut self) {
        U16String::deallocate_bstr(self.bstr)
    }
}

impl ToString for Bstr {
    fn to_string(&self) -> String {
        U16String::from_bstr(self.bstr.as_ptr()).to_string_lossy()
    }
}

/// Trait that implements basic conversions from SAFEARRAY to Rust types
pub trait FromSafeArray {
    fn from_safearray(safearray: *mut SAFEARRAY) -> Self;
}

impl<T: FromIterator<String>> FromSafeArray for T {
    /// Converts SAFEARRAY to FromIterator<String>
    ///
    /// # Examples
    ///
    /// ```
    /// let array: *mut SAFEARRAY;
    ///
    /// let vec: Vec<String> = Vec::from_safearray(array);
    ///
    /// let hashSet: HashSet<String> = HashSet::from_safearray(array);
    /// ```
    ///
    /// # Panics
    ///
    /// - passing null pointer as safearray
    /// - in case of any error happened during conversion from SAFEARRAY
    fn from_safearray(array: *mut SAFEARRAY) -> Self {
        let ptr = Ptr::with_checked(array).expect("Got null pointer");

        let vec =
            IntoIter::<U16String>::from_safearray(ptr).expect("Cannot convert from SAFEARRAY");

        vec.iter().map(|s| s.to_string_lossy()).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn convert_to_bstr_and_backward() {
        let s = "some test string";

        let bstr = Bstr::from(s);

        assert_eq!(bstr.to_string(), s);
    }

    #[test]
    fn convert_from_raw_bstr() {
        let s = "some test string";

        let bstr = Bstr::from(s);

        let bstr_from = Bstr::from(bstr.as_ptr());

        assert_eq!(bstr_from.to_string(), s);
    }

    #[test]
    fn convert_from_safearray_to_vec() {
        let v = vec![
            U16String::from_str("test1"),
            U16String::from_str("test2"),
            U16String::from_str("test3"),
        ];

        let safe_array = v.into_iter().into_safearray().unwrap();

        let parsed = Vec::from_safearray(safe_array.as_ptr());

        assert_eq!(parsed, vec!["test1", "test2", "test3"]);
    }

    #[test]
    fn convert_from_safearray_to_hashset() {
        let v = vec![
            U16String::from_str("test1"),
            U16String::from_str("test2"),
            U16String::from_str("test3"),
        ];

        let safe_array = v.into_iter().into_safearray().unwrap();

        let parsed: HashSet<String> = HashSet::from_safearray(safe_array.as_ptr());

        assert_eq!(parsed.len(), 3);
        assert!(parsed.contains("test1"));
        assert!(parsed.contains("test2"));
        assert!(parsed.contains("test3"));
    }
}

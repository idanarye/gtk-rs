// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use ffi as glib_ffi;
use error::ErrorDomain;
use translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum KeyFileError {
    UnknownEncoding,
    Parse,
    NotFound,
    KeyNotFound,
    GroupNotFound,
    InvalidValue,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for KeyFileError {
    type GlibType = ffi::GKeyFileError;

    fn to_glib(&self) -> ffi::GKeyFileError {
        match *self {
            KeyFileError::UnknownEncoding => ffi::G_KEY_FILE_ERROR_UNKNOWN_ENCODING,
            KeyFileError::Parse => ffi::G_KEY_FILE_ERROR_PARSE,
            KeyFileError::NotFound => ffi::G_KEY_FILE_ERROR_NOT_FOUND,
            KeyFileError::KeyNotFound => ffi::G_KEY_FILE_ERROR_KEY_NOT_FOUND,
            KeyFileError::GroupNotFound => ffi::G_KEY_FILE_ERROR_GROUP_NOT_FOUND,
            KeyFileError::InvalidValue => ffi::G_KEY_FILE_ERROR_INVALID_VALUE,
            KeyFileError::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileError> for KeyFileError {
    fn from_glib(value: ffi::GKeyFileError) -> Self {
        match value {
            ffi::G_KEY_FILE_ERROR_UNKNOWN_ENCODING => KeyFileError::UnknownEncoding,
            ffi::G_KEY_FILE_ERROR_PARSE => KeyFileError::Parse,
            ffi::G_KEY_FILE_ERROR_NOT_FOUND => KeyFileError::NotFound,
            ffi::G_KEY_FILE_ERROR_KEY_NOT_FOUND => KeyFileError::KeyNotFound,
            ffi::G_KEY_FILE_ERROR_GROUP_NOT_FOUND => KeyFileError::GroupNotFound,
            ffi::G_KEY_FILE_ERROR_INVALID_VALUE => KeyFileError::InvalidValue,
        }
    }
}

impl ErrorDomain for KeyFileError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::g_key_file_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            x if x == ffi::G_KEY_FILE_ERROR_UNKNOWN_ENCODING as i32 => Some(KeyFileError::UnknownEncoding),
            x if x == ffi::G_KEY_FILE_ERROR_PARSE as i32 => Some(KeyFileError::Parse),
            x if x == ffi::G_KEY_FILE_ERROR_NOT_FOUND as i32 => Some(KeyFileError::NotFound),
            x if x == ffi::G_KEY_FILE_ERROR_KEY_NOT_FOUND as i32 => Some(KeyFileError::KeyNotFound),
            x if x == ffi::G_KEY_FILE_ERROR_GROUP_NOT_FOUND as i32 => Some(KeyFileError::GroupNotFound),
            x if x == ffi::G_KEY_FILE_ERROR_INVALID_VALUE as i32 => Some(KeyFileError::InvalidValue),
            _ => Some(KeyFileError::__Nonexhaustive(())),
        }
    }
}


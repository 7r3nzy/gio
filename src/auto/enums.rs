// This file was generated by gir (4ffdbd3) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ResourceError {
    NotFound,
    Internal,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for ResourceError {
    type GlibType = ffi::GResourceError;

    fn to_glib(&self) -> ffi::GResourceError {
        match *self {
            ResourceError::NotFound => ffi::G_RESOURCE_ERROR_NOT_FOUND,
            ResourceError::Internal => ffi::G_RESOURCE_ERROR_INTERNAL,
            ResourceError::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GResourceError> for ResourceError {
    fn from_glib(value: ffi::GResourceError) -> Self {
        match value {
            ffi::G_RESOURCE_ERROR_NOT_FOUND => ResourceError::NotFound,
            ffi::G_RESOURCE_ERROR_INTERNAL => ResourceError::Internal,
        }
    }
}

impl ErrorDomain for ResourceError {
    fn domain() -> glib_ffi::GQuark {
        unsafe { ffi::g_resource_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            x if x == ffi::G_RESOURCE_ERROR_NOT_FOUND as i32 => Some(ResourceError::NotFound),
            x if x == ffi::G_RESOURCE_ERROR_INTERNAL as i32 => Some(ResourceError::Internal),
            _ => Some(ResourceError::__Nonexhaustive(())),
        }
    }
}


use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::os::raw::c_int;
use std::{borrow::Cow, str::from_utf8_unchecked};

use crate::error::DatabaseError;

#[derive(Debug)]
pub struct LibsqlHttpError {
    message: String,
}

impl LibsqlHttpError {
    pub(crate) fn new(msg: String) -> Self {
        Self {
            message
        }
    }
}

impl Display for LibsqlHttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, self.message)
    }
}

impl StdError for LibsqlHttpError {}

impl DatabaseError for LibsqlHttpError {
    /// The extended result code.
    #[inline]
    fn code(&self) -> Option<Cow<'_, str>> {
        Some(self.message.into())
    }

    #[inline]
    fn message(&self) -> &str {
        &self.message
    }

    #[doc(hidden)]
    fn as_error(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn as_error_mut(&mut self) -> &mut (dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn into_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync + 'static> {
        self
    }
}

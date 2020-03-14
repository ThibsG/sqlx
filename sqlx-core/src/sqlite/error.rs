use crate::error::DatabaseError;
use libsqlite3_sys::sqlite3_errstr;
use std::ffi::CStr;
use std::os::raw::c_int;

pub struct SqliteError {
    #[allow(dead_code)]
    code: c_int,
    message: String,
}

impl SqliteError {
    pub(crate) fn new(code: c_int) -> Self {
        #[allow(unsafe_code)]
        let message = unsafe { CStr::from_ptr(sqlite3_errstr(code)) };

        Self {
            code,
            message: message.to_string_lossy().into_owned(),
        }
    }
}

impl DatabaseError for SqliteError {
    fn message(&self) -> &str {
        &self.message
    }
}

impl_fmt_error!(SqliteError);

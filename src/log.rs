/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use crate::cstring;
use std::ffi::{c_char, CStr};
use tracing::info;

pub fn init(prefix: &str) {
    unsafe {
        cstring(prefix, |p| acfutils_sys::log_init(Some(log), p));
    }
}

extern "C" fn log(message: *const c_char) {
    unsafe {
        info!("{}", CStr::from_ptr(message).to_string_lossy());
    }
}

/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::ffi::{c_char, CString};

pub mod conf;
pub mod crc64;
pub mod glew;
pub mod log;

mod bindings;

#[allow(clippy::missing_panics_doc)]
pub fn cstring<F, R>(value: &str, f: F) -> R
where
    F: Fn(*const c_char) -> R,
{
    let c_value = CString::new(value)
        .unwrap_or_else(|e| panic!("Could not create CString from '{value}': {e}"));
    f(c_value.as_ptr())
}

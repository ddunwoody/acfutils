/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod bindings;

use nanorand::{Rng, WyRand};
use std::ffi::{c_char, CStr, CString};
use tracing::info;

pub fn glew_init() {
    unsafe {
        bindings::glewInitWrapper();
    }
}

pub fn crc64_init(seed: Option<u64>) {
    unsafe {
        acfutils_sys::crc64_init();
        let seed = seed.unwrap_or(WyRand::default().generate());
        acfutils_sys::crc64_srand(seed);
    }
}

pub fn log_init(prefix: &str) {
    unsafe {
        cstring(prefix, |p| acfutils_sys::log_init(Some(log), p));
    }
}

extern "C" fn log(message: *const c_char) {
    unsafe {
        info!("{}", CStr::from_ptr(message).to_string_lossy());
    }
}

fn cstring<F>(value: &str, f: F)
where
    F: Fn(*const c_char),
{
    let c_value = CString::new(value)
        .unwrap_or_else(|e| panic!("Could not create CString from '{value}': {e}"));
    f(c_value.as_ptr());
}

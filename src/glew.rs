/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use crate::bindings;
use std::ffi::c_ulong;

pub fn init() {
    unsafe {
        bindings::glew_init();
    }
}

pub fn dllmain_hook(reason: c_ulong) {
    unsafe {
        bindings::glew_dllmain_hook(reason);
    }
}

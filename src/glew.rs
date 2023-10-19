/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use crate::bindings;

pub fn init() {
    unsafe {
        bindings::glew_init();
    }
}

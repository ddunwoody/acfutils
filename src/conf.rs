/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use crate::cstring;
use acfutils_sys::conf_t;
use std::ptr::null_mut;

pub struct Conf {
    conf: *mut conf_t,
}

impl Conf {
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            Conf {
                conf: acfutils_sys::conf_create_empty(),
            }
        }
    }

    #[must_use]
    pub fn write_file(&self, filename: &str) -> bool {
        unsafe {
            cstring(filename, |f| {
                acfutils_sys::conf_write_file(self.conf, f) != 0
            })
        }
    }

    #[must_use]
    pub fn read_file(filename: &str) -> Option<Self> {
        let conf = unsafe { cstring(filename, |f| acfutils_sys::conf_read_file(f, null_mut())) };
        if conf.is_null() {
            None
        } else {
            Some(Conf { conf })
        }
    }

    #[must_use]
    pub fn conf_t(&self) -> *mut conf_t {
        self.conf
    }
}

impl Drop for Conf {
    fn drop(&mut self) {
        unsafe { acfutils_sys::conf_free(self.conf) }
    }
}

impl Default for Conf {
    fn default() -> Self {
        Conf::new()
    }
}

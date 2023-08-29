/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

use nanorand::{Rng, WyRand};

pub fn init(seed: Option<u64>) {
    unsafe {
        acfutils_sys::crc64_init();
        let seed = seed.unwrap_or(WyRand::default().generate());
        acfutils_sys::crc64_srand(seed);
    }
}

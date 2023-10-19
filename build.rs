/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use build_support::{get_acfutils_cflags, get_target_platform, Platform};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    println!("cargo:rerun-if-env-changed=LIBACFUTILS");

    let acfutils_path = Path::new(env!("LIBACFUTILS"));
    let xplane_sdk_path = Path::new(env!("XPLANE_SDK"));
    let platform = get_target_platform();

    #[cfg(feature = "generate-bindings")]
    generate_bindings();
    build(platform, acfutils_path, xplane_sdk_path);
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    println!("cargo:rerun-if-changed=wrapper.h");
    bindgen::Builder::default()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}

fn build(platform: Platform, acfutils_path: &Path, xplane_sdk_path: &Path) {
    println!("cargo:rerun-if-changed=wrapper.c");
    let mut builder = cc::Build::new();
    for flag in get_acfutils_cflags(platform, acfutils_path, xplane_sdk_path) {
        builder.flag(&flag);
    }
    builder.flag("-Wno-unused-parameter");
    builder.file("wrapper.c").compile("acfutils-wrapper");
}

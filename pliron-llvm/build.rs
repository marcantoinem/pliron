// SPDX-License-Identifier: Apache-2.0
// Copyright (c) The pliron contributors

fn main() {
    if std::env::var_os("CARGO_FEATURE_LINK_LIBFFI").is_some() {
        println!("cargo::rustc-link-lib=ffi");
    }
}

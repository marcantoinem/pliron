// SPDX-License-Identifier: Apache-2.0
// Copyright (c) The pliron contributors

fn main() {
    // Tell Cargo to link to libffi
    println!("cargo::rustc-link-lib=ffi");

    #[cfg(feature = "llvm-sys")]
    build_cpp_shim();
}

/// Build the C++ shim exposing the bits of LLVM's C++ API that the C API doesn't cover.
#[cfg(feature = "llvm-sys")]
fn build_cpp_shim() {
    const SHIM: &str = "cpp/intrinsic_signature.cpp";
    println!("cargo::rerun-if-changed={SHIM}");

    // llvm-sys publishes the llvm-config it settled on, so we build against the LLVM it linked.
    let llvm_config = std::env::var("DEP_LLVM_22_CONFIG_PATH")
        .expect("llvm-sys did not publish DEP_LLVM_22_CONFIG_PATH");

    let out = std::process::Command::new(&llvm_config)
        .arg("--cxxflags")
        .output()
        .unwrap_or_else(|e| panic!("failed to run `{llvm_config} --cxxflags`: {e}"));
    assert!(out.status.success(), "`{llvm_config} --cxxflags` failed");
    let cxxflags = String::from_utf8(out.stdout).expect("llvm-config emitted non-UTF-8");

    let mut build = cc::Build::new();
    build.opt_level(3);
    build.cpp(true).file(SHIM);
    for flag in cxxflags.split_whitespace() {
        // `cc` supplies its own codegen flags; we only need the include path, language version
        // and the defines LLVM's headers expect.
        if flag.starts_with("-I") || flag.starts_with("-D") || flag.starts_with("-std=") {
            build.flag(flag);
        }
    }
    build.flag_if_supported("-fno-rtti");
    build.compile("pliron_llvm_shim");
}

// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

fn main() {
    // Tell cargo to tell rustc to link the system dispatch
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=System");

    #[cfg(target_os = "linux")]
    {
        // Tell cargo to look for static libraries in the specified directory
        println!("cargo:rustc-link-search=/usr/local/lib/");
        // Tell cargo to tell rustc to link the dispatch static libraries.
        println!("cargo:rustc-link-lib=static=dispatch");
        println!("cargo:rustc-link-lib=static=BlocksRuntime");
    }
}

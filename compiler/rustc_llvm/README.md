# `rustc-llvm`

This package serves to wrap some of the LLVM functions which are not natively exposed as part of the LLVM C-API, in a Rust-friendly way. 

This code is taken directly from the [Rust compiler source code (version 1.64.0)](https://github.com/rust-lang/rust/tree/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52), which is the last version of the Rust compiler to use LLVM 14 (which is currently the version used by `ruSTy`). The Rust compiler uses this code to interface with LLVM in order to add code coverage instrumentation to Rust binaries, among other features.

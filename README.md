# RuSTy
[![Rust Build](https://github.com/PLC-lang/ruSTy/workflows/Rust%20on%20Docker/badge.svg)](https://github.com/PLC-lang/ruSTy/actions)
[![codecov](https://codecov.io/gh/PLC-lang/rusty/branch/master/graph/badge.svg?token=7ZZ5XZYE9V)](https://codecov.io/gh/PLC-lang/rusty)
[![Lines of Code](https://tokei.rs/b1/github/PLC-lang/rusty)](https://github.com/XAMPPRocky/tokei)

[Structured text](https://en.wikipedia.org/wiki/Structured_text) compiler written in Rust

## About RuSTy
RuSTy is a structured text (ST) compiler written in Rust. RuSTy utilizes the
LLVM framework to compile eventually to native code. 

## Getting started
The easiest way to compile this project is to use the provided `Dockerfile`. The project offers a `.devcontainer` when using [VSCode](https://code.visualstudio.com/docs/remote/containers). The Dockerfile offers a linux-image which contains everything you need to run `cargo build` / `cargo test` in the project's root directory.

If you want to build the project without docker, start [here](https://plc-lang.github.io/rusty/build_and_install.html).

### Documentation
The compiler's documentation can be found here: [documentation](https://plc-lang.github.io/rusty/).

### Contributing
If you want to contribute to the project you should look for some [beginner-friendly issues](https://github.com/PLC-lang/rusty/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) and reach out to project's maintainers.

## Why RuSTy
Structured Text is a popular langauge in the domain of automation. A standardized specification of the language ([IEC 61131](https://en.wikipedia.org/wiki/IEC_61131))  was published in the 90s. It was updated several times in the meantime, while its initial spirit - beeing built for cyclic, robust and deterministic automation applications - still applies. 

Several automation plattforms suppliers built proprietary compilers and runtime libraries, native to the vendor's hard- and software platform.

RuSTy is aiming towards a *fast*, *modern* and *open-source* industry-grade ST compiler for a wide range of plattforms, sticking close to the standard.

## Dependencies
We use the [_logos_](https://crates.io/crates/logos/0.8.0)
crate library to perform lexical analysis before a handwritten recursive decent parser creates the AST. 
Generating LLVM IR is accomplished with the help of [_inkwell_](https://github.com/TheDan64/inkwell), a Rust-wrapper for the native LLVM C++ API.


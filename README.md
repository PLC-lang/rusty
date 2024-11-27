
<div align="center">
<!-- Eventually(?) we'll have a logo, which we can insert here -->
<h1>RuSTy</h1>
<p>A <a href="https://en.wikipedia.org/wiki/Structured_text">structured text</a> compiler written in Rust, utilizing the LLVM framework for native code compilation.</p>

<a href="https://github.com/PLC-lang/ruSTy/actions/workflows/linux.yml"><img src="https://github.com/PLC-lang/rusty/actions/workflows/linux.yml/badge.svg"/></a>
<a href="https://github.com/PLC-lang/ruSTy/actions/workflows/windows.yml"><img src="https://github.com/PLC-lang/rusty/actions/workflows/windows.yml/badge.svg"/></a>
<a href="https://plc-lang.github.io/metrics"><img src="https://github.com/PLC-lang/rusty/actions/workflows/metrics.yml/badge.svg"/></a>
<a href="https://codecov.io/gh/PLC-lang/rusty"><img src="https://codecov.io/gh/PLC-lang/rusty/branch/master/graph/badge.svg?token=7ZZ5XZYE9V"/></a>
<a href="https://github.com/XAMPPRocky/tokei"><img src="https://tokei.rs/b1/github/PLC-lang/rusty"/></a>

<p>
    <a href="https://github.com/PLC-lang/rusty/tree/master/examples">Examples</a> | 
    <a href="https://plc-lang.github.io/rusty/">Documentation</a> | 
    <a href="https://github.com/PLC-lang/rusty/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22">Contributing</a>
</p>

</div>


## Why RuSTy?

Structured Text is a popular language in the domain of automation. A standardized specification of the language ([IEC 61131](https://en.wikipedia.org/wiki/IEC_61131)) was published in the 90s. It was updated several times in the meantime, while its initial spirit - being built for cyclic, robust and deterministic automation applications - still applies.

Several automation platform suppliers built proprietary compilers and runtime libraries, native to the vendor's hard- and software platform.

RuSTy is aiming towards a **fast**, **modern** and **open-source** industry-grade ST compiler for a wide range of platforms, sticking close to the standard.

## Getting started

The easiest way to compile this project is to use the provided `Dockerfile`. The project offers a `.devcontainer` when using [VSCode](https://code.visualstudio.com/docs/remote/containers). The Dockerfile offers a linux-image which contains everything you need to run `cargo build` / `cargo test` in the project's root directory.

If you want to build the project without docker, start [here](https://plc-lang.github.io/rusty/build_and_install.html).
# Building and Installing

## Prerequisites
To be able to build the source code, you will need to [install Rust](https://www.rust-lang.org/tools/install)
and the following dependencies:
* the usual **standard build tools** (aka `build-essential`)
* **LLVM 12**: On Ubuntu, the package manager version of LLVM (e.g. `llvm-12-dev`, `liblld-12-dev`) will work fine,
on debian you'll need to add additional repository sources (`deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-12 main`), since Debian 11 (latest) only includes LLVM packages up to version 11. For Windows, you need a
[special build](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v12.0.1).
* **zlib** (apt: `libz-dev`)
* **Polly** in the form of a static library (included in apt package `libclang-common-12-dev`). Alternatively,
building LLVM from source should also provide you with that file. Building from source may take a while, though.
* If you want to clone and work on the repository, you'll also need **git**.

### Tips for troubleshooting
* Because of weak compatibility guarantees of the LLVM API, the LLVM installation must exactly match the
major version of the `llvm-sys` crate. Currently you will need to install LLVM 12 to satisfy this constraint.
[Read more](https://crates.io/crates/llvm-sys)
* To avoid installation conflicts on Linux/Ubuntu, make sure you don't have a default installation available
(like you get by just installing `llvm-dev`), which may break things. If you do, make sure you have set
the appropriate environment variable (`LLVM_SYS_120_PREFIX=/usr/lib/llvm-12` for LLVM 12), so
the build of the `llvm-sys` crate knows what files to grab.

## Cloning the repository
On your local computer, open up a shell and clone the repository.
```bash
git clone https://github.com/PLC-lang/rusty
cd rusty
```

## Building
Building is as easy as typing a single command. If you just want to use the
compiler without doing development, building the release configuration will
give you a smaller and likely a faster binary.
```bash
cargo build --release
```

You can find the binary at `./target/release/rustyc`.

## Installing
_TODO_

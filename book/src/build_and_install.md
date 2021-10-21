# Building and Installing

## Prerequisites
To be able to build the source code, you will need to [install Rust](https://www.rust-lang.org/tools/install)
and LLVM 11, along with the standard build tools (`build-essential`) and `libz-dev` on your machine.
For Linux the package manager version of LLVM (e.g. `llvm-11-dev`, `liblld-11-dev` for apt) will work fine, for Windows, you need a
[special build](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v11.0.1). If you want to
clone and work on the repository, you'll also need _git_.

### Tips for troubleshooting
* Because of weak compatibility guarantees of the LLVM API, the LLVM installation must exactly match the
major version of the `llvm-sys` crate. Currently you will need to install LLVM 11 to satisfy this constraint.
[Read more](https://crates.io/crates/llvm-sys)
* To avoid installation conflicts on Linux/Ubuntu, make sure you don't have a default installation available
(like just installing `llvm-dev`), which may break things. If you do, make sure you have set
the appropriate environment variable (`LLVM_SYS_110_PREFIX=/usr/lib/llvm-11` for LLVM 11), so
the build of the `llvm-sys` crate knows what files to grab.
* If you get an error stating that the `native static library 'Polly'` cannot be found, you might want to
install the `libclang-common-11-dev` package, which includes this library in the form of a static library.
Alternatively, building LLVM from source should also provide you with that file. Building from source may
take a while, though.

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

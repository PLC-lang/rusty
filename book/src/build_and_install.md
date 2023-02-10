# Build & Install

RuSTys code can be found on [GitHub](https://github.com/PLC-lang/rusty).
By default a `Dockerfile` and a `devcontainer.json` file are provided. If you wish to develop natively 
however, you will need some additional dependencies namely:

- [Rust](https://www.rust-lang.org/tools/install)
- LLVM 14
- LLVM Polly
- Build Tools (e.g. `build-essential` on Ubuntu)
- zlib

The next sections cover how to install these dependencies on different platforms, if you already have them
however, RuSTy can be build using the `cargo` command. For debug builds this can be accomplished by executing
`cargo build` and for release builds (smaller & faster) you would execute `cargo build --release`. The 
resulting binaries can be found at `target/debug/rustyc` and `target/release/rustyc` respectively.

## Ubuntu

The specified dependencies can be installed with the following command on Ubuntu :

```bash
sudo apt install                \
    build-essential             \
    llvm-14-dev liblld-14-dev   \
    libz-dev                    \
    libclang-common-14-dev 
```
Additionally you _might_ need `libffi7`, which can be installed with `sudo apt install libffi7`.

## Debian

Same as Ubuntu with the exception of adding additional repository sources since Debian 11 only includes LLVM packages up to version 11.
To do so follow the [official documentation](https://apt.llvm.org/).

## MacOS

On MacOS you need to install the [`Xcode Command Line Tools`](https://developer.apple.com/downloads/).

Furthermore LLVM 14 is needed, which can be easily installed with [homebrew](https://brew.sh) :

```bash
brew install llvm@14
````

After the installation you have to add `/opt/homebrew/opt/llvm@14/bin` to your `$PATH` environment variable, e.g. with the following command :

```bash
echo 'export PATH="/opt/homebrew/opt/llvm@14/bin:$PATH"' >> ~/.zshrc
```

## Windows

For Windows you will need a [custom build](https://github.com/plc-lang/llvm-package-windows/releases/tag/v14.0.6).
## Installing

_TODO_

## Troubleshooting

- Because of weak compatibility guarantees of the LLVM API, the LLVM installation must exactly match the
major version of the `llvm-sys` crate.Currently you will need to install LLVM 14 to satisfy this constraint.
[Read more](https://crates.io/crates/llvm-sys)
- To avoid installation conflicts on Linux/Ubuntu, make sure you don't have a default installation available
(like you get by just installing `llvm-dev`), which may break things. If you do, make sure you have set
the appropriate environment variable (`LLVM_SYS_140_PREFIX=/usr/lib/llvm-14` for LLVM 14), so
the build of the `llvm-sys` crate knows what files to grab.
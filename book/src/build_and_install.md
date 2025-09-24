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
resulting binaries can be found at `target/debug/plc` and `target/release/plc` respectively.

## Ubuntu (24.04)

The specified dependencies can be installed with the following command on Ubuntu:

```bash
sudo apt install                \
    build-essential             \
    llvm-14-dev liblld-14-dev   \
    libz-dev                    \
    lld                         \
    libclang-common-14-dev      \
    libpolly-14-dev
```
Additionally you _might_ need `libffi7`, which can be installed with `sudo apt install libffi7`.
__Note__: On other Ubuntu versions, the required packages _may_ differ.

## Debian

Same as Ubuntu with the exception of adding additional repository sources since Debian 11 only includes LLVM packages up to version 11.
To do so follow the [official documentation](https://apt.llvm.org/).

## MacOS

On MacOS you need to install the [`Xcode Command Line Tools`](https://developer.apple.com/downloads/) and the LLVM toolchain using [Homebrew](https://brew.sh):

```bash
brew install llvm@14
```

While optional, it is strongly recommended to also install `gnu-getopt` and `lit` so the helper scripts and integration tests work without extra steps:

```bash
brew install gnu-getopt lit
```

`gnu-getopt` is required by `./scripts/build.sh`, and `lit` is needed when running the integration tests via `./scripts/build.sh --lit`.

After the installation, make sure the Homebrew binaries are discoverable for both LLVM and GNU getopt:

```bash
echo 'export PATH="/opt/homebrew/opt/llvm@14/bin:$PATH"' >> ~/.zshrc
echo 'export PATH="/opt/homebrew/opt/gnu-getopt/bin:$PATH"' >> ~/.zshrc
```

The `lit` test suite expects `FileCheck-14` to be available. If `FileCheck-14` is not already present, create a symlink to the `FileCheck` binary:

```bash
ln -svf /opt/homebrew/opt/llvm@14/bin/FileCheck /opt/homebrew/opt/llvm@14/bin/FileCheck-14
```

## Windows
Compiling RuSTy on Windows requires three dependencies:
1. Windows 10 SDK
2. MSVC (at the point of writing this we tested it on v142 - VS 2019 C++ x64/x86 build tools)
3. [LLVM 14.0.6](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v14.0.6)

The first two dependencies are typically installed during the Rust installation itself. More specifically during the
installation you should have been prompted to install them. If not, you'll be able to install them via Visual Studio at any point.
The third dependency is based on a custom build which is hosted on [GitHub](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v14.0.6).
Download it, extract it and add the `bin/` directory to your [environment variables](https://docs.oracle.com/en/database/oracle/machine-learning/oml4r/1.5.1/oread/creating-and-modifying-environment-variables-on-windows.html).
In theory this should cover everything to be able to compile RuSTy (with some reboots here and there).

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

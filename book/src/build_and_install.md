# Building and Installing
RuSTys code can be found on [GitHub](https://github.com/PLC-lang/rusty). By default a `Dockerfile` and a `devcontainer.json` file are provided. If you wish to develop natively however, you will need some additional dependencies namely
- [Rust](https://www.rust-lang.org/tools/install)
- LLVM 13
- LLVM Polly
- Build Tools (e.g. `build-essential` on Ubuntu)
- zlib

The next sections will cover how to install these dependencies on different platforms.
## Ubuntu
The specified dependencies can be installed with the following command on Ubuntu
```bash
sudo apt install                \
    build-essential             \
    llvm-13-dev liblld-13-dev   \
    libz-dev                    \
    libclang-common-13-dev 
```

## Debian
Same as Ubuntu with the exception of adding additional repository sources since Debian 11 only includes LLVM packages up to version 11. To do so follow the [official documentation](https://apt.llvm.org/).

## MacOS
On MacOS you need to install the [`Xcode Command Line Tools`](https://developer.apple.com/downloads/). Furthermore LLVM 13 is needed, which can be easily installed with [homebrew](https://brew.sh)
```bash
brew install llvm@13
````
After the installation you have to add `/opt/homebrew/opt/llvm@13/bin` to your `$PATH` environment variable, e.g. with the following command
```bash
echo 'export PATH="/opt/homebrew/opt/llvm@13/bin:$PATH"' >> ~/.bashrc
```

## Windows
For Windows you will need a [custom build](https://github.com/plc-lang/llvm-package-windows/releases/tag/v13.0.0).

## Troubleshooting
* Because of weak compatibility guarantees of the LLVM API, the LLVM installation must exactly match the
major version of the `llvm-sys` crate. Currently you will need to install LLVM 13 to satisfy this constraint.
[Read more](https://crates.io/crates/llvm-sys)
* To avoid installation conflicts on Linux/Ubuntu, make sure you don't have a default installation available
(like you get by just installing `llvm-dev`), which may break things. If you do, make sure you have set
the appropriate environment variable (`LLVM_SYS_130_PREFIX=/usr/lib/llvm-13` for LLVM 13), so
the build of the `llvm-sys` crate knows what files to grab.

## Building
Just like any Rust project binaries can be built with `cargo build`. For release builds, i.e. faster and smaller binaries, you have to pass the `--release` flag, like so `cargo build --release`. The resulting binaries can be found at `target/release/rusty`

## Improving Compile Times
By default Rust uses the GNU Linker on Linux which compared to [lld](https://lld.llvm.org/) is slower by a margin of [~2x - 4x](https://llvm.org/devmtg/2016-10/slides/Ueyama-lld.pdf). To improve compile times we can therefore use `lld`. To do so you will need to run the `rusty/scripts/lld.sh` script inside the `rusty` root folder, i.e. by executing `./scripts/lld.sh`. **Note** that the script was only tested on Ubuntu based distributions thus far.


## Installing
_TODO_

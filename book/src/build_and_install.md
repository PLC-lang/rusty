# Building and Installing
RuSTy by default provides a `Dockerfile` as well as a `devcontainer.json` for easier development on VSCode. If you wish to develop natively, you will need some additional dependencies based on your OS.

## Ubuntu
Besides [Rust](https://www.rust-lang.org/tools/install) you will have to install the following dependencies to develop on Ubuntu
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
On MacOS the most straightforward approach is to install LLVM 13 with [homebrew](https://brew.sh) 
```bash
brew install llvm@13
````
After the installation you will need to add `/opt/homebrew/opt/llvm@13/bin` to your `$PATH` environment variable, e.g. with the following command
```bash
echo 'export PATH="/opt/homebrew/opt/llvm@13/bin:$PATH"' >> ~/.bashrc
```

## Windows
For Windows you will need a [special build](https://github.com/plc-lang/llvm-package-windows/releases/tag/v13.0.0).

## Troubleshooting
* Because of weak compatibility guarantees of the LLVM API, the LLVM installation must exactly match the
major version of the `llvm-sys` crate. Currently you will need to install LLVM 13 to satisfy this constraint.
[Read more](https://crates.io/crates/llvm-sys)
* To avoid installation conflicts on Linux/Ubuntu, make sure you don't have a default installation available
(like you get by just installing `llvm-dev`), which may break things. If you do, make sure you have set
the appropriate environment variable (`LLVM_SYS_130_PREFIX=/usr/lib/llvm-13` for LLVM 13), so
the build of the `llvm-sys` crate knows what files to grab.

## Cloning RuSTy
On your local computer, open up a shell and execute the following commands
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

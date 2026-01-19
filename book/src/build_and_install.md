# Build & Install

We provide a [`devcontainer.json`](https://containers.dev/) and a `Dockerfile` for development. For native development, refer to the following sections.

## Ubuntu 24.04

```bash
# Install pre-requisites
sudo apt install lsb-release wget software-properties-common gnupg build-essential zlib1g-dev libzstd-dev lld clang

# Install Rust, see https://rust-lang.org/tools/install/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install LLVM 21, see https://apt.llvm.org/
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
./llvm.sh 21 && sudo apt install libpolly-21-dev

# Install uv, see https://docs.astral.sh/uv/getting-started/installation/
# (Optional, but required for development)
curl -LsSf https://astral.sh/uv/install.sh | sh
uv tool install lit
```

## Debian Trixie

Follow the Ubuntu 24.04 instructions, but omit `software-properties-common` which is not available on Debian.

## macOS

Install the [Xcode Command Line Tools](https://developer.apple.com/downloads/) and the LLVM toolchain using [Homebrew](https://brew.sh):

```bash
brew install llvm@21 gnu-getopt lit
```

After installation, ensure the Homebrew binaries are in your PATH:

```bash
echo 'export PATH="/opt/homebrew/opt/llvm@21/bin:$PATH"' >> ~/.zshrc
echo 'export PATH="/opt/homebrew/opt/gnu-getopt/bin:$PATH"' >> ~/.zshrc
```

The `lit` test suite expects `FileCheck-21` to be available. If not present, create a symlink:

```bash
ln -svf /opt/homebrew/opt/llvm@21/bin/FileCheck /opt/homebrew/opt/llvm@21/bin/FileCheck-21
```

## Windows

Install [Rust](https://www.rust-lang.org/tools/install) and download the appropriate LLVM version from https://github.com/PLC-lang/llvm-package-windows/releases/. Extract it and add the `bin/` directory to your PATH.

## Troubleshooting

- The LLVM installation must exactly match the major version of the `llvm-sys` crate due to LLVM's API compatibility guarantees. Currently LLVM 21 is required. See https://crates.io/crates/llvm-sys

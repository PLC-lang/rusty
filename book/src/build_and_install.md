# Build & Install

We provide a [dev container](https://containers.dev/) for development. For native development, refer to the sections after it.

## Dev Container

Open the repository in [VS Code](https://code.visualstudio.com/docs/devcontainers/containers) and choose "Reopen in Container". The container is defined in `.devcontainer/` and is based on Ubuntu 26.04.

The container runs with a read-only root filesystem and without `sudo`. The workspace is mounted at `/workspace`, and the home directory and the `target/` directory live on named volumes, so builds and tools installed with `cargo install` persist across container rebuilds. The `.devcontainer/` directory is mounted read-only.

## Ubuntu 26.04

Ubuntu 26.04 ships LLVM 21 in its own package archive, so there is no need for the LLVM apt repository:

```bash
# Install pre-requisites
sudo apt install build-essential clang lld zlib1g-dev libzstd-dev llvm-21-dev llvm-21-tools libpolly-21-dev

# Install Rust, see https://rust-lang.org/tools/install/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# lit is shipped with llvm-21-tools, make it available as `lit`
sudo ln -s /usr/lib/llvm-21/bin/lit /usr/local/bin/lit
```

## Ubuntu 24.04

```bash
# Install pre-requisites
sudo apt install lsb-release wget software-properties-common gnupg build-essential zlib1g-dev libzstd-dev lld clang

# Install Rust, see https://rust-lang.org/tools/install/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install LLVM 21, see https://apt.llvm.org/
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 21 && sudo apt install libpolly-21-dev

# Install uv, see https://docs.astral.sh/uv/getting-started/installation/
# (Optional, but required for development)
curl -LsSf https://astral.sh/uv/install.sh | sh
source $HOME/.local/bin/env
uv tool install lit
```

## Debian Trixie

Follow the Ubuntu 24.04 instructions, but omit `software-properties-common` which is not available on Debian.

## macOS

Install the [Xcode Command Line Tools](https://developer.apple.com/downloads/) and the LLVM toolchain using [Homebrew](https://brew.sh):

```bash
brew install llvm@21 lld gnu-getopt lit
```

After installation, ensure the Homebrew binaries are in your PATH:

```bash
echo 'export PATH="/opt/homebrew/opt/llvm@21/bin:$PATH"' >> ~/.zprofile
echo 'export PATH="/opt/homebrew/opt/gnu-getopt/bin:$PATH"' >> ~/.zprofile
```

The `lit` test suite expects `FileCheck-21` to be available. If not present, create a symlink:

```bash
ln -svf /opt/homebrew/opt/llvm@21/bin/FileCheck /opt/homebrew/opt/llvm@21/bin/FileCheck-21
```

## Windows

Install [Rust](https://www.rust-lang.org/tools/install) and download the appropriate LLVM version from https://github.com/PLC-lang/llvm-package-windows/releases/. Extract it and add the `bin/` directory to your PATH.

Note: Rust will require C++ Build Tools. The recommended way is to install them with a full Visual Studio installation. However, a much faster and easier way is to install the build tools directly. Head over to https://visualstudio.microsoft.com/downloads/, scroll down to "Tools for Visual Studio" and download the binary (https://aka.ms/vs/stable/vs_BuildTools.exe). 

## Troubleshooting

- The LLVM installation must exactly match the major version of the `llvm-sys` crate due to LLVM's API compatibility guarantees. Currently LLVM 21 is required. See https://crates.io/crates/llvm-sys

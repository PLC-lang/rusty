# Install

There are two ways to get the compiler. Install a release if you only want to compile Structured Text. Build from source if you want to work on the compiler itself.

The compiler binary is called `plc`.


## Install a release

Every release publishes its assets on the [releases page](https://github.com/PLC-lang/rusty/releases).

| Asset | Contents |
|---|---|
| `plc-linux-x86_64`, `plc-linux-aarch64` | The compiler binary for Linux |
| `plc-compiler_<version>-1_<arch>.deb` | The compiler as a Debian package |
| `plc-stdlib_<version>-1_<arch>.deb` | The standard library as a Debian package |
| `stdlib-<version>.tar.gz` | The standard library with its declaration files |
| `plc.exe` | The compiler binary for Windows |
| `iec61131std.dll`, `iec61131std.lib` | The standard library for Windows |

### Debian and Ubuntu

Install both packages together, so that the standard library is available to your programs:

```bash
sudo apt install ./plc-compiler_<version>-1_amd64.deb ./plc-stdlib_<version>-1_amd64.deb
```

The packages install these files:

| File | Purpose |
|---|---|
| `/usr/bin/plc` | The compiler |
| `/usr/share/plc/schema/plc-json.schema` | The schema of the project file |
| `/usr/lib/<triplet>/libiec61131std.so`, `libiec61131std.a` | The standard library |
| `/usr/share/plc/include/*.st` | The declarations of the standard library |

### Other systems

Copy the binary for your platform into a directory in your `PATH`, and make it executable.

LLVM is linked into the compiler binary, so you do not install LLVM to use a release.

`plc` does need a linker on the system to produce executables and shared objects. Use the compiler driver of the system: `cc` on Linux, and `clang` on macOS and Windows.


## Verify the installation

```bash
plc --version
```

The command prints the version, the commit date, and the commit hash of the binary.


## Build from source

Building from source needs Rust and a full LLVM installation.

> [!NOTE]
> The LLVM installation must match the major version that the compiler is built against. This is LLVM 21. LLVM gives no API compatibility between major versions, so another version does not link.

### Dev container

A [dev container](https://containers.dev/) is the shortest way to a complete environment. Open the repository in [VS Code](https://code.visualstudio.com/docs/devcontainers/containers) and choose "Reopen in Container". The container is defined in `.devcontainer/` and is based on Ubuntu 26.04.

The container runs with a read-only root filesystem and without `sudo`. The workspace is mounted at `/workspace`. The home directory and the `target/` directory live on named volumes, so builds and tools installed with `cargo install` survive a rebuild of the container.

### Ubuntu 26.04

Ubuntu 26.04 ships LLVM 21 in its own package archive, so the LLVM apt repository is not necessary:

```bash
# Install the prerequisites
sudo apt install build-essential clang lld zlib1g-dev libzstd-dev llvm-21-dev llvm-21-tools libpolly-21-dev

# Install Rust, see https://rust-lang.org/tools/install/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# lit is shipped with llvm-21-tools, make it available as `lit`
sudo ln -s /usr/lib/llvm-21/bin/lit /usr/local/bin/lit
```

### Ubuntu 24.04

```bash
# Install the prerequisites
sudo apt install lsb-release wget software-properties-common gnupg build-essential zlib1g-dev libzstd-dev lld clang

# Install Rust, see https://rust-lang.org/tools/install/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install LLVM 21, see https://apt.llvm.org/
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 21 && sudo apt install libpolly-21-dev

# Install uv, see https://docs.astral.sh/uv/getting-started/installation/
# (only necessary to run the test suite)
curl -LsSf https://astral.sh/uv/install.sh | sh
source $HOME/.local/bin/env
uv tool install lit
```

### Debian Trixie

Use the instructions for Ubuntu 24.04, but remove `software-properties-common`, which Debian does not have.

### macOS

Install the [Xcode Command Line Tools](https://developer.apple.com/downloads/) and the LLVM toolchain with [Homebrew](https://brew.sh):

```bash
brew install llvm@21 lld gnu-getopt lit
```

Then put the Homebrew binaries in your `PATH`:

```bash
echo 'export PATH="/opt/homebrew/opt/llvm@21/bin:$PATH"' >> ~/.zprofile
echo 'export PATH="/opt/homebrew/opt/gnu-getopt/bin:$PATH"' >> ~/.zprofile
```

The `lit` test suite expects `FileCheck-21`. If it is not there, make a symbolic link:

```bash
ln -svf /opt/homebrew/opt/llvm@21/bin/FileCheck /opt/homebrew/opt/llvm@21/bin/FileCheck-21
```

### Windows

Install [Rust](https://www.rust-lang.org/tools/install) and the matching LLVM build from the [llvm-package-windows releases](https://github.com/PLC-lang/llvm-package-windows/releases). Extract it and add its `bin/` directory to your `PATH`.

Rust needs the C++ build tools. A full Visual Studio installation gives them, but the [build tools alone](https://aka.ms/vs/stable/vs_BuildTools.exe) are smaller and faster to install.

### Build

```bash
cargo build --release
```

The binary is written to `target/release/plc`.


## Troubleshooting

| Symptom | Cause |
|---|---|
| The build fails while it links LLVM | The installed LLVM is not version 21. Check with `llvm-config --version` |
| `plc` reports that it cannot find a linker | No linker on the system. Install `clang` or `lld`, or name one with `--linker` |
| A program does not link against standard functions | The standard library is missing. Install the `plc-stdlib` package, or link it with `-l iec61131std` |


## What's next

The compiler is ready. Write your first program in the [next chapter](hello-world.md).

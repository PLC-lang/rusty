#!/usr/bin/env sh

LD_LLD_WITH_VERSION=$(command -v ld.lld-$1)
LD_LLD_WITHOUT_VERSION=$(command -v ld.lld)

if [ -z $1 ]; then
    echo "No version specified!"
    echo "Usage:   $0 <LLD version>"
    echo "Example: $0 13"
    exit 1
fi

# Check if the specified `lld` version is present on the system
if [ -z $LD_LLD_WITH_VERSION ]; then
    echo "Could not find ld.lld-$1 binary, aborting"
    exit 1
fi

# Check if `ld.lld` is present on the system
if [ -z $LD_LLD_WITHOUT_VERSION ]; then 
    # Not present, create a `/usr/bin/ld.lld -> /usr/bin/ld.lld-$1` symlink
    sudo ln -sf $LD_LLD_WITH_VERSION /usr/bin/ld.lld
else 
    # Present, do nothing because we don't want to modifiy system-wide configurations
    echo "Note: Using already present $LD_LLD_WITHOUT_VERSION binary"
    echo "      Make sure $LD_LLD_WITHOUT_VERSION matches with RuSTys LLVM version"
fi

# Create `config.toml` to make `lld` the default linker for RuSTy
mkdir -p .cargo
echo -n '[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]' > .cargo/config.toml

# Makes sure new builds with lld succeed
cargo clean
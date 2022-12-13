#!/usr/bin/env sh

TEXT_BOLD=$(tput bold)
TEXT_NORMAL=$(tput sgr0)

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
    echo "Could not find ld.lld-$1, make sure lld is installed"
    exit 1
fi

# Check if `ld.lld` is present on the system
if [ -z $LD_LLD_WITHOUT_VERSION ]; then 
    # Not present, create a symlink into ~/.local/bin
    mkdir -p $HOME/.local/bin
    ln -svf $LD_LLD_WITH_VERSION $HOME/.local/bin/ld.lld

    # Append new $PATH to `.bashrc`
    echo 'export PATH=$HOME/.local/bin:$PATH' >> $HOME/.bashrc
    echo 'A new $PATH entry has been added to your .bashrc'
    echo 'For changes to take effect close and reopen your current shell.'

else 
    # Present, do nothing because we don't want to modifiy system-wide configurations
    echo "Note: Using already present $LD_LLD_WITHOUT_VERSION binary"
fi

# Create `config.toml` to make `lld` the default linker for RuSTy
mkdir -p .cargo
echo -n '[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]' > .cargo/config.toml

# Makes sure new builds with lld succeed
cargo clean
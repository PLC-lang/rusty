#!/usr/bin/env sh

if [ -z $1 ]; then
    echo "Usage:   $0 <LLD version>"
    echo "Example: $0 13"
    exit 1
fi

# Check if the specified LLD version is installed
if ! [ -x "$(command -v ld.lld-$1)" ]; then
    echo "LLD-$1 was not found!"
    exit 1
fi

# Symlink `lld` to `ld.lld` which is needed by `.cargo/config.toml`
sudo ln -svf $(command -v ld.lld-$1) /usr/bin/ld.lld

# Create the `.cargo/config.toml` file
mkdir -p .cargo
echo -n '[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]' > .cargo/config.toml

cargo clean
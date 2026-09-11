# Ubuntu 26.04 LTS, pinned by digest
FROM docker.io/library/ubuntu:26.04@sha256:2260313b31c8c011cd2eebe728008efac1b3982be73eb71348ea2648d2c0e09b

ARG DEBIAN_FRONTEND=noninteractive
ARG LLVM_VERSION=21
ARG RUST_VERSION=1.95.0

USER root

# Build dependencies, see book/src/build_and_install.md. Ubuntu 26.04 ships LLVM 21 in its own archive.
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential clang lld zlib1g-dev libzstd-dev \
    llvm-${LLVM_VERSION}-dev llvm-${LLVM_VERSION}-tools libpolly-${LLVM_VERSION}-dev lldb \
    git openssh-client curl wget ca-certificates gnupg \
    lsb-release software-properties-common python3 procps vim \
    && rm -rf /var/lib/apt/lists/*

# Rust toolchain
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --profile minimal --default-toolchain ${RUST_VERSION} -c clippy -c rustfmt -c rust-src
RUN cargo install --locked cargo-insta && rm -rf /usr/local/cargo/registry /usr/local/cargo/git

# lit test runner shipped with the llvm tools package
RUN ln -s /usr/lib/llvm-${LLVM_VERSION}/bin/lit /usr/local/bin/lit

# The root filesystem is read-only at runtime: the cargo registry, Claude Code and other user installed
# tools live in the home volume, and the volume mount points must be owned by the container user.
ENV CARGO_HOME=/home/ubuntu/.cargo
ENV PATH=/home/ubuntu/.cargo/bin:/home/ubuntu/.local/bin:$PATH

RUN mkdir -p /workspace/target && chown -R ubuntu:ubuntu /workspace

# Colored prompt and tool output in shells started with docker exec, which sets no terminal type
ENV TERM=xterm-256color
ENV COLORTERM=truecolor

# Lock down the container user: no setuid binaries, locked password, no telemetry
ENV DISABLE_TELEMETRY=1

RUN find / -xdev -perm /6000 -type f -exec chmod a-s {} +
RUN passwd -l ubuntu

WORKDIR /workspace
USER ubuntu

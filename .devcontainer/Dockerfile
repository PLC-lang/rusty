FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG LLVM_VERSION=21

# Install the native build dependencies and tools used inside the development container.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        clang \
        curl \
        git \
        gnupg \
        libzstd-dev \
        lld \
        lsb-release \
        software-properties-common \
        sudo \
        vim \
        wget \
        zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Install the LLVM toolchain required by llvm-sys.
RUN wget --quiet https://apt.llvm.org/llvm.sh --output-document=/tmp/llvm.sh \
    && chmod +x /tmp/llvm.sh \
    && /tmp/llvm.sh "${LLVM_VERSION}" \
    && apt-get install -y --no-install-recommends "libpolly-${LLVM_VERSION}-dev" \
    && rm /tmp/llvm.sh \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Configure the image's non-root account for development and install Starship globally.
RUN echo "ubuntu ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/ubuntu \
    && chmod 0440 /etc/sudoers.d/ubuntu \
    && curl -sS https://starship.rs/install.sh | sh -s -- --yes

USER ubuntu
ENV HOME=/home/ubuntu
ENV PATH="${HOME}/.cargo/bin:${HOME}/.local/bin:${PATH}"

# Install the latest stable Rust toolchain.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install the Claude Code development harness.
RUN curl -fsSL https://claude.ai/install.sh | bash

# Install the development test runner.
RUN curl -LsSf https://astral.sh/uv/install.sh | sh \
    && uv tool install lit

# Install the snapshot testing CLI.
RUN cargo install cargo-insta

# Configure Claude Code and Starship for interactive Bash sessions.
RUN printf '\n# Run Claude Code without permission prompts.\nalias claude="claude --dangerously-skip-permissions"\n\n# Initialize the Starship prompt.\neval "$(starship init bash)"\n' >> "${HOME}/.bashrc"

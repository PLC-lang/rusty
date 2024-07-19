# FROM mcr.microsoft.com/vscode/devcontainers/rust:latest
FROM ghcr.io/plc-lang/rust-llvm:latest

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# This Dockerfile adds a non-root user with sudo access. Use the "remoteUser"
# property in devcontainer.json to use it. On Linux, the container user's GID/UIDs
# will be updated to match your local UID/GID (when using the dockerFile property).
# See https://aka.ms/vscode-remote/containers/non-root-user for details.
# ARG USERNAME=vscode
# ARG USER_UID=1000
# ARG USER_GID=$USER_UID

# # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
# RUN groupadd --gid $USER_GID $USERNAME \
# && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
# # [Optional] Add sudo support for the non-root user
# && apt-get install -y sudo \
# && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
# && chmod 0440 /etc/sudoers.d/$USERNAME 

# RUN apt-get -y update
# RUN apt-get -y install git gdb docker.io

# RUN cargo install cargo-insta cargo-watch

# Give all users access to cargo and rust home
RUN chmod -R a+rw $CARGO_HOME \
    && chmod -R a+rw $RUSTUP_HOME

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog
ENV LLVM_VER=14

# Required if we want to use `lld` as the default linker for RuSTy
RUN ln -sf /usr/bin/ld.lld-$LLVM_VER /usr/bin/ld.lld

# Install the local RuSTy version
WORKDIR /rusty
COPY . .
RUN sed -i 's/build=0/build=1/' ./scripts/build.sh && \
    ./scripts/build.sh

# Allow invoking `plc` from anywhere
ENV PATH="/rusty/target/debug:${PATH}"

ENTRYPOINT [ "/bin/bash", "-c" ]
CMD ["plc", "--help"]

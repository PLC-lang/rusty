# FROM mcr.microsoft.com/vscode/devcontainers/rust:latest
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# # This Dockerfile adds a non-root user with sudo access. Use the "remoteUser"
# # property in devcontainer.json to use it. On Linux, the container user's GID/UIDs
# # will be updated to match your local UID/GID (when using the dockerFile property).
# # See https://aka.ms/vscode-remote/containers/non-root-user for details.
# ARG USERNAME=vscode
# ARG USER_UID=1000
# ARG USER_GID=$USER_UID

RUN apt-get update && apt-get install -y vim git


ARG LLVM_VER=13
# Use the bullseye llvm version because there is no newer one yet
ARG LLVM_DISTRO_VERSION=bullseye 

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive
# Setup llvm sources
RUN echo "deb http://apt.llvm.org/$LLVM_DISTRO_VERSION/ llvm-toolchain-$LLVM_DISTRO_VERSION-$LLVM_VER  main" >> /etc/apt/sources.list.d/llvm.list
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

RUN apt-get update
#Install Clang dependencies
RUN apt-get install -y zip clang-$LLVM_VER lldb-$LLVM_VER lld-$LLVM_VER clangd-$LLVM_VER liblld-$LLVM_VER-dev llvm-$LLVM_VER-dev

#Install documentation and coverage tools
RUN cargo install mdbook grcov

# # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
# RUN groupadd --gid $USER_GID $USERNAME \
# && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
# # [Optional] Add sudo support for the non-root user
# && apt-get install -y sudo \
# && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
# && chmod 0440 /etc/sudoers.d/$USERNAME 

RUN cargo install cargo-insta cargo-watch 

# Give all users access to cargo and rust home
RUN chmod -R a+rw $CARGO_HOME \
    && chmod -R a+rw $RUSTUP_HOME

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog

FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

ARG LLVM_VER=11
RUN echo "deb http://apt.llvm.org/buster/ llvm-toolchain-buster-$LLVM_VER  main" >> /etc/apt/sources.list

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update
RUN apt-get install -y clang-$LLVM_VER lldb-$LLVM_VER lld-$LLVM_VER clangd-$LLVM_VER
RUN apt-get install -y vim

USER vscode
RUN rustup update stable && rustup component add rustfmt && rustup component add clippy && rustup default stable
RUN cargo install cargo-watch 

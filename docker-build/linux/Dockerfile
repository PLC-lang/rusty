FROM rust:buster

ARG LLVM_VER=12
ARG RUST_VER=1.54.0

# Setup llvm sources
RUN echo "deb http://apt.llvm.org/buster/ llvm-toolchain-buster-$LLVM_VER  main" >> /etc/apt/sources.list.d/llvm.list
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

RUN apt-get update
#Install Clang dependencies
RUN apt-get install -y zip clang-$LLVM_VER lldb-$LLVM_VER lld-$LLVM_VER clangd-$LLVM_VER liblld-$LLVM_VER-dev llvm-$LLVM_VER-dev

RUN rustup install $RUST_VER && rustup install nightly && rustup default $RUST_VER 
RUN rustup component add rustfmt clippy llvm-tools-preview 

RUN cargo install mdbook grcov

WORKDIR /build
ENTRYPOINT ["bash"]
SHELL ["bash"]

#Activate this for local builds to enable watching
#RUN cargo install cargo-watch 
#ENTRYPOINT ["cargo"]

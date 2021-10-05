FROM rust:buster

ARG LLVM_VER=11
ARG RUST_VER=1.54.0

# Setup llvm sources
RUN echo "deb http://apt.llvm.org/buster/ llvm-toolchain-buster-$LLVM_VER  main" >> /etc/apt/sources.list.d/llvm.list
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

RUN apt-get update
#Install Clang dependencies
RUN apt-get install -y clang-$LLVM_VER lldb-$LLVM_VER lld-$LLVM_VER clangd-$LLVM_VER liblld-$LLVM_VER-dev llvm-$LLVM_VER-dev

RUN rustup install $RUST_VER 
RUN rustup default $RUST_VER 
RUN rustup component add rustfmt 
RUN rustup component add clippy 

RUN cargo install mdbook

#RUN cargo install cargo-watch #Activate this for local builds to enable watching

#ENTRYPOINT ["cargo"]

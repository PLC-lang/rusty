FROM rust:buster

ARG LLVM_VER=12
RUN echo "deb http://apt.llvm.org/buster/ llvm-toolchain-buster-$LLVM_VER  main" >> /etc/apt/sources.list

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update
RUN apt-get install -y clang-$LLVM_VER lldb-$LLVM_VER lld-$LLVM_VER clangd-$LLVM_VER
RUN rustup component add rustfmt
RUN rustup component add clippy

#ENV LLVM_SYS_100_PREFIX /usr/lib/llvm-10  #Enable on local systems, the github action does this automatically

#RUN cargo install cargo-watch #Activate this for local builds to enable watching

ENTRYPOINT ["cargo"]

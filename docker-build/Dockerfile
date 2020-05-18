FROM rust:latest


    
RUN echo "deb http://apt.llvm.org/buster/   llvm-toolchain-buster-10  main" >> /etc/apt/sources.list

RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update
RUN apt-get install -y clang-10 lldb-10 lld-10 clangd-10
ENV LLVM_SYS_80_PREFIX /usr/lib/llvm-10 
#RUN apt-get install lsb-release software-properties-common -y
#RUN bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"

VOLUME ["/project"]

#RUN useradd -ms /bin/bash builder -u 1000
#RUN chown builder /project
#USER builder

#RUN cargo install cargo-watch
#WORKDIR /

ENTRYPOINT = ["cargo"]

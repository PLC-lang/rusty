FROM ghcr.io/plc-lang/rust-llvm:latest

# Give all users access to cargo and rust home
RUN chmod -R a+rw $CARGO_HOME \
    && chmod -R a+rw $RUSTUP_HOME

# Required if we want to use `lld` as the default linker for RuSTy
ENV LLVM_VER=14
RUN ln -sf /usr/bin/ld.lld-$LLVM_VER /usr/bin/ld.lld

# Install the local RuSTy version
WORKDIR /rusty
COPY . .
RUN ./scripts/build.sh --build

# Allow invoking `plc` from anywhere
ENV PATH="/rusty/target/debug:${PATH}"

ENTRYPOINT [ "/bin/bash", "-c" ]
CMD ["plc", "--help"]

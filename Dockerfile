ARG RUST_LLVM_VERSION=latest
FROM ghcr.io/plc-lang/rust-llvm:${RUST_LLVM_VERSION}

# Allow invoking `plc` from anywhere
ENV PLCLOC="/opt/rusty"
ENV STDLIBLOC="/opt/rusty/stdlib"
ENV PATH="${PLCLOC}:${PATH}"

# Install the local RuSTy version
COPY artifacts/plc /opt/rusty
# Make the binary executable
RUN chmod +x /opt/rusty/plc
# Copy the standard library
COPY artifacts/stdlib /opt/rusty/stdlib

ENTRYPOINT [ "/bin/bash", "-c" ]
CMD ["plc", "--help"]

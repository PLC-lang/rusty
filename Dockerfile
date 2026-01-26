ARG LLVM_VER=21
ARG RUST_VER=1.90
ARG CONTAINER_VERSION=$LLVM_VER-$RUST_VER
ARG BASE_IMAGE=ghcr.io/plc-lang/rust-llvm:$CONTAINER_VERSION
FROM $BASE_IMAGE

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

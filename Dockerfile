ARG LLVM_VER=21
ARG RUST_VER=1.90
ARG CONTAINER_VERSION=$LLVM_VER-$RUST_VER
ARG BASE_IMAGE=ghcr.io/plc-lang/rust-llvm:$CONTAINER_VERSION
FROM $BASE_IMAGE

# Install plc compiler and standard library from .deb packages
# Include both amd64 and arm64 stdlib for cross-compilation support
COPY artifacts/deb/*.deb /tmp/deb/

RUN native_arch=$(dpkg --print-architecture) && \
    # Determine the foreign architecture and add it
    if [ "$native_arch" = "amd64" ]; then foreign_arch="arm64"; \
    else foreign_arch="amd64"; fi && \
    dpkg --add-architecture "$foreign_arch" && \
    # Install the foreign libc6 and all .deb packages with dependency resolution
    apt-get update && \
    apt-get install -y --no-install-recommends "libc6:${foreign_arch}" /tmp/deb/*.deb && \
    rm -rf /tmp/deb && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    ldconfig

# Standard library include files for the compiler
ENV STDLIBLOC="/usr/share/plc/include"

ENTRYPOINT ["plc"]
CMD ["--help"]

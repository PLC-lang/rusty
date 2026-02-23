ARG LLVM_VER=21
ARG RUST_VER=1.90
ARG CONTAINER_VERSION=$LLVM_VER-$RUST_VER
ARG BASE_IMAGE=ghcr.io/plc-lang/rust-llvm:$CONTAINER_VERSION
FROM $BASE_IMAGE

# Install plc compiler and standard library from .deb packages
# Include both amd64 and arm64 stdlib for cross-compilation support
COPY artifacts/deb/*.deb /tmp/deb/

RUN native_arch=$(dpkg --print-architecture) && \
    for deb in /tmp/deb/*.deb; do \
        deb_arch=$(dpkg-deb --field "$deb" Architecture) ; \
        if [ "$deb_arch" != "$native_arch" ] && [ "$deb_arch" != "all" ]; then \
            dpkg --add-architecture "$deb_arch" 2>/dev/null || true ; \
        fi ; \
    done && \
    dpkg --force-architecture -i /tmp/deb/*.deb && \
    rm -rf /tmp/deb && \
    ldconfig

# Standard library include files for the compiler
ENV STDLIBLOC="/usr/share/plc/include"

ENTRYPOINT [ "/bin/bash", "-c" ]
CMD ["plc", "--help"]

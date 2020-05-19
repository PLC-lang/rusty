#!/bin/bash
podman run -it -v ./:/usr/local/src -w /usr/local/src rust-llvm sh -c ./cargo_watch.sh

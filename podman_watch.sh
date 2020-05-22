#!/bin/bash

if [ ! -z "$1" ]  && [ $1 == '-d' ] 
then
	podman run -it -v ./:/usr/local/src -w /usr/local/src rust-llvm bash -c 'export RUST_BACKTRACE=1 && ./cargo_watch.sh'
else
	podman run -it -v ./:/usr/local/src -w /usr/local/src rust-llvm bash -c ./cargo_watch.sh
fi

# Building and Installing

## Prerequisites
To be able to build the source code, you will need to [install Rust](https://www.rust-lang.org/tools/install)
on your machine. If you want to clone and work on the repository, you'll also need _git_.

## Cloning the repository
On your local computer, open up a shell and clone the repository.
```
$ git clone https://github.com/ghaith/rusty
$ cd rusty
```

## Building
Building is as easy as typing a single command. If you just want to use the
compiler without doing development, building the release configuration will
give you a smaller and likely a faster binary.
```
$ cargo build --release
```

You can find the binary at `./target/release/rustyc`.

## Installing
_TODO_
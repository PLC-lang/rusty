# AGENTS

[Guidelines for coding agents](https://agents.md/) working in this repository. Do not modify this file unless
a human explicitly asks.

## Build and test

```bash
make check  # Compile
make lint   # Format and lint
make test   # Unit and integration tests
```

Prefer to use `make check` for quick feedback and `make lint` followed by `make test` before finalizing your
changes or handing off.

## Architecture

WIP

## Debugging

Sometimes you might want to understand the execution flow, in which case temporary `panic!()` calls at the
point of interest run with `RUST_BACKTRACE=1 cargo run` or `RUST_BACKTRACE=full cargo run` (more detailed) can
be helpful. For most cases this might be overkill and your usual grep approach works fine, however.

## Documentation

Document your code. Module and item documentation is mandatory. Inline comments are optional except for two
cases (1) when the intent of a code snippet is not immediately obvious and (2) when multiple statements form a
single logical step.


## Worktree Changes

NEVER run any `git` commands without explicit user approval.

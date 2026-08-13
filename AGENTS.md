# Agent Guidelines

This file defines instructions for project agents. Only a human may modify it unless instructed otherwise. Agents MUST follow these instructions.


## Conversation Style

- Keep messages concise without omitting important details. Use ASD-STE100 Simplified Technical English.
- End every message with a 1-2 sentence TL;DR that summarizes the key points.


## Build

```bash
# Build the project
cargo build

# Run the test suite
cargo test --workspace && ./scripts/build.sh --lit

# Run the linters
cargo fmt --all && cargo clippy --workspace
```


## Git

- Use conventional commits: `<type>(<scope>): <description>`. Keep titles under 72 characters.
- Use a commit body only when necessary, such as for a large change that needs a brief explanation. Keep it concise and use:
    ```
    Problem: <concise but detailed problem description>

    Solution: <concise but detailed solution description>
    ```
- Use the same format for PR titles. Include a mandatory `Problem` and `Solution` description.
- Before committing or opening a PR, run `cargo test --workspace`, `./scripts/build.sh --lit`, `cargo fmt --all`, and `cargo clippy --workspace`. Fix all failures first.


## Code Style

- Keep changes small and implement only what was requested. Make code self-explanatory.
- Avoid comments except for genuinely complex code. Use brief skimming comments to separate every logical group, so functions read as outlines.
- Define Rust structs, enums, and related items first, in logical order. Follow each with its `impl` block in the same order.
- Use inline snapshots: `insta::assert_snapshot!(result, @r"");`, not `insta::assert_snapshot!(result);`.
- Avoid fully qualified paths such as `plc_source::source_location::SourceLocation` unless name clashes require them.
- Do not reference external plans, phases, tickets, or roadmap items in source code, doc comments, or tests. Describe the current behavior directly instead.

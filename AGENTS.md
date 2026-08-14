# Agent Guidelines

This file defines instructions for project agents. Only a human may modify it unless instructed otherwise. Agents MUST follow these instructions.


## Build

```bash
# Build the project
cargo build

# Run the test suite
cargo test --workspace && ./scripts/build.sh --lit

# Run the linters
cargo fmt --all && cargo clippy --workspace
```


## Conversation Style

- Keep messages concise without omitting important details. Use ASD-STE100 Simplified Technical English.
- End every message with a 1-2 sentence TL;DR that summarizes the key points.
- Do not use em dashes. Use commas, semicolons, or parentheses instead. This applies to all writing: messages, commit messages, PR descriptions, and comments.


## Code Quality

- Read files in full before making any changes, do not rely on partial information.
- Keep changes small and implement only what was requested. Make code self-explanatory.
- Avoid comments except for genuinely complex code. Use brief skimming comments to separate every logical group, so functions read as outlines.
- Define Rust structs, enums, and related items first, in logical order. Follow each with its `impl` block in the same order.
- Use inline snapshots: `insta::assert_snapshot!(result, @r"");`, not `insta::assert_snapshot!(result);`.
- Avoid fully qualified paths such as `plc_source::source_location::SourceLocation` unless name clashes require them.
- Do not reference external plans, phases, tickets, or roadmap items in source code, doc comments, or tests. Describe the current behavior directly instead.


## Git

- Use conventional commits: `<type>(<scope>): <description>`. Keep titles under 72 characters.
- PR descriptions are for humans. They orient the reviewer: after reading, the reviewer must know the problem and the solution before they open the diff. Keep each part to 1-3 sentences and use:
    ```
    Problem: <what breaks or is missing, with the observable effect>

    Solution: <the approach, not the file-by-file details>

    Refs: PRG-<JIRA Ticket ID>   (omit if unknown)
    ```
- Do not enumerate affected functions or files in the description; the diff shows them.
- Use the same `Problem:`/`Solution:` format for commit bodies, and only when the change needs explanation beyond its title.
- Before committing or opening a PR, run `cargo test --workspace`, `./scripts/build.sh --lit`, `cargo fmt --all`, and `cargo clippy --workspace`. Fix all failures first.


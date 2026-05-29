# AGENTS

Instruction for agents in this project, only to be modified by a human unless instructed otherwise. See https://agents.md.

## Build
```bash
cargo run -- <files>                                # Compile files
cargo test --workspace && ./scripts/build.sh --lit  # Run all unit and integration tests
cargo fmt --all && cargo clippy --workspace         # Run the formatter and linter
```

Guidelines (not mandatory but helpful for bigger changes)
- Run the tests whenever you make final changes to verify for correctness
- The formatter and linter when all tests work and you're about to report back to the user
- `cargo run -- --help` when debugging to discover flags like `--ast`, `--ast-lowered`, `--ir`, etc..


## Code style
1. Do not eagerly add `#[derive(...)]` implementations like a `Debug` or `Clone`. Only add them when needed.
2. Avoid section and header comments like for example `// -- Types ---------`; use `// Types` instead if it truly makes sense.
3. Define all Rust structs, enums, etc.. first (in logical order) followed by their `impl` blocks in the same order.
```rust
// Bad
enum Foo { ... }
impl Foo { ... }

struct Bar { ... }
impl Bar { ... }

// Good
enum Foo { ... }
struct Bar { ... }

impl Foo { ... }
impl Bar { ... }
```

4. Group statements by logical intent, separated by blank lines. Each group should have a brief comment describing *what* it does so the function reads like an outline — readers can skim the comments top-to-bottom and only dive into the code when they need the *how*. Omit the comment only when the intent is immediately obvious from the code itself. For example
```rust
pub fn source_text(&self, span: Span) -> &str {
    let file = self.lookup_source_file(span.lo());

    // Convert absolute byte positions to file-relative offsets.
    let lo = (span.lo().0 - file.start_pos.0) as usize;
    let hi = (span.hi().0 - file.start_pos.0) as usize;

    &file.src[lo..hi]
}

pub fn lookup_line_col(&self, pos: BytePos) -> (&str, u32, u32) {
    let file = self.lookup_source_file(pos);

    // Convert absolute position to file-relative offset.
    let relative = RelativeBytePos(pos.0 - file.start_pos.0);

    // Binary search for the line containing this position.
    let line_idx = file.line_starts.partition_point(|&line| line.0 <= relative.0).saturating_sub(1);

    // Offset from the start of that line.
    let column_idx = relative.0 - file.line_starts[line_idx].0;

    (&file.name, line_idx as u32 + 1, column_idx)
}
```

5. When snapshot testing, prefer inline snapshots. Also use `cargo insta --help` to interact with snapshot management.
```rust
// Bad
insta::assert_snapshot!(result);

// Good
insta::assert_snapshot!(result, @r"");
```

6. Avoid fully qualified paths like `plc_source::source_location::SourceLocation` unless required due to name clashes
```
// Bad
fn get_base(node: &AstNode) -> plc_source::source_location::SourceLocation { ... }

// Good
fn get_base(node: &AstNode) -> SourceLocation { ... }
```

7. No phase references in code. Never write "Phase 1", "for now", "later phase", or similar roadmap labels in source, doc comments, or tests — those belong in PLAN.md, PRs, or issues. If a type or function is intentionally incomplete, describe *what* is incomplete (e.g. "inner content is captured opaquely") rather than tying it to a numbered phase.

8. No assumed reference files in finalized code. Never `include_str!`/`include_bytes!` paths that traverse out of the crate, write doc comments that point at files outside the crate (e.g. "mirrors path/to/schema.xsd"), or read test fixtures from sibling directories — fixtures live in the crate's own `tests/fixtures/`. External specs, vendor schemas, and sample inputs are reference-only and must not be load-bearing for build, test, or docs.

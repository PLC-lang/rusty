# AGENTS

Instruction for agents in this project, only to be modified by a human unless instructed otherwise. See https://agents.md.

## Build
```bash
# Build
cargo build

# Test
cargo test --workspace      # Unit and integration tests
./scripts/build.sh --lit    # Integration (LLVM lit) tests
```

Make sure to run any commands involving the compiler in the VS Code devcontainer if it is active. If not run
natively on host. To check, see if a container is active and has a docker image name starting with `vsc-rusty`.

## Code style
1. Do not eagerly add `#[derive(...)]` implementations like a `Debug` or `Clone`. Only add them when needed.
2. Try to keep line lengths at max 110 characters.
3. Avoid section and header comments like for example `// -- Types ---------`; use `// Types` instead if it truly makes sense.
4. Define all Rust structs, enums, etc.. first (in logical order) followed by their `impl` blocks in the same order.
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

5. Group statements by logical intent, separated by blank lines. Each group should have a brief comment describing *what* it does so the function reads like an outline — readers can skim the comments top-to-bottom and only dive into the code when they need the *how*. Omit the comment only when the intent is immediately obvious from the code itself. For example
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

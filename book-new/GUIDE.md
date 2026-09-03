# Writing Guide (temporary)

This file records how the technical documentation in this book is written. It exists so that a new agent or contributor can continue the work in the same way. Remove it when the book is complete.


## Goal

The technical documentation explains the architecture and internals of the compiler to both humans and agents. After reading it, a reader must understand how the whole compiler works and how each stage fits in. The reader does not need to know internal names such as the exact name of a struct or function; that information lives in the source code.

The book lives in `book-new/` as plain markdown files. It has two parts, `user/` and `technical/`. Only `technical/` is in scope for now. Integration into a book generator such as mdBook happens later, once the content is complete.


## Source of truth

The `plc_driver` crate is the entry point for the whole pipeline. Its run loop calls every stage in order and registers every participant. Start there for any chapter, then dive into the crate that implements the stage.

`cargo r -- --help` lists all compiler flags. The flags `--ast`, `--ast-lowered`, `--ir`, and `--check` are useful for tracing without adding any logging.


## Structure

```
book-new/
  README.md
  GUIDE.md                # this file, temporary
  bugs.md                 # bugs found while researching
  technical/
    overview.md           # one-page view of the whole compiler; placed first, written last
    pipeline/             # one chapter per stage, in execution order
    participants/         # one chapter per participant, in registration order
    outputs/              # alternative pipeline outputs (headers, hardware map)
    foundations/          # shared building blocks (AST, diagnostics, project model)
    internals/            # one chapter per language construct: how it is indexed, resolved, and generated (mirrors src/tests/adr/)
```

Chapters are numbered by the order in which the compiler runs them. Participants are numbered by the order in which the driver registers them, because later participants depend on earlier ones.


## Workflow per chapter

1. **Write an example first.** Create a small Structured Text project that exercises the module, including its edge cases. Keep it in the scratchpad, not in the repository.
2. **Trace it.** Add temporary `eprintln!` statements that cover the whole module: its internal data structures (for example the full `pub struct Index { ... }`) and its complete call flow. Compile and run the example. Adapt the method to the module; not every module has a single central struct.
3. **Collect and summarize.** Read the output, work out (a) the internal structure and (b) the call flow, and only then write.
4. **Remove all logging** before committing. Use `git stash` or a throwaway branch for the tracing changes.
5. **Explain with minimal examples.** Each section gets its own short snippet (a few lines) that shows exactly the concept of that section, and the outcome is described in prose. Do not paste trace output or a large shared example into the chapter; the trace is research material, not documentation.
6. **Log bugs.** When the research hits a bug, work around it, do not fix it, and add an entry to `bugs.md` in the format described there (severity, source location, title, description).

Chapters are written one at a time. Each chapter gets a review before the next one starts.


## Writing rules

- Detailed but not overly specific. Describe architecture, data flow, and responsibilities. Do not walk through functions line by line.
- Reference code only when it helps the reader visualize something, for example the fields of the symbol table or the annotation map. Then copy the real struct, trimmed to the relevant fields.
- No unnecessary jargon. Technical but simple to read. Use ASD-STE100 Simplified Technical English.
- No em dashes. Use commas, semicolons, or parentheses.
- Do not reference plans, tickets, or roadmap items. Describe current behavior.
- Every pipeline chapter follows the same shape: an intro that says what the stage is for and why it exists (motivated by a tiny snippet), the pipeline mermaid diagram with the current stage highlighted (`style <node> fill:#1e3a8a,stroke:#0f172a,stroke-width:3px,color:#fff`), one section per concept with its own minimal example, and a closing "What's next" section that hands over to the next stage with a concrete open question.
- Prefer prose over dumps. A short annotated call stack or a diff block is fine when it shows a mechanism; raw trace output, big shared examples, and long struct listings are not. When a struct is shown, trim it to its fields, put a blank line between fields, and give each a concise one-line comment.
- Do not enumerate entry or node fields in prose. Point to the source file and to the internals chapter instead.
- Do not explain library choices (for example which lexer generator is used). Describe the mechanism as if it were hand-written.
- Indent Structured Text examples: variable blocks and bodies one level inside the POU, declarations two levels, a blank line between the last `END_VAR` and the body.
- Use `> **Note**` blockquotes for short asides and `> **Developer Note**` for known technical debt or historical context.
- Do not mention that facts were obtained with temporary logging.


## Status

Done and reviewed by the maintainer:

- `technical/pipeline/00-driver.md`: the pipeline, the participant model with a hook diagram, and a Developer Note on participants as technical debt.
- `technical/pipeline/01-lexer-parser.md`: lexer with a token table, recursive descent parser with an annotated call stack and the resulting AST, error recovery, a Note on why POU declarations and implementations are separate lists.

Written, awaiting review:

- `technical/pipeline/02-index.md` (rewritten in the resolver's style): pre-processing shown as a diff, the index struct, a step-by-step walk of indexing one function block with a caret-marked visualization of the entries, merging, a step-by-step constant evaluation with a visualization of the arena.

- `technical/pipeline/03-resolver.md`: annotations versus type hints, the annotation map struct, walk order and name resolution strategies, assignments and promotion, calls, literals and generated types, dependencies.

Next: `technical/pipeline/04-validation.md`. Then codegen, linker, then the participants, outputs, foundations, and internals chapters. The overview is written last.

Review loop: the maintainer reads each chapter and gives feedback in several rounds before the next chapter starts. Expect requests to trim; when in doubt, write less and offer alternatives labeled A, B, C in the markdown for the maintainer to pick from.

Tracing notes: apply logging with a Python script that does exact string replacements (kept in the scratchpad), gate it behind an environment variable, and revert with `git checkout` on the touched files before writing. The driver runs the index and annotate stages many times because of participants; gate traces to the first run. `plc --ast`, `--ast-lowered`, `--ir`, and `--check` avoid the need for logging in many cases.

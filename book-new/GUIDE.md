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
- Every pipeline chapter follows the same shape: an intro that says what the stage is for and why it exists (motivated by a tiny snippet), the pipeline mermaid diagram with the current stage highlighted (`style <node> fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a`), one section per concept with its own minimal example, and a closing "What's next" section that hands over to the next stage with a concrete open question.
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

- `technical/pipeline/04-validation.md` (kept shallow on purpose): the validation context with its two sources (index and annotations), global validation from the index, per-unit validation with a caret-marked body example, severity registry and reporting, a section on the four participants that validate during lowering with a Developer Note on why.

- `technical/pipeline/05-codegen.md` (rewritten): one running project introduced up front, then one section per mechanism (module build order and the LLVM index, data types, globals, functions and the setup part, the two expression rules, statements, initialization, output), each paired with the trimmed IR of the part of the project that shows it. Closes with a "Where it lives" section.

- `technical/pipeline/06-linker.md` (trimmed): inputs, the output format table, how the linker program is chosen, the assembled command line with one example, a Developer Note on the external linker.

Every pipeline chapter has a "Where it lives" table (what, where) directly above "What's next" that names the crates and top-level modules; the prose above it stays free of crate and file names. The participants README carries the locations in a Where column of its table instead of per-chapter sections; the internals chapters have no locations, because they cut across the whole compiler. The outputs chapters get the same table as the pipeline chapters once they are written.

- `technical/participants/01-loop-desugar.md`: the first participant chapter and the template for the others: an intro with a snippet and the hook diagram (the hooks the participant uses are highlighted), a Transformation section with Structured Text diffs, an Interactions section, and a Validation section only where the participant reports diagnostics. No "What's next"; the participants README carries the order.

- `technical/participants/02-property.md`, `04-control-statements.md`, `05-reference-to-return.md`, `06-init.md`, `07-retain.md`, `08-generic.md`, `09-aggregate-return.md`, `10-inheritance.md`, `11-array.md`: written to the same template, each researched with its own example set; the bugs found along the way are in `bugs.md`.

- `technical/participants/README.md`: the ordered table of all twelve participants with their hooks, one-line purposes, and locations, plus one paragraph on the order. Index pages (the README of every part) stay at surface level: a short intro, a table with one line per chapter, at most one paragraph after it.

- `technical/participants/03-polymorphism.md`: the two-hook participant, reshaped from the draft in `src/lowering/polymorphism/draft.md` into the template with one running example (`Rect`, `Square`, `Shape`); method tables with the slot table, interface tables with upcast members, the fat pointer and its four rewrites, and a Validation section on E126 and E129. About twice the length of the other participant chapters because it holds two lowerings.

- `technical/participants/00-cfc.md`: the participant with a foothold in the parse step; the document model (ids, pins, wires) shown on a trimmed XML excerpt, the resolver's roles table and wire tracing, blocks (stateless temporaries versus member reads, execution control), priority order, rendering with block locations, the inference rounds for generic temporaries, and the list of wiring diagnostics. Architecture over features, on purpose.

- `technical/internals/03-strings.md`: the first internals chapter and the candidate template for the others: one running project followed through Declaration, Index (the trimmed type record and the four string types of the example), Annotations (caret map, per-literal sized types, the hidden compare call), Lowering (one paragraph, links only), Codegen (Layout, Assignment, Passing and returning, Comparison, each with trimmed IR), Validation, and a closing "At a glance" table. The ADR covers codegen only; index and annotations were traced with a throwaway test.

- `technical/internals/00-pous.md`, `01-structs.md`, `02-arrays.md`, `04-enums.md`, `05-variable-length-arrays.md`, `06-reference-expressions.md`, `07-initializers.md`: written to the strings template, each from its own traced running example and the matching ADR. `08-annotated-ast.md` deviates on purpose: one section per annotation kind (trimmed variant, caret example, who reads it) instead of the construct outline. `internals/README.md` has the chapter table with each construct's one-sentence internal truth. The research found a P0 (fixed array passed to a function block's VLA parameter is not wrapped), several P1s (VLA panics, string argument over-read, bare constant repeat count), and smaller items, all in `bugs.md`. One correction landed in `pipeline/02-index.md`: the `__X__init` default-instance entries are registered but never read.

- `technical/outputs/00-header-generator.md`: invocation table, the template model struct, the type translation, one running project rendered part by part (types, stateful POUs, functions, globals), combining, a Developer Note on the ordering bugs. `01-hardware-map.md`: why the map exists (the pre-processor's generated globals), one project with every binding kind and its map, the two collection passes, the name mangling rule, files and formats, a Developer Note on the deprecated `--hardware-conf`. Both branch off the pipeline diagram after Validate and have no "What's next". Research found three P2 and two P3 header generator bugs, in `bugs.md`.

- `technical/overview.md`: one page, written last: the pipeline diagram with the participant loop and the output branch, one bold paragraph per part with links to every pipeline chapter, and the four tracing flags.

Every part has a README index page (pipeline, participants, outputs, internals). The content is complete; what remains is the review of the outputs and overview chapters and the removal of this guide. The foundations part was dropped; the AST, diagnostics, and project model are covered where the pipeline chapters use them.

Tracing internals: `--ast`, `--ast-lowered`, and `--ir` give the parser, lowering, and codegen views. For the index and annotation views, add a throwaway test module under `src/tests/adr/` that calls `index_with_ids`, `evaluate_constants`, and `annotate_with_ids` from `test_utils` and prints the records with `{:#?}`; remove the file and its `mod` line before committing.

Review loop: the maintainer reads each chapter and gives feedback in several rounds before the next chapter starts. Expect requests to trim; when in doubt, write less and offer alternatives labeled A, B, C in the markdown for the maintainer to pick from.

Tracing notes: apply logging with a Python script that does exact string replacements (kept in the scratchpad), gate it behind an environment variable, and revert with `git checkout` on the touched files before writing. The driver runs the index and annotate stages many times because of participants; gate traces to the first run. `plc --ast`, `--ast-lowered`, `--ir`, and `--check` avoid the need for logging in many cases.

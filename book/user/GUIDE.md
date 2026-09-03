# User Writing Guide (temporary)

This file records how the user documentation in this book is written. It exists so that a new agent or contributor can continue the work in the same way. Remove it when the user part is complete. The companion file `../GUIDE.md` covers the technical part.


## Goal

The user documentation explains how to install the compiler, how to use it, and how to write Structured Text for it. It has two readers:

- The application developer, who writes Structured Text and compiles it.
- The integrator, who builds, links and ships projects, and who connects them to foreign code.

After reading, a reader can install `plc`, compile and run a project, use every language construct the compiler accepts, and call into and out of C. The reader does not need to know how the compiler works inside; that is the technical part.

The language guide is complete for the supported subset, not a list of deviations. IEC 61131-3 is not freely available, so the reader has no other source.


## Two kinds of page

The book follows the split that the documentation of Rust, Go, Kotlin, and TypeScript uses: material to learn from, and material to look things up in. A page is one or the other, never both.

| Kind | Parts | Shape |
|---|---|---|
| Guide | Get Started, Language, Building, Interoperability | Prose with worked examples. The reader reads it once, from the top |
| Reference | Reference | Tables and lists. The reader arrives from a search or a link, reads one row, and leaves |

The rule that follows from this: a complete list belongs in the reference. A guide chapter shows the few forms that a reader needs to start and links to the reference for the rest. The command line chapter of the guide explains six options; the reference lists all of them.


## Source of truth

The compiler is the source of truth, not the old book. Every claim is verified by compiling a snippet.

| What | Where |
|---|---|
| Flags and subcommands | `cargo r -- --help`, `compiler/plc_driver/src/cli.rs` |
| Keywords, attributes, operators | `compiler/plc_lexer/src/lexer/tokens.rs`, `src/parser/expressions_parser.rs` |
| Builtin functions | `src/builtins.rs` |
| Error codes and their text | `compiler/plc_diagnostics/src/diagnostics/error_codes/` |
| Project file keys | `compiler/plc_project/schema/plc-json.schema` |
| Accepted and rejected programs | `tests/lit/`, `src/tests/` |
| Standard library functions | `libs/stdlib/iec61131-st/` |
| The mechanism behind a behavior | `book/technical/`, linked, never repeated |


## Structure

```
book/
  user/
    GUIDE.md        # this file, temporary
    README.md       # what the reader will be able to do, and the reading order
    get-started/    # install, hello world, a first project
    language/       # the language guide, one chapter per area, in reading order
    building/       # the compiler as a tool, task by task
    interop/        # foreign code in both directions
    reference/      # the complete lists
```

File names are words, not numbers. The order comes from `SUMMARY.md`, and the name appears in the URL, so `language/control-flow.md` is better than `language/05-control-flow.md`. The technical part keeps its numbers, because its chapters follow the run order of the compiler.


## Chapter shape

Every guide chapter:

1. Opens with one or two sentences that say what the reader will be able to do.
2. Has one section per idea, and each section has one example.
3. Ends with **What's next**, one sentence that hands over to the next chapter.

Examples are complete programs where that is possible, and they compile as written. Print the result and show the output when the output is the point of the section. A section that only shows syntax needs no output.

Reference pages open with one line that says they are for looking up, and they may be nothing but tables.


## Writing rules

- Prose first. Use a table only for content that is a table: type sizes, escape sequences, operator precedence, flags, keys. A table of three rows that could be a sentence is a sentence.
- Describe behavior, not implementation. The reader must not need the compiler source.
- Write in the second person for tasks, and in the third person for rules.
- State the limits of a construct in the same section, and name the error code (for example `E062`) when the compiler rejects the form.
- Prefer the short example. If a chapter needs a larger program, build it once and reuse it across sections.
- No unnecessary jargon. Technical but simple to read. Use ASD-STE100 Simplified Technical English.
- No em dashes. Use commas, semicolons, or parentheses.
- Do not reference plans, tickets, or roadmap items. Describe current behavior.
- Indent Structured Text examples: variable blocks and bodies one level inside the POU, declarations two levels, a blank line between the last `END_VAR` and the body.
- Use the admonition blocks of mdBook: `> [!NOTE]` for a short aside, `> [!IMPORTANT]` for a rule that breaks code when it is missed, and `> [!WARNING]` for something that is deprecated or wrong today. A custom title is not supported, so put the label in the text (`> **Deprecated.** ...`). At most one per section, and never two in a row.
- An index page (the README of every part) is one paragraph of at most five sentences that says what the part and its chapters teach. It carries no table and no list of chapter links, because the sidebar and the navigation arrows already do that.


## Coverage

### get-started

| Chapter | Covers |
|---|---|
| `install.md` | Release packages, binaries, build from source, verification |
| `hello-world.md` | One file, compile, link, run, and the first diagnostic |
| `first-project.md` | A program, a function block, two files, `plc.json`, `plc build` |

### language

Read in this order. Each chapter uses only what the chapters before it introduced.

| Chapter | Covers |
|---|---|
| `source-files.md` | Files and units, comments, identifiers and case, scope, attributes |
| `variables.md` | The variable blocks, constants, retained data, initial values |
| `basic-types.md` | Integers, bit strings, `BOOL`, reals, literals, conversion between them |
| `text.md` | `STRING` and `WSTRING`, length, escapes, comparison, standard functions |
| `time.md` | The eight time and date types, their literals, and their units |
| `composite-types.md` | Arrays, structs, enumerations, subranges, aliases |
| `expressions.md` | Every operator, precedence, associativity, and the conversion rules |
| `control-flow.md` | `IF`, `CASE`, the three loops, `EXIT`, `CONTINUE`, `RETURN` |
| `functions.md` | `FUNCTION`, parameters, the call rules, the return value |
| `function-blocks.md` | `FUNCTION_BLOCK` and `PROGRAM`, instances, state, actions, `FB_INIT` |
| `methods-and-properties.md` | `METHOD`, `PROPERTY_GET`, `PROPERTY_SET` |
| `inheritance.md` | `EXTENDS`, `INTERFACE`, `CLASS`, dispatch, `THIS`, `SUPER`, access modifiers |
| `generics.md` | Type parameters, constraints, and how a call resolves |
| `pointers.md` | `REF_TO`, `POINTER TO`, `REFERENCE TO`, `REF`, `ADR`, `NULL` |
| `hardware-access.md` | Direct access on a value, `AT` addresses, `VAR_CONFIG` |
| `cfc.md` | Graphical code: what the compiler reads, execution order, the diagnostics |

### building

| Chapter | Covers |
|---|---|
| `compiling.md` | Files in, artifact out; optimization, target, check, the project file |
| `linking.md` | Linker choice, libraries, the standard library, relocation model |
| `diagnostics.md` | Reading a message, `plc explain`, changing a severity |
| `debugging.md` | `-g`, DWARF versions, path remapping for shipped builds |

### interop

| Chapter | Covers |
|---|---|
| `calling-c.md` | `{external}`, includes, variadic functions |
| `c-interface.md` | The C type of every construct, parameter passing, constructor symbols |
| `headers.md` | Generating C headers for a project |
| `api-guidelines.md` | How to design a library interface |

### reference

| Chapter | Covers |
|---|---|
| `command-line.md` | Every option and subcommand, grouped by purpose |
| `project-file.md` | Every key of `plc.json` |
| `supported-features.md` | Every construct, and what is not supported |
| `standard-library.md` | The function families of `iec61131std` |
| `error-codes.md` | The generated page per diagnostic code |


## Workflow per chapter

1. **Collect the claims.** List what the chapter must state, from the sources of truth above. For a language chapter, this includes the forms the compiler rejects.
2. **Write the examples.** One small program per claim, in the scratchpad, not in the repository.
3. **Compile every example.** Use `plc --check` for accepted forms, run the program when the output is part of the chapter, and record the real diagnostic for rejected forms.
4. **Write the chapter.** One section per idea, prose first.
5. **Link, do not repeat.** Point to the reference for complete lists, and to `book/technical/` for the mechanism.
6. **Log bugs.** When the research hits a bug, work around it, do not fix it, and add an entry to `../bugs.md`.


## Verification

Examples are verified by hand, per chapter, at the time of writing. There is no automatic gate. Before a pull request, the `doc-sync` skill checks the diff against the book.


## Facts that the research established

The chapters must stay consistent with these. Each one was verified by compiling and running the case.

- Properties use the 2025 syntax, `PROPERTY_GET` and `PROPERTY_SET` with `END_PROPERTY`. There is no `PROPERTY ... GET ... SET` block.
- Names are case-insensitive, so `Main` and `main` collide with `E004`.
- A member of a function block or of a class is private without an access modifier. An access from outside gives `E049`, which is a warning.
- `THIS` works in a `FUNCTION_BLOCK` only, not in a `CLASS`, which gives `E120`.
- A call of a `FUNCTION` must supply every parameter, the outputs included, in the positional form and in the named form (`E032`). A call of a function block instance can supply a part of them.
- `**` calls `EXPT` from the standard library, computes in `REAL`, is left associative, and binds weaker than unary minus (see `bugs.md`).
- String comparison with `=` and `<` calls the standard library.
- Subrange bounds are checked only when the project provides `CheckRangeSigned` or `CheckRangeUnsigned`.
- `VAR_EXTERNAL` is parsed and has no effect (`E106`).
- The default output name is the first input file plus the extension of the format, so `input.st` gives `input.st.o`.
- One `--target` and one `--sysroot` per invocation.
- `$PROJECT_ROOT` and `$ARCH` do not resolve inside `plc.json`, see `bugs.md`.


## Status

Complete. Every chapter of the coverage tables above exists.

The part was written twice. The first version carried the old book over, chapter by chapter; the second rebuilt it into the shape above after a look at how Rust, Go, Kotlin, and TypeScript separate a guide from a reference. What changed in the second pass, and what a later chapter should keep doing:

- The exhaustive option and key tables moved out of the narrative chapters into `reference/command-line.md` and `reference/project-file.md`. A guide chapter now names the handful of options that a build needs and links to the reference.
- The language part grew from 5 chapters to 16, in reading order, each with one idea per section.
- Tables that were prose in disguise became prose. Tables that are real tables (type sizes, escapes, precedence, options, diagnostics of CFC) stayed.
- Seven tool chapters became four, and `interop/02-libraries.md` merged into `building/linking.md`.
- File names lost their number prefixes.

Verification: every complete example in the user part was compiled. The blocks that do not pass `--check` on their own are, by design, the ones that continue a declaration from an earlier block, the three that demonstrate a diagnostic, one that needs the standard library, and the CFC declaration excerpt.

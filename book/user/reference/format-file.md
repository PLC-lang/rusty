# Format File

Every key of `.plcformat.yaml`, for looking up. [Command Line](command-line.md) lists the `format` subcommand's own flags.

`plc format` reformats Structured Text source files: indentation, spacing, keyword casing, and blank-line limits. It works from the token stream alone, not from a parse tree, so it never changes what a file means, only how it looks; comments and pragmas are always kept, verbatim, attached to the line they were on.

```bash
plc format main.st                 # rewrites the file in place
plc format --check main.st          # reports whether it would change, writes nothing
plc format --stdout main.st          # writes the result to standard output
plc format --dump-config             # prints the default configuration
```

The style is controlled by a YAML file named `.plcformat.yaml`. The compiler looks for one next to the input file and then in every parent directory, the same way `.clang-format` is found; the first one it finds wins. `--config <file>` uses that file instead, skipping the search. With no file found and no `--config`, the compiler uses the defaults listed below.


## Keys

| Key | Default | Purpose |
|---|---|---|
| `indent_width` | `4` | Spaces per indentation level |
| `use_tab` | `false` | Indent with tabs instead of spaces |
| `keyword_case` | `Upper` | Casing for ST keywords, see [keyword_case](#keyword_case) |
| `max_consecutive_empty_lines` | `1` | The most blank lines kept in a row; extra ones are dropped |
| `blank_line_after_block_open` | `false` | Force a blank line right after `VAR`, `IF`, `FOR`, and similar openers |
| `blank_line_before_block_close` | `false` | Force a blank line right before `END_VAR`, `END_IF`, and similar closers |
| `space_around_assignment` | `true` | A space on both sides of `:=` |
| `space_around_binary_operators` | `true` | A space on both sides of `+`, `-`, `=`, `AND`, and the other binary operators |
| `space_after_comma` | `true` | A space after `,` |
| `space_before_colon` | `false` | A space before the `:` of a declaration, for example `x : INT` instead of `x: INT` |
| `space_after_colon` | `true` | A space after the `:` of a declaration |
| `trailing_comment_min_spacing` | `2` | Spaces before a comment that trails code on the same line |
| `align_consecutive_declarations` | `false` | Line up the `:` and `:=` of consecutive `VAR`-block declarations, see [Alignment](#alignment) |
| `align_consecutive_assignments` | `false` | Line up the `:=` of consecutive assignment statements in program/function bodies |
| `align_trailing_comments` | `false` | Line up trailing comments across consecutive lines |

Any other key is an error, and the message names the keys the compiler accepts. The file always ends with exactly one newline; there is no key to change that.

```yaml
indent_width: 2
keyword_case: Lower
max_consecutive_empty_lines: 2
```


## keyword_case

| Value | Effect |
|---|---|
| `Upper` | `IF`, `END_IF`, `VAR_INPUT`, ... |
| `Lower` | `if`, `end_if`, `var_input`, ... |
| `Preserve` | Keep whatever casing the file already uses |

Only ST keywords are affected: `IF`, `VAR`, `AND`, `TRUE`, and so on. Identifiers are never touched, which includes built-in type names such as `INT` and `BOOL` - the lexer treats those as ordinary identifiers, not as keywords, so `keyword_case` has no opinion on their casing.


## Alignment

```
VAR
    x                  : INT;
    longName           : BOOL := TRUE;
    EXTREMELY_LONG_NAME: DINT := 100;  // with align_trailing_comments too
END_VAR
```

`align_consecutive_declarations` and `align_consecutive_assignments` are separate options because they cover separate things: the first only ever looks inside `VAR`/`VAR_INPUT`/`VAR_OUTPUT`/... blocks (declarations), the second only ever looks at assignment statements in program and function bodies (logic code). A `VAR` block's declarations are never mixed into a body's assignment alignment, or the other way around.

All three alignment options work on *runs* of consecutive lines, and a blank line always starts a new run - lines above and below a blank line are never aligned to each other. `align_consecutive_declarations` and `align_consecutive_assignments` additionally end a run at any line that is not itself a declaration, respectively not itself an assignment statement; a lone qualifying line with nothing to align against is left alone. `align_trailing_comments` is looser: it aligns across any adjacent lines that each have a trailing comment, including a mix of declarations and code, as long as nothing blank separates them.

A declaration's `:` always aligns across every declaration in its run, since every declaration has one; its `:=` only aligns across the declarations in that run that actually have a default value.


## What the formatter does not do yet

The formatter works directly on tokens, without building a syntax tree, so it does not reflow an expression that already spans multiple lines, and it does not wrap a line that runs past a column limit. It still reindents and respects `max_consecutive_empty_lines` everywhere, including inside a statement; it just keeps the line breaks the file already had within one statement.

A block that is not balanced - a `VAR` without its `END_VAR`, for example - is left untouched: the formatter reports the problem and writes nothing, rather than guess at an indentation for code it cannot make sense of. [`plc check`](command-line.md) is the tool for diagnosing why a file does not parse.

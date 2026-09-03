# Known Bugs

Bugs found while researching the compiler for this documentation. These bugs were worked around during the research, not fixed.

Each entry uses this format:

```
<severity, P0 to P4> <file>:<line>:<column>: <title>
<description: what was observed, what was expected, and how to reproduce it>
```

Severity: P0 wrong code or data loss, P1 crash or compile abort on valid input, P2 wrong diagnostic or missing diagnostic, P3 confusing behavior or inconsistent CLI, P4 cosmetic.

---

P2 compiler/plc_diagnostics/src/reporter/codespan.rs:149:13: Rich reporter drops the diagnostic for a POU named like a builtin
Declaring `FUNCTION add : DINT ... END_FUNCTION` (also seen with `ADR`, `SIZEOF`, `MUX`, `SEL`, `LOWER_BOUND`, `REF`) aborts with only "Compilation aborted due to critical errors" and no diagnostic. With `--error-format clang` the expected `error[E004]: add: Duplicate symbol.` at `add.st:1:10` is printed, so the diagnostic exists. The duplicate check in `src/validation/global.rs:42` only treats undefined or internal locations as builtin; a builtin function has a real text range in the `<builtin>` source, so a normal duplicate diagnostic is created with a secondary "see also" location in `<builtin>`. That source is never registered with the diagnostician, its file handle resolves to `usize::MAX`, the codespan emit call returns an error, and the error is swallowed because the main location is not internal. Expected: a diagnostic such as "add can not be used as a name because it is a built-in function" pointing at the user's declaration, in every error format.

P3 compiler/plc_lexer/src/lexer.rs:229:17: Two diagnostics for one missing closing token
`x := (2 * 3;` produces `E006 Missing expected Token [KeywordParensClose]` and `E007 Unexpected token: expected KeywordParensClose but found ';'` at the same position. The recovery reports the missing token when it finds that the current token closes an outer region (line 229), then closing the region reports the same token as unexpected (line 176). Expected: one diagnostic.

P4 compiler/plc_diagnostics/src/diagnostics.rs:171:13: Plain-text diagnostic summary uses zero-based line and column
The "Details:" list printed after "Compilation aborted due to critical parse errors" shows `broken.st:4:13` for the error the rich renderer shows at `broken.st:5:14`. The text form prints the raw internal location; the renderer adds one to line and column. Expected: both forms agree, one-based.

P3 src/index.rs:343:29: Hardware address segments are registered with the type name "32"
When a variable is declared with a hardware address such as `sensor AT %IX0.0 : BOOL`, each address segment is stored as a constant expression whose target type is the integer constant `DINT_SIZE` converted to a string, so the arena entry reads `target_type=32`. The intent is the type name `DINT`. Because "32" is not a type, the constant evaluator skips the overflow check for these segments. Reproduce with `TRACE`-style logging of `add_constant_expression`, or by inspecting the const expressions arena after indexing a file with an `AT %IX0.0` declaration. Expected: `DINT_TYPE`.

P4 src/resolver.rs:303:9: Call arguments are annotated twice
`visit_call_statement` visits the argument list at line 2703 and then calls `annotate_arguments`, whose first statement (line 303) visits the same argument list again. Every argument expression is therefore resolved and annotated twice per call, which a trace of `i := scale(bufferInstance.values[2], factor := small);` confirms: each reference in the argument list appears twice with identical results. The second pass is idempotent (the map insert overwrites the same value, string literals go into a set), so the output is correct, but the work is doubled for every call in the project and the annotate stage already runs once per participant. Expected: one visit.

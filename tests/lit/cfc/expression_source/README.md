# expression_source

Integration test: a **data source whose identifier is a whole ST expression** (`localA + 5`),
not a bare variable or literal. The transpiler resolves it by running the identifier through
the full ST expression parser, so it lowers to the expression itself.

```text
   localA + 5  ----------->  result   (0)

   (0)  evaluation-priority badge shown by the IDE
```

- `expr.cfc` — `PROGRAM Expr` (`localA`, `result`): an expression data source wired to a sink.
- `main.st` — entry point: sets `localA := 37`, runs `Expr`, prints `result`.

Lowers to `result := localA + 5;`, so `result = 42`. This is the runtime counterpart of the
`expression_source` unit fixture: if the source were mistaken for a one-token reference, the
name `localA + 5` would not resolve to any variable and compilation would fail — so a passing
`result = 42` proves the expression was actually parsed and evaluated.

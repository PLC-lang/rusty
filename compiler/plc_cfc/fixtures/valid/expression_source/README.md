# expression_source

A data source whose identifier is not a bare variable or literal but an arbitrary ST
**expression** (`localA + 5`). The resolver stores it verbatim; the transpiler resolves
it by parsing the identifier through the full ST expression parser, so it lowers to the
expression itself rather than being mistaken for a one-token reference.

```text
   localA + 5  ----------->  result   (0)

   (0)  evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test; an expression data source wired to a sink.

The source feeds the `result` sink directly, so the network means:

```text
result := localA + 5;
```

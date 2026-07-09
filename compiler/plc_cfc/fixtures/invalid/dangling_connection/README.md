# dangling_connection

A sink whose incoming wire references a value that no object produces. Every CFC wire
connects a consumer back to a producer by connection id; here the sink `result`
references `refConnectionPointOutId="999"`, but the only producer in the network is
`localA` at id `2`. Nothing produces `999`, so the wire is dangling.

```text
   localA  --(2)                 (no object produces 999)
                    result  --(999?)-->
   (0)  evaluation-priority badge shown by the IDE
```

Without a check this would slip past the other validations — the evaluation-order
check ignores a connection that resolves to no block, and the variable check only
looks at the identifier text — and then panic the transpiler, whose `resolve` treats
an unknown connection id as unreachable. It is reported here as `E081` so compilation
aborts with a diagnostic instead.

- `mainProgram.cfc` — the program under test; its sink wire points at a missing producer.

See the `dangling_connection_is_reported` test in `validator.rs` and
`dangling_connection_is_not_resolvable` in `resolver.rs`.

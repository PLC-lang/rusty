# call_in_variable (invalid)

Networks where a **variable's free-text expression contains a call**. A variable (data source or
sink) may hold any plain value expression — a name (`foo`), a literal (`100`), or arithmetic over
them (`foo + 1`) — but a call must be drawn as a block, so the network's evaluation priorities state
explicitly when it runs. A call hiding inside a variable's text escapes that ordering and would be
re-evaluated at every consumer. Each `.cfc` file here must be rejected with **E143**.

The called names (`conjure`, `drain`) are never resolved — the validation is purely syntactic — so
no `.st` counterparts are needed.

## `source.cfc` — call in a data source

```text
   conjure() + 5  ------->  result  (0)

   (n)   evaluation-priority badge shown by the IDE
```

## `sink.cfc` — call in a data sink

```text
   localA  ------->  drain()  (0)
```

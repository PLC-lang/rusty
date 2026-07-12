# unconnected_output_function

A block (here a **function**) with a `VAR_OUTPUT` pin that feeds nothing. An unconnected
output is still emitted as a named argument with an **empty right-hand side** (`extra => `)
rather than being routed through a temp no one reads. This mirrors the unconnected-input
case, on the output side.

```text
                  +------ myFunc ------+ (1)
   localA ------->| a           myFunc |------>  localResult  (2)
                  |              extra |   (unconnected)
                  +--------------------+

   extra    an output pin with no outgoing wire
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myFunc.st` — the function it calls; `a` is an input, the result and `extra` are outputs.

The result pin (`myFunc`) is wired to `localResult` and goes through a temp as usual, while
the unconnected `extra` output is emitted empty, so the network means:

```text
__myFunc_res_0 := myFunc(a := localA, extra => );
localResult := __myFunc_res_0;
```

The empty `extra => ` simply discards that output downstream. The transpiler lowers it
faithfully without minting a temp that nothing consumes.

# multi_output

Integration test: a **block with multiple distinct named outputs**, each wired to its own
sink. The CFC program `Split` calls the `Splitter` FB instance, routing its two outputs
(`lo`, `hi`) to two separate variables. Proves multi-output `=>` binding and per-pin temp
creation run correctly — every other lit block wires only a single output.

```text
                  +------- Splitter (inst) -------+ (0)
   n  ----------->| n                          lo |--->  outLo  (1)
                  |                            hi |--->  outHi  (2)
                  +-------------------------------+

   (0),(1),(2)  evaluation-priority badges shown by the IDE
```

- `splitter.st` — `FUNCTION_BLOCK Splitter` (input `n`, outputs `lo := n`, `hi := n + 100`).
- `split.cfc` — `PROGRAM Split` calling `inst : Splitter` with each output wired to a distinct sink.
- `main.st` — entry point: runs `Split` with `n := 7`, prints both outputs.

Lowers to:

```text
inst(n := n, lo => __Splitter_res_0, hi => __Splitter_res_1);
outLo := __Splitter_res_0;
outHi := __Splitter_res_1;
```

Both outputs land correct values (`lo = 7`, `hi = 107`), each through its own temp.

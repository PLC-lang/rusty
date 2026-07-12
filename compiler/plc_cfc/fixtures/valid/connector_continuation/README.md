# connector_continuation

A **connector / continuation pair** — a *named virtual wire*. Rather than drawing one
long line from a producer to a far-away consumer, the diagram drops two labelled stubs
that the tooling matches by `label`:

- a **`Connector`** (the input/definition end) takes a wire *in* and names it,
- a **`Continuation`** (the output/use end) of the same label re-emits that wire *out*.

The pair is exactly equivalent to a direct wire from the producer to every reader of the
continuation; it carries no semantics of its own and resolves away during lowering.

```text
   +-- alwaysFive --+ (0)
   |      alwaysFive|--id 12-->[ Connector "five" ]
   +----------------+

                      [ Continuation "five" ]--id 7-->  result  (1)

   "five"    the label matching the connector to the continuation
   (0),(1)   evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test. `alwaysFive`'s result feeds the connector
  `five`; the continuation `five` re-emits it to the `result` sink.
- `alwaysFive.st` — the nullary function whose result is relayed.

Resolving through the pair (`result` reads wire 7 → continuation `five` → connector `five`
→ wire 12 → `alwaysFive`'s result), the network lowers as if `result` were wired straight
to the block:

```text
__alwaysFive_res_0 := alwaysFive();
result := __alwaysFive_res_0;
```

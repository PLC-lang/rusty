# connector_chain

Integration test: a **transitive connector/continuation chain**. The value hops through
two named virtual wires (`x` then `y`) before reaching the sink. The sink must resolve
*transitively* back to the original producer. The `connector` test is a single hop, and
`motor_control` fans one connector to two consumers — neither chains hops, which is what
the `resolve_alias` transitive loop handles.

```text
   src  ----------->  > x          Connector "x" names the src wire

         x >  ----------->  > y    Continuation "x" feeds Connector "y"

                       y >  --------------->  out   (0)   Continuation "y" feeds the sink
```

- `chain.cfc` — `PROGRAM Chain` (`src`, `out`) wiring `src -> x -> y -> out`.
- `main.st` — entry point: sets `Chain.src := 99`, runs it, prints `Chain.out`.

Each continuation aliases back to the wire its like-labelled connector consumes, so the
sink resolves `y -> x -> src` in one transitive walk: `out := src`. Observing `99` proves
the multi-hop alias resolves at runtime.

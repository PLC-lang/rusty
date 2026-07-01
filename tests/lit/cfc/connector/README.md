# connector

Integration test: a **connector/continuation pair** — a *named virtual wire*. Instead of
drawing one long line from `src` to `out`, the diagram cuts it with a `Connector` labelled
`x` (the sink end) and reconnects it elsewhere with a `Continuation` of the same label
`x` (the source end). The value flows through unchanged.

```text
   src  ----------->  > x        Connector "x": names the wire feeding it

         x >  ----------------->  out   (0)   Continuation "x": re-emits that wire
```

- `wire.cfc` — `PROGRAM Wire` (`src : DINT`, `out : DINT`).
- `main.st` — entry point: sets `Wire.src := 42`, runs it, prints `Wire.out`.

During lowering the continuation's output is aliased back to the wire the connector
consumes, so the sink resolves straight through to the real producer — `out := src`.
Observing `42` proves the alias resolves at runtime.

# connector_continuation_cycle

A **cyclic** connector/continuation chain — a malformed diagram with no real producer:
connector `x` is fed by continuation `y`, and connector `y` is fed by continuation `x`, so
following the labels loops forever (`x → y → x → …`). The `result` sink reads one end,
which forces the resolver to walk the chain.

```text
   [Cont y]--(11)-->[Conn x]   [Cont x]--(10)-->[Conn y]   [Cont x]--(10)-->  result  (0)
        ^                                            |
        +--------------------------------------------+   (y feeds x feeds y ...)

   x,y   labels; the two pairs reference each other's output
   (0)   evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test. The two pairs reference each other, and the
  sink consumes continuation `x`'s output (wire 10) to trigger resolution.

**TODO:** resolving such a cycle currently *panics* (`Resolver::resolve_alias`). Once this
crate has an error-reporting story, this should instead surface a proper diagnostic
(cf. `E085`, "Sink is connected to itself"). The test below pins the panic so
the behaviour is visible and the upgrade path is obvious.

# connector_continuation_cycle

A **cyclic** connector/continuation chain — a malformed diagram with no real producer:
connector `x` is fed by continuation `y`, and connector `y` is fed by continuation `x`, so
following the labels loops forever (`x → y → x → …`) and never reaches a value. The `result`
sink reads one end of the loop, so it consumes a wire that nothing ultimately produces.

```text
   [Cont y]--(11)-->[Conn x]   [Cont x]--(10)-->[Conn y]   [Cont x]--(10)-->  result  (0)
        ^                                            |
        +--------------------------------------------+   (y feeds x feeds y ...)

   x,y   labels; the two pairs reference each other's output
   (0)   evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test. The two pairs reference each other, and the
  sink consumes continuation `x`'s output (wire 10) to trigger resolution.

Because the sink's wire resolves to no producer, this is reported as `E081` (a connection that
references no producer) and compilation aborts before transpilation — see the
`connector_continuation_cycle_is_reported` test in `validator.rs`. Note the resolver's
`resolve_alias` still terminates safely on the cycle rather than looping (it does not panic);
the `connector_continuation_cycle` test in `resolver.rs` pins that behaviour.

# connector_continuation_chain

A **chain of four connector/continuation pairs**. Each continuation feeds the next
connector, so the named virtual wires are linked end to end. The resolver follows the
chain **transitively** back to the one real producer.

```text
   +-- alwaysFive --+ (0)
   |      alwaysFive|--(10)-->[Conn a]   [Cont a]--(11)-->[Conn b]   [Cont b]--(12)-->[Conn c]
   +----------------+                                                            |
        [Cont c]--(13)-->[Conn d]   [Cont d]--(14)-->  result  (1)  <------------+

   a,b,c,d   labels matching each connector to its continuation
   (0),(1)   evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test. `alwaysFive`'s result enters connector `a`;
  it is relayed `a → b → c → d` and the last continuation feeds the `result` sink.
- `alwaysFive.st` — the nullary function whose result is relayed.

Resolving wire 14 walks every hop (`14 → 13 → 12 → 11 → 10`) to `alwaysFive`'s result, so
the whole chain collapses to a direct wire:

```text
temp_0 := alwaysFive();
result := temp_0;
```

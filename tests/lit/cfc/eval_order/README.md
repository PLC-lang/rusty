# eval_order

Integration test: **independent call blocks run in `EvaluationPriority` order, not document
order**. Two side-effecting blocks mutate a shared global `acc`; their order is observable
because the operations don't commute. The block that appears *first* in the XML has the
*higher* priority number, so document order and priority order disagree.

```text
   +--- DoubleIt ---+  (1)     acc := acc * 2     <- 1st in document, runs 2nd
   +----------------+

   +---  AddOne   ---+ (0)     acc := acc + 1     <- 2nd in document, runs 1st
   +-----------------+

   (0),(1)  evaluation-priority badges shown by the IDE
```

- `ops.st` — `VAR_GLOBAL acc` plus `DoubleIt` (`acc := acc * 2`) and `AddOne` (`acc := acc + 1`).
- `order.cfc` — `PROGRAM Order` with `DoubleIt` (priority 1) before `AddOne` (priority 0) in document order.
- `main.st` — entry point: seeds `acc := 5`, runs `Order`, prints `acc`.

Sorted by priority the body is `AddOne(); DoubleIt();`, so starting from `acc = 5`:

```text
acc := acc + 1;   (* 6 *)
acc := acc * 2;   (* 12 *)
```

`acc = 12` proves priority order ran; document order would have given `(5*2)+1 = 11`.

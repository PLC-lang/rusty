# conditional_return

A **conditional, negated `Return`**. A `Return` object has a single optional input wire
carrying a boolean condition: wired, the POU returns early when the condition holds;
unwired, it is an unconditional `RETURN`. Negation (carried by the `negated` vendor
extension in the object's `AddData`) inverts the condition, so the POU returns when the
wire is *false*.

```text
   enable  --o--->| RETURN |  (0)

   input   ------>  result    (1)

   --o-->   a negated condition wire (returns when enable is FALSE)
   (0),(1)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test. `enable` feeds the negated return; the
  assignment `result := input` follows it.

The return has the lower priority and is emitted first. Because it is negated, its
condition is wrapped in `NOT`, and the later assignment runs only when the early return
did not fire:

```text
IF NOT enable THEN RETURN; END_IF;
result := input;
```

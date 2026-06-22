# unconditional_return

An **unconditional `Return`**. A `Return` object with no input wire (no
`ConnectionPointIn`) always fires, lowering to a bare `RETURN;`. Contrast with
`conditional_return`, where a wired condition guards the return.

```text
   input  ------>  result    (0)

                  | RETURN |  (1)

   (no wire into RETURN -> unconditional)
   (0),(1)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test. The assignment `result := input` runs
  first, then the unconditional return.

The return has no condition wire, so it lowers to a plain `RETURN;` emitted after the
assignment (its higher priority orders it last):

```text
result := input;
RETURN;
```
